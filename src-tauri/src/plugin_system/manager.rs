use crate::constants::*;
use crate::errors::AppError;
use crate::plugin_system::rate_limiter::RateLimiter;
use crate::plugin_system::runtime::PluginRuntime;
use crate::plugin_system::types::{Plugin, PluginType};
use dashmap::DashMap;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PluginManager {
    indexer_plugins: DashMap<String, Arc<Plugin>>,
    resolver_plugins: DashMap<String, Arc<Plugin>>,
    utility_plugins: DashMap<String, Arc<Plugin>>,
    plugins_dir: PathBuf,
    runtime: Arc<PluginRuntime>,
    rate_limiter: RateLimiter,
    method_lookup: DashMap<String, HashMap<String, String>>,
    loading_locks: Arc<DashMap<String, Arc<Mutex<()>>>>,
    wasm_cache: Arc<DashMap<String, Arc<Vec<u8>>>>,
    execution_locks: Arc<DashMap<String, Arc<Mutex<()>>>>,
}

impl PluginManager {
    pub fn new(plugins_dir: PathBuf) -> Self {
        let runtime = PluginRuntime::new()
            .expect("Failed to create WASM runtime")
            .with_timeout(DEFAULT_PLUGIN_TIMEOUT_MS)
            .with_memory_limit(Some(DEFAULT_MAX_MEMORY_PAGES));

        let rate_limiter = RateLimiter::new().with_window_seconds(RATE_LIMIT_WINDOW_SECONDS);

        Self {
            indexer_plugins: DashMap::new(),
            resolver_plugins: DashMap::new(),
            utility_plugins: DashMap::new(),
            plugins_dir,
            runtime: Arc::new(runtime),
            rate_limiter,
            method_lookup: DashMap::new(),
            loading_locks: Arc::new(DashMap::new()),
            wasm_cache: Arc::new(DashMap::new()),
            execution_locks: Arc::new(DashMap::new()),
        }
    }

    fn cache_wasm_bytes(&self, plugin_id: &str, plugin: &Plugin) -> Result<Arc<Vec<u8>>, AppError> {
        if let Some(cached) = self.wasm_cache.get(plugin_id) {
            return Ok(cached.value().clone());
        }

        let wasm_path = self.plugins_dir.join(plugin_id).join(&plugin.filename);
        let wasm_bytes = fs::read(&wasm_path).map_err(|e| {
            AppError::Io(std::io::Error::new(
                e.kind(),
                format!("Failed to read WASM file at {}: {}", wasm_path.display(), e),
            ))
        })?;

        let wasm_arc = Arc::new(wasm_bytes);
        self.wasm_cache
            .insert(plugin_id.to_string(), wasm_arc.clone());

        Ok(wasm_arc)
    }

