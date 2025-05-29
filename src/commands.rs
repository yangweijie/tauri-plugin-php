use crate::{models::*, Error, PhpManager, Result};
use tauri::{command, State};

#[command]
pub async fn download_php_binary(
    state: State<'_, PhpManager>,
    version: String,
) -> Result<PhpBinaryInfo> {
    let binary_manager = state.binary_manager.lock().await;
    binary_manager.download_php_binary(&version).await
}

#[command]
pub async fn start_php_server(
    state: State<'_, PhpManager>,
    request: StartServerRequest,
) -> Result<String> {
    let binary_manager = state.binary_manager.lock().await;
    let mut server_manager = state.server_manager.lock().await;

    // Get PHP version to use
    let php_version = request
        .php_version
        .clone()
        .unwrap_or_else(|| "8.3.0".to_string());

    // Get PHP executable path
    let php_executable = binary_manager.get_php_executable_path(&php_version);

    if !php_executable.exists() {
        return Err(Error::PhpBinaryNotFound);
    }

    // Start the server
    server_manager.start_server(request, php_executable).await
}

#[command]
pub async fn stop_php_server(state: State<'_, PhpManager>, server_id: String) -> Result<()> {
    let mut server_manager = state.server_manager.lock().await;
    server_manager.stop_server(&server_id).await
}

#[command]
pub async fn get_server_status(
    state: State<'_, PhpManager>,
    server_id: String,
) -> Result<ServerStatus> {
    let server_manager = state.server_manager.lock().await;
    server_manager.get_server_status(&server_id)
}

#[command]
pub async fn clone_project(
    state: State<'_, PhpManager>,
    request: CloneProjectRequest,
) -> Result<ProjectInfo> {
    let project_manager = state.project_manager.lock().await;
    project_manager.clone_project(request).await
}

#[command]
pub async fn detect_framework(
    state: State<'_, PhpManager>,
    project_path: String,
) -> Result<Framework> {
    let project_manager = state.project_manager.lock().await;
    project_manager.detect_framework(&std::path::PathBuf::from(project_path))
}

#[command]
pub async fn get_php_version(state: State<'_, PhpManager>, version: String) -> Result<String> {
    let binary_manager = state.binary_manager.lock().await;
    binary_manager.get_php_version(&version).await
}

#[command]
pub async fn list_available_php_versions(
    state: State<'_, PhpManager>,
) -> Result<Vec<PhpBinaryInfo>> {
    let binary_manager = state.binary_manager.lock().await;
    binary_manager.list_installed_versions()
}

#[command]
pub async fn set_php_config(_state: State<'_, PhpManager>, config: Config) -> Result<()> {
    // Save configuration to file
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("tauri-php-plugin");

    std::fs::create_dir_all(&config_dir)?;

    let config_file = config_dir.join("config.json");
    let config_json = serde_json::to_string_pretty(&config)?;
    std::fs::write(config_file, config_json)?;

    Ok(())
}

#[command]
pub async fn get_php_config(_state: State<'_, PhpManager>) -> Result<Config> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("tauri-php-plugin");

    let config_file = config_dir.join("config.json");

    if config_file.exists() {
        let config_content = std::fs::read_to_string(config_file)?;
        let config: Config = serde_json::from_str(&config_content)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

#[command]
pub async fn list_running_servers(
    state: State<'_, PhpManager>,
) -> Result<Vec<(String, ServerStatus)>> {
    let server_manager = state.server_manager.lock().await;
    Ok(server_manager.list_running_servers())
}

#[command]
pub async fn list_projects(state: State<'_, PhpManager>) -> Result<Vec<ProjectInfo>> {
    let project_manager = state.project_manager.lock().await;
    project_manager.list_projects()
}

#[command]
pub async fn remove_project(state: State<'_, PhpManager>, project_name: String) -> Result<()> {
    let project_manager = state.project_manager.lock().await;
    project_manager.remove_project(&project_name)
}

#[command]
pub async fn remove_php_binary(state: State<'_, PhpManager>, version: String) -> Result<()> {
    let binary_manager = state.binary_manager.lock().await;
    binary_manager.remove_php_binary(&version)
}

#[command]
pub async fn get_server_logs(state: State<'_, PhpManager>, server_id: String) -> Result<String> {
    let server_manager = state.server_manager.lock().await;
    server_manager.get_server_logs(&server_id).await
}

#[command]
pub async fn find_available_port(state: State<'_, PhpManager>, start_port: u16) -> Result<u16> {
    let server_manager = state.server_manager.lock().await;
    Ok(server_manager.find_available_port(start_port))
}

#[command]
pub async fn stop_all_servers(state: State<'_, PhpManager>) -> Result<()> {
    let mut server_manager = state.server_manager.lock().await;
    server_manager.stop_all_servers()
}

#[command]
pub async fn get_framework_info(
    state: State<'_, PhpManager>,
    framework: Framework,
) -> Result<crate::framework_detector::FrameworkInfo> {
    let project_manager = state.project_manager.lock().await;
    Ok(project_manager.get_framework_info(&framework))
}

#[command]
pub async fn validate_project_path(path: String) -> Result<bool> {
    let project_path = std::path::PathBuf::from(path);
    Ok(project_path.exists() && project_path.is_dir())
}

#[command]
pub async fn get_project_info(
    state: State<'_, PhpManager>,
    project_path: String,
) -> Result<ProjectInfo> {
    let project_manager = state.project_manager.lock().await;
    let path = std::path::PathBuf::from(&project_path);

    let framework = project_manager.detect_framework(&path).ok();
    let php_version = project_manager
        .detect_php_version_requirement_public(&path)
        .ok()
        .flatten();

    let project_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    Ok(ProjectInfo {
        name: project_name,
        path: project_path,
        framework,
        git_url: None,
        php_version,
        entry_point: None,
    })
}
