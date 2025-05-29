use std::sync::Arc;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use tokio::sync::Mutex;

pub use models::*;

mod commands;
mod error;
pub mod framework_detector;
pub mod models;
pub mod php_binary;
pub mod php_server;
pub mod project_manager;

pub use error::{Error, Result};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("php")
        .invoke_handler(tauri::generate_handler![
            commands::download_php_binary,
            commands::start_php_server,
            commands::stop_php_server,
            commands::get_server_status,
            commands::clone_project,
            commands::detect_framework,
            commands::get_php_version,
            commands::list_available_php_versions,
            commands::set_php_config,
            commands::get_php_config,
            commands::list_running_servers,
            commands::list_projects,
            commands::remove_project,
            commands::remove_php_binary,
            commands::get_server_logs,
            commands::find_available_port,
            commands::stop_all_servers,
            commands::get_framework_info,
            commands::validate_project_path,
            commands::get_project_info,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let php = mobile::init(app, api)?;
            #[cfg(desktop)]
            let php = desktop::init(app, api)?;
            app.manage(php);
            Ok(())
        })
        .build()
}

#[cfg(desktop)]
mod desktop {
    use super::*;

    pub fn init<R: Runtime>(
        _app: &tauri::AppHandle<R>,
        _api: tauri::plugin::PluginApi<R, super::models::Config>,
    ) -> crate::Result<PhpManager> {
        Ok(PhpManager::new())
    }
}

#[cfg(mobile)]
mod mobile {
    use super::*;

    pub fn init<R: Runtime>(
        _app: &tauri::AppHandle<R>,
        _api: tauri::plugin::PluginApi<R, super::models::Config>,
    ) -> crate::Result<PhpManager> {
        // Mobile implementation would be different
        Ok(PhpManager::new())
    }
}

pub struct PhpManager {
    pub binary_manager: Arc<Mutex<php_binary::PhpBinaryManager>>,
    pub server_manager: Arc<Mutex<php_server::PhpServerManager>>,
    pub project_manager: Arc<Mutex<project_manager::ProjectManager>>,
}

impl Default for PhpManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PhpManager {
    pub fn new() -> Self {
        Self {
            binary_manager: Arc::new(Mutex::new(php_binary::PhpBinaryManager::new())),
            server_manager: Arc::new(Mutex::new(php_server::PhpServerManager::new())),
            project_manager: Arc::new(Mutex::new(project_manager::ProjectManager::new())),
        }
    }
}