    pub async fn register_plugin(&self, plugin: Plugin) -> Result<(), AppError> {
        let plugin_id_str = plugin.id.clone();

        let load_lock = self
            .loading_locks
            .entry(plugin_id_str.clone())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone();
        let _guard = load_lock.lock().await;

        if self.indexer_plugins.contains_key(&plugin_id_str)
            || self.resolver_plugins.contains_key(&plugin_id_str)
            || self.utility_plugins.contains_key(&plugin_id_str)
        {
            self.indexer_plugins.remove(&plugin_id_str);
            self.resolver_plugins.remove(&plugin_id_str);
            self.utility_plugins.remove(&plugin_id_str);
            self.method_lookup.remove(&plugin_id_str);
            self.rate_limiter.remove_plugin(&plugin_id_str);
            self.wasm_cache.remove(&plugin_id_str);
            self.runtime.unload_plugin(&plugin_id_str);
        }

        let primary_type = if plugin.types.contains(&PluginType::Indexer) {
            PluginType::Indexer
        } else if plugin.types.contains(&PluginType::Resolver) {
            PluginType::Resolver
        } else if plugin.types.contains(&PluginType::Utility) {
            PluginType::Utility
        } else {
            return Err(AppError::Validation(format!(
                "Plugin '{}' has no valid types",
                plugin_id_str
            )));
        };

        let rate_limit = if let Some(ref plugin_rate_limit) = plugin.rate_limit {
            plugin_rate_limit.max_calls
        } else {
            match primary_type {
                PluginType::Indexer => INDEXER_RATE_LIMIT,
                PluginType::Resolver => RESOLVER_RATE_LIMIT,
                PluginType::Utility => UTILITY_RATE_LIMIT,
            }
        };
        self.rate_limiter.set_limit(&plugin_id_str, rate_limit);

        if let Some(ref plugin_rate_limit) = plugin.rate_limit {
            self.rate_limiter
                .set_window(&plugin_id_str, plugin_rate_limit.window_seconds);
        }

        let timeout_ms = match primary_type {
            PluginType::Indexer => INDEXER_PLUGIN_TIMEOUT_MS,
            PluginType::Resolver => RESOLVER_PLUGIN_TIMEOUT_MS,
            PluginType::Utility => UTILITY_PLUGIN_TIMEOUT_MS,
        };

        let memory_pages = match primary_type {
            PluginType::Indexer => HIGH_MAX_MEMORY_PAGES,
            PluginType::Resolver => DEFAULT_MAX_MEMORY_PAGES,
            PluginType::Utility => DEFAULT_MAX_MEMORY_PAGES,
        };

        let mut plugin_methods = HashMap::new();
        for method in &plugin.methods {
            plugin_methods.insert(
                method.interface_method.clone(),
                method.plugin_method.clone(),
            );
        }
        self.method_lookup
            .insert(plugin_id_str.clone(), plugin_methods);

        let is_indexer = plugin.types.contains(&PluginType::Indexer);
        let is_resolver = plugin.types.contains(&PluginType::Resolver);
        let is_utility = plugin.types.contains(&PluginType::Utility);

        let plugin_arc = Arc::new(plugin);

        let wasm_bytes = self.cache_wasm_bytes(&plugin_id_str, &plugin_arc)?;

        if is_indexer {
            self.indexer_plugins
                .insert(plugin_id_str.clone(), Arc::clone(&plugin_arc));
        }
        if is_resolver {
            self.resolver_plugins
                .insert(plugin_id_str.clone(), Arc::clone(&plugin_arc));
        }
        if is_utility {
            self.utility_plugins
                .insert(plugin_id_str.clone(), Arc::clone(&plugin_arc));
        }

        self.runtime.set_plugin_timeout(&plugin_id_str, timeout_ms);
        self.runtime
            .set_plugin_memory_limit(&plugin_id_str, memory_pages);

        if !self.runtime.plugins.contains_key(&plugin_id_str) {
            self.runtime
                .load_plugin(
                    plugin_id_str.clone(),
                    &wasm_bytes,
                    &plugin_arc.permissions.validated_hosts,
                )
                .map_err(|e| AppError::Runtime(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn ensure_plugin_loaded(&self, plugin_id: &str) -> Result<(), AppError> {
        if self.runtime.plugins.contains_key(plugin_id) {
            return Ok(());
        }

        let load_lock = self
            .loading_locks
            .entry(plugin_id.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone();

        let _guard = load_lock.lock().await;

        if self.runtime.plugins.contains_key(plugin_id) {
            return Ok(());
        }

        let plugin_arc = self
            .indexer_plugins
            .get(plugin_id)
            .or_else(|| self.resolver_plugins.get(plugin_id))
            .or_else(|| self.utility_plugins.get(plugin_id))
            .ok_or_else(|| AppError::NotFound(format!("Plugin not found: {}", plugin_id)))?
            .clone();

        let wasm_bytes = self
            .wasm_cache
            .get(plugin_id)
            .ok_or_else(|| {
                AppError::Runtime(format!(
                    "WASM cache miss for plugin '{}'. This shouldn't happen.",
                    plugin_id
                ))
            })?
            .value()
            .clone();

        self.runtime
            .load_plugin(
                plugin_id.to_string(),
                &wasm_bytes,
                &plugin_arc.permissions.validated_hosts,
            )
            .map_err(|e| AppError::Runtime(e.to_string()))?;

        Ok(())
    }

    pub async fn call_plugin_method(
        &self,
        plugin_name: &str,
        interface_method: &str,
        args: Vec<Value>,
    ) -> Result<Value, AppError> {
        self.rate_limiter
            .check_limit(plugin_name)
            .map_err(|e| AppError::RateLimit(format!("Plugin '{}': {}", plugin_name, e)))?;

        let plugin_method = self.get_plugin_method(plugin_name, interface_method)?;

        let exec_lock = self
            .execution_locks
            .entry(plugin_name.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone();

        let _guard = exec_lock.lock().await;

        self.ensure_plugin_loaded(plugin_name).await?;

        let runtime: Arc<PluginRuntime> = Arc::clone(&self.runtime);
        let plugin_name_owned: String = plugin_name.to_string();
        let plugin_method_owned: String = plugin_method.clone();

        let result: Result<Value, AppError> = tokio::task::spawn_blocking(move || {
            runtime
                .execute_plugin_method(&plugin_name_owned, &plugin_method_owned, args)
                .map_err(|e| AppError::Runtime(e.to_string()))
        })
        .await
        .map_err(|e| AppError::Runtime(format!("Plugin execution task failed: {}", e)))?;

        match result {
            Ok(value) => Ok(value),
            Err(e) => {
                let err_str = e.to_string();

                if err_str.contains("panicked") || err_str.contains("has been unloaded") {
                    self.execution_locks.remove(plugin_name);
                    self.loading_locks.remove(plugin_name);

                    Err(AppError::PluginCrashed {
                        plugin_id: plugin_name.to_string(),
                        details: format!("Plugin crashed: {}", err_str),
                    })
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn unregister_plugin(&self, plugin_id: &str) {
        self.unregister_plugin_internal(plugin_id);
    }

    fn unregister_plugin_internal(&self, plugin_id: &str) {
        self.indexer_plugins.remove(plugin_id);
        self.resolver_plugins.remove(plugin_id);
        self.utility_plugins.remove(plugin_id);
        self.method_lookup.remove(plugin_id);
        self.rate_limiter.remove_plugin(plugin_id);
        self.loading_locks.remove(plugin_id);
        self.wasm_cache.remove(plugin_id);
        self.runtime.unload_plugin(plugin_id);
    }

    pub fn unload_plugin_from_runtime(&self, plugin_id: &str) {
        self.runtime.unload_plugin(plugin_id);
    }

    pub async fn refresh_plugin(&self, plugin_id: &str) -> Result<(), AppError> {
        self.unregister_plugin(plugin_id);

        let plugin_dir = self.plugins_dir.join(plugin_id);
        if !plugin_dir.exists() {
            return Err(AppError::NotFound(format!(
                "Plugin directory not found: {}",
                plugin_id
            )));
        }

        let plugin = crate::plugin_system::loader::load_plugin_from_dir(plugin_dir)?;

        self.register_plugin(plugin).await?;

        self.ensure_plugin_loaded(plugin_id).await?;

        Ok(())
    }

    fn get_plugin_method(
        &self,
        plugin_name: &str,
        interface_method: &str,
    ) -> Result<String, AppError> {
        self.method_lookup
            .get(plugin_name)
            .and_then(|methods| methods.get(interface_method).cloned())
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Method '{}' not found in plugin '{}'",
                    interface_method, plugin_name
                ))
            })
    }
}

impl Clone for PluginManager {
    fn clone(&self) -> Self {
        Self {
            indexer_plugins: self.indexer_plugins.clone(),
            resolver_plugins: self.resolver_plugins.clone(),
            utility_plugins: self.utility_plugins.clone(),
            plugins_dir: self.plugins_dir.clone(),
            runtime: Arc::clone(&self.runtime),
            rate_limiter: self.rate_limiter.clone(),
            method_lookup: self.method_lookup.clone(),
            loading_locks: Arc::clone(&self.loading_locks),
            wasm_cache: Arc::clone(&self.wasm_cache),
            execution_locks: Arc::clone(&self.execution_locks),
        }
    }
}
