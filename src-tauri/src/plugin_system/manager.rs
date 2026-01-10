use crate::constants::{INDEXER_RATE_LIMIT, RATE_LIMIT_WINDOW_SECONDS, RESOLVER_RATE_LIMIT};
use crate::errors::AppError;
use crate::plugin_system::rate_limiter::RateLimiter;
use crate::plugin_system::runtime::PluginRuntime;
use crate::plugin_system::types::{Plugin, PluginType};
use dashmap::DashMap;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

pub struct PluginManager {
    indexer_plugins: HashMap<String, Arc<Plugin>>,
    resolver_plugins: HashMap<String, Arc<Plugin>>,
    plugins_dir: PathBuf,
    runtime: Arc<RwLock<PluginRuntime>>,
    rate_limiter: RateLimiter,
    method_lookup: HashMap<String, HashMap<String, String>>,
    loading_locks: Arc<DashMap<String, Arc<Mutex<()>>>>,
    wasm_cache: Arc<DashMap<String, Arc<Vec<u8>>>>,
}

impl PluginManager {
    pub fn new(plugins_dir: PathBuf) -> Self {
        let runtime = PluginRuntime::new()
            .expect("Failed to create WASM runtime")
            .with_timeout(crate::constants::DEFAULT_PLUGIN_TIMEOUT_MS);
        let rate_limiter = RateLimiter::new().with_window_seconds(RATE_LIMIT_WINDOW_SECONDS);

        Self {
            indexer_plugins: HashMap::new(),
            resolver_plugins: HashMap::new(),
            plugins_dir,
            runtime: Arc::new(RwLock::new(runtime)),
            rate_limiter,
            method_lookup: HashMap::new(),
            loading_locks: Arc::new(DashMap::new()),
            wasm_cache: Arc::new(DashMap::new()),
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

    pub fn register_plugin(&mut self, plugin: Plugin) -> Result<(), AppError> {
        let plugin_id_str = plugin.id.clone();

        let rate_limit = if plugin.types.contains(&PluginType::Indexer) {
            INDEXER_RATE_LIMIT
        } else {
            RESOLVER_RATE_LIMIT
        };

        self.rate_limiter.set_limit(&plugin_id_str, rate_limit);

        let timeout_ms = if plugin.types.contains(&PluginType::Indexer) {
            crate::constants::INDEXER_PLUGIN_TIMEOUT_MS
        } else if plugin.types.contains(&PluginType::Resolver) {
            crate::constants::RESOLVER_PLUGIN_TIMEOUT_MS
        } else {
            crate::constants::DEFAULT_PLUGIN_TIMEOUT_MS
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

        let plugin_arc = Arc::new(plugin);

        let wasm_bytes = self.cache_wasm_bytes(&plugin_id_str, &plugin_arc)?;

        match (is_indexer, is_resolver) {
            (true, true) => {
                self.indexer_plugins
                    .insert(plugin_id_str.clone(), Arc::clone(&plugin_arc));
                self.resolver_plugins
                    .insert(plugin_id_str.clone(), Arc::clone(&plugin_arc));
            }
            (true, false) => {
                self.indexer_plugins
                    .insert(plugin_id_str.clone(), Arc::clone(&plugin_arc));
            }
            (false, true) => {
                self.resolver_plugins
                    .insert(plugin_id_str.clone(), Arc::clone(&plugin_arc));
            }
            (false, false) => {
                eprintln!("Warning: Plugin '{}' has no valid types", plugin_id_str);
            }
        }

        let mut runtime_guard = self
            .runtime
            .write()
            .map_err(|_| AppError::Runtime("Runtime lock poisoned".into()))?;

        runtime_guard.set_plugin_timeout(&plugin_id_str, timeout_ms);

        if !runtime_guard.plugins.contains_key(&plugin_id_str) {
            runtime_guard
                .load_plugin(
                    plugin_id_str.clone(),
                    &wasm_bytes,
                    &plugin_arc.permissions.validated_hosts,
                )
                .map_err(|e| AppError::Runtime(e.to_string()))?;
        }

        Ok(())
    }

    fn ensure_plugin_loaded(&mut self, plugin_id: &str) -> Result<(), AppError> {
        {
            let runtime_guard = self
                .runtime
                .read()
                .map_err(|_| AppError::Runtime("Runtime lock poisoned".into()))?;

            if runtime_guard.plugins.contains_key(plugin_id) {
                return Ok(());
            }
        }

        let load_lock = self
            .loading_locks
            .entry(plugin_id.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone();

        let _guard = load_lock
            .lock()
            .map_err(|_| AppError::Runtime("Plugin loading lock poisoned".into()))?;

        {
            let runtime_guard = self
                .runtime
                .read()
                .map_err(|_| AppError::Runtime("Runtime lock poisoned".into()))?;

            if runtime_guard.plugins.contains_key(plugin_id) {
                return Ok(());
            }
        }

        let plugin_arc = self
            .indexer_plugins
            .get(plugin_id)
            .or_else(|| self.resolver_plugins.get(plugin_id))
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

        let mut runtime_guard = self
            .runtime
            .write()
            .map_err(|_| AppError::Runtime("Runtime lock poisoned".into()))?;

        if !runtime_guard.plugins.contains_key(plugin_id) {
            runtime_guard
                .load_plugin(
                    plugin_id.to_string(),
                    &wasm_bytes,
                    &plugin_arc.permissions.validated_hosts,
                )
                .map_err(|e| AppError::Runtime(e.to_string()))?;
        }

        Ok(())
    }

    pub fn call_plugin_method(
        &mut self,
        plugin_name: &str,
        interface_method: &str,
        args: Vec<Value>,
    ) -> Result<Value, AppError> {
        self.rate_limiter
            .check_limit(plugin_name)
            .map_err(|e| AppError::RateLimit(e.to_string()))?;

        let plugin_method = self
            .method_lookup
            .get(plugin_name)
            .and_then(|methods| methods.get(interface_method))
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Method '{}' not found in plugin '{}'",
                    interface_method, plugin_name
                ))
            })?
            .clone();

        self.ensure_plugin_loaded(plugin_name)?;

        let mut runtime_guard = self
            .runtime
            .write()
            .map_err(|_| AppError::Runtime("Runtime lock poisoned".into()))?;

        runtime_guard
            .execute_plugin_method(plugin_name, &plugin_method, args)
            .map_err(|e| AppError::Runtime(e.to_string()))
    }

    pub fn unregister_plugin(&mut self, plugin_id: &str) {
        self.indexer_plugins.remove(plugin_id);
        self.resolver_plugins.remove(plugin_id);

        self.method_lookup.remove(plugin_id);

        self.rate_limiter.remove_plugin(plugin_id);

        self.loading_locks.remove(plugin_id);

        let _ = self.unload_plugin(plugin_id);
    }

    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), AppError> {
        let mut runtime_guard = self
            .runtime
            .write()
            .map_err(|_| AppError::Runtime("Runtime lock poisoned".into()))?;

        runtime_guard.plugins.remove(plugin_id);

        drop(runtime_guard);

        self.wasm_cache.remove(plugin_id);

        Ok(())
    }
}
