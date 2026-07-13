use crate::db::Database;
use crate::errors::AppError;
use crate::plugin_system::loader::validate_and_extract_host;
use crate::plugin_system::types::{
    ApiKeyRequirement, MethodMapping, Plugin, PluginPermissions, PluginRateLimit, PluginType,
    SourceType,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

#[derive(Clone)]
pub struct PluginCacheManager {
    db: Arc<Database>,
}

#[derive(sqlx::FromRow)]
struct PluginCacheRow {
    name: String,
    filename: String,
    author: String,
    homepage: Option<String>,
    description: String,
    version: String,
    api_version: String,
    cacheless: bool,
    requires_api_key: String,
    sources: String,
    types: String,
    network_patterns: String,
    allow_private: bool,
    methods: String,
    manifest_hash: String,
    wasm_hash: String,
    rate_limit: Option<String>,
}

impl PluginCacheManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn load_if_fresh(
        &self,
        plugin_id: &str,
        manifest_content: &str,
        wasm_bytes: &[u8],
    ) -> Result<Option<Plugin>, AppError> {
        let manifest_hash = sha256_str(manifest_content);
        let wasm_hash = sha256_bytes(wasm_bytes);

        let row = sqlx::query_as::<_, PluginCacheRow>(
            "SELECT name, filename, author, homepage, description,
                    version, api_version, cacheless, requires_api_key,
                    sources, types, network_patterns, allow_private, methods,
                    manifest_hash, wasm_hash, rate_limit
             FROM plugin_cache WHERE id = ?",
        )
        .bind(plugin_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            None => Ok(None),
            Some(row) => {
                if row.manifest_hash != manifest_hash || row.wasm_hash != wasm_hash {
                    return Ok(None);
                }

                let requires_api_key: ApiKeyRequirement =
                    serde_json::from_str(&format!("\"{}\"", row.requires_api_key))
                        .unwrap_or(ApiKeyRequirement::Never);
                let sources: Vec<SourceType> =
                    serde_json::from_str(&row.sources).map_err(AppError::Json)?;
                let types: Vec<PluginType> =
                    serde_json::from_str(&row.types).map_err(AppError::Json)?;
                let network_patterns: Vec<String> =
                    serde_json::from_str(&row.network_patterns).map_err(AppError::Json)?;
                let methods: Vec<MethodMapping> =
                    serde_json::from_str(&row.methods).map_err(AppError::Json)?;
                let rate_limit: Option<PluginRateLimit> =
                    row.rate_limit.and_then(|j| serde_json::from_str(&j).ok());

                let mut validated_hosts = Vec::new();
                for pattern in &network_patterns {
                    validated_hosts.push(validate_and_extract_host(pattern, row.allow_private)?);
                }

                Ok(Some(Plugin {
                    id: plugin_id.to_string(),
                    name: row.name,
                    filename: row.filename,
                    author: row.author,
                    homepage: row.homepage,
                    description: row.description,
                    version: row.version,
                    api_version: row.api_version,
                    cacheless: row.cacheless,
                    requires_api_key,
                    sources,
                    types,
                    permissions: PluginPermissions {
                        validated_hosts,
                        network_patterns,
                        allow_private_networks: row.allow_private,
                    },
                    methods,
                    rate_limit,
                }))
            }
        }
    }

    pub async fn upsert(
        &self,
        plugin: &Plugin,
        manifest_content: &str,
        wasm_bytes: &[u8],
    ) -> Result<(), AppError> {
        let requires_api_key_str = serde_json::to_string(&plugin.requires_api_key)
            .unwrap()
            .trim_matches('"')
            .to_string();

        let rate_limit_json = plugin
            .rate_limit
            .as_ref()
            .map(|r| serde_json::to_string(r).unwrap());

        sqlx::query(
            "INSERT INTO plugin_cache (
                id, name, filename, author, homepage, description,
                version, api_version, cacheless, requires_api_key,
                sources, types, network_patterns, allow_private, methods,
                manifest_hash, wasm_hash, rate_limit, registered_at
            ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,unixepoch())
            ON CONFLICT(id) DO UPDATE SET
                name             = excluded.name,
                filename         = excluded.filename,
                author           = excluded.author,
                homepage         = excluded.homepage,
                description      = excluded.description,
                version          = excluded.version,
                api_version      = excluded.api_version,
                cacheless        = excluded.cacheless,
                requires_api_key = excluded.requires_api_key,
                sources          = excluded.sources,
                types            = excluded.types,
                network_patterns = excluded.network_patterns,
                allow_private    = excluded.allow_private,
                methods          = excluded.methods,
                manifest_hash    = excluded.manifest_hash,
                wasm_hash        = excluded.wasm_hash,
                rate_limit       = excluded.rate_limit,
                registered_at    = unixepoch()",
        )
        .bind(&plugin.id)
        .bind(&plugin.name)
        .bind(&plugin.filename)
        .bind(&plugin.author)
        .bind(&plugin.homepage)
        .bind(&plugin.description)
        .bind(&plugin.version)
        .bind(&plugin.api_version)
        .bind(plugin.cacheless)
        .bind(requires_api_key_str)
        .bind(serde_json::to_string(&plugin.sources).unwrap())
        .bind(serde_json::to_string(&plugin.types).unwrap())
        .bind(serde_json::to_string(&plugin.permissions.network_patterns).unwrap())
        .bind(plugin.permissions.allow_private_networks)
        .bind(serde_json::to_string(&plugin.methods).unwrap())
        .bind(sha256_str(manifest_content))
        .bind(sha256_bytes(wasm_bytes))
        .bind(rate_limit_json)
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }

    pub async fn remove_missing(&self, present_ids: &[String]) -> Result<(), AppError> {
        if present_ids.is_empty() {
            sqlx::query("DELETE FROM plugin_cache")
                .execute(&self.db.pool)
                .await?;
            return Ok(());
        }

        let placeholders = present_ids
            .iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "DELETE FROM plugin_cache WHERE id NOT IN ({})",
            placeholders
        );

        let mut query = sqlx::query(&sql);
        for id in present_ids {
            query = query.bind(id);
        }

        query.execute(&self.db.pool).await?;
        Ok(())
    }
}

fn sha256_str(s: &str) -> String {
    sha256_bytes(s.as_bytes())
}

fn sha256_bytes(b: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b);
    format!("{:x}", h.finalize())
}
