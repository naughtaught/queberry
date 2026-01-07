mod commands;
pub mod constants;
mod plugin_system;
mod utils;

use commands::AppState;
use plugin_system::PluginManager;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let start = std::time::Instant::now();
            println!("Setup starting...");

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            println!("Logger initialized (took {:?})", start.elapsed());

            println!("Getting plugins dir...");
            let plugins_dir = utils::get_plugins_dir()?;
            println!(
                "Plugins dir: {:?} (took {:?})",
                plugins_dir,
                start.elapsed()
            );

            println!("Creating manager...");
            let mut manager = PluginManager::new(plugins_dir);
            println!("Manager created (took {:?})", start.elapsed());

            // Only register plugins (fast - just reads manifests)
            println!("Loading plugins...");
            if let Ok(plugins) = plugin_system::load_all_plugins() {
                println!("Found {} plugins", plugins.len());
                for plugin in plugins {
                    println!("Registering plugin: {}", plugin.id);
                    manager.register_plugin(plugin);
                }
            }
            println!("All plugins registered (took {:?})", start.elapsed());

            app.manage(AppState {
                plugin_manager: Mutex::new(manager),
            });

            println!("Setup complete (took {:?})", start.elapsed());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_plugins,
            commands::call_plugin_method
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
