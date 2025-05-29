use crate::models::{ServerStatus, StartServerRequest};
use crate::{Error, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::{Child, Command};

pub struct PhpServerManager {
    servers: HashMap<String, ServerInstance>,
}

struct ServerInstance {
    #[allow(dead_code)]
    id: String,
    child: Child,
    port: u16,
    host: String,
    document_root: String,
    started_at: String,
    #[allow(dead_code)]
    php_version: String,
}

impl Default for PhpServerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PhpServerManager {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }

    pub async fn start_server(
        &mut self,
        request: StartServerRequest,
        php_executable_path: PathBuf,
    ) -> Result<String> {
        let server_id = uuid::Uuid::new_v4().to_string();
        let port = request.port.unwrap_or(8000);
        let host = request.host.unwrap_or_else(|| "127.0.0.1".to_string());
        let document_root = request
            .document_root
            .unwrap_or(request.project_path.clone());

        // Check if port is available
        if self.is_port_in_use(port) {
            return Err(Error::PhpServer(format!("Port {} is already in use", port)));
        }

        // Validate document root exists
        let doc_root_path = PathBuf::from(&document_root);
        if !doc_root_path.exists() {
            return Err(Error::PhpServer(format!(
                "Document root does not exist: {}",
                document_root
            )));
        }

        // Build PHP server command
        let mut cmd = Command::new(&php_executable_path);
        cmd.arg("-S")
            .arg(format!("{}:{}", host, port))
            .arg("-t")
            .arg(&document_root)
            .current_dir(&request.project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Add router script if it exists (for frameworks like Laravel)
        let router_script = self.detect_router_script(&doc_root_path)?;
        if let Some(router) = router_script {
            cmd.arg(router);
        }

        // Start the server process
        let child = cmd
            .spawn()
            .map_err(|e| Error::PhpServer(format!("Failed to start PHP server: {}", e)))?;

        let server_instance = ServerInstance {
            id: server_id.clone(),
            child,
            port,
            host: host.clone(),
            document_root: document_root.clone(),
            started_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            php_version: request.php_version.unwrap_or_else(|| "unknown".to_string()),
        };

        self.servers.insert(server_id.clone(), server_instance);

        // Wait a moment to ensure the server started successfully
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Check if the server is actually running
        if !self.is_server_running(&server_id) {
            self.servers.remove(&server_id);
            return Err(Error::PhpServer("Failed to start PHP server".to_string()));
        }

        log::info!(
            "PHP server started on {}:{} with document root: {}",
            host,
            port,
            document_root
        );
        Ok(server_id)
    }

    pub async fn stop_server(&mut self, server_id: &str) -> Result<()> {
        if let Some(mut server) = self.servers.remove(server_id) {
            // Try to kill the process gracefully
            if let Err(e) = server.child.kill().await {
                log::warn!("Failed to kill PHP server process: {}", e);
            }

            // Wait for the process to exit
            if let Err(e) = server.child.wait().await {
                log::warn!("Failed to wait for PHP server process: {}", e);
            }

            log::info!("PHP server {} stopped", server_id);
            Ok(())
        } else {
            Err(Error::PhpServer(format!("Server {} not found", server_id)))
        }
    }

    pub fn stop_all_servers(&mut self) -> Result<()> {
        let server_ids: Vec<String> = self.servers.keys().cloned().collect();

        for server_id in server_ids {
            if let Some(mut server) = self.servers.remove(&server_id) {
                // Kill the process (non-async version)
                let _ = server.child.start_kill();
            }
        }

        self.servers.clear();
        Ok(())
    }

    pub fn get_server_status(&self, server_id: &str) -> Result<ServerStatus> {
        if let Some(server) = self.servers.get(server_id) {
            // 文件内容无需更改，建议清理 target 目录后重新编译。
            // 直接在此处判断进程是否存活，避免可变借用
            let is_running = if let Some(pid) = server.child.id() {
                self.is_process_running(pid)
            } else {
                false
            };
            Ok(ServerStatus {
                is_running,
                pid: server.child.id(),
                port: Some(server.port),
                host: Some(server.host.clone()),
                document_root: Some(server.document_root.clone()),
                started_at: Some(server.started_at.clone()),
            })
        } else {
            Ok(ServerStatus {
                is_running: false,
                pid: None,
                port: None,
                host: None,
                document_root: None,
                started_at: None,
            })
        }
    }

    pub fn list_running_servers(&self) -> Vec<(String, ServerStatus)> {
        let mut result = Vec::new();
        for (id, server) in &self.servers {
            // 直接在此处判断进程是否存活，避免可变借用
            let is_running = if let Some(pid) = server.child.id() {
                self.is_process_running(pid)
            } else {
                false
            };
            let status = ServerStatus {
                is_running,
                pid: server.child.id(),
                port: Some(server.port),
                host: Some(server.host.clone()),
                document_root: Some(server.document_root.clone()),
                started_at: Some(server.started_at.clone()),
            };
            result.push((id.clone(), status));
        }
        result
    }

    fn is_server_running(&mut self, server_id: &str) -> bool {
        if let Some(server) = self.servers.get_mut(server_id) {
            // Check if the process is still alive using PID
            if let Some(pid) = server.child.id() {
                self.is_process_running(pid)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_process_running(&self, pid: u32) -> bool {
        // On Unix systems, we can check if a process is running by sending signal 0
        #[cfg(unix)]
        {
            use std::process::Command;
            Command::new("kill")
                .arg("-0")
                .arg(pid.to_string())
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }

        // On Windows, we can use tasklist
        #[cfg(windows)]
        {
            use std::process::Command;
            Command::new("tasklist")
                .arg("/FI")
                .arg(format!("PID eq {}", pid))
                .output()
                .map(|output| {
                    output.status.success()
                        && String::from_utf8_lossy(&output.stdout).contains(&pid.to_string())
                })
                .unwrap_or(false)
        }
    }

    fn detect_router_script(&self, document_root: &Path) -> Result<Option<String>> {
        // Laravel router script
        let laravel_router = document_root.join("server.php");
        if laravel_router.exists() {
            return Ok(Some("server.php".to_string()));
        }

        // Other frameworks might have their own router scripts
        // Add more detection logic here as needed

        Ok(None)
    }

    fn is_port_in_use(&self, port: u16) -> bool {
        // Check if any of our managed servers are using this port
        // This is a basic check, a more robust solution would involve trying to bind to the port
        for server in self.servers.values() {
            if server.port == port {
                return true;
            }
        }

        // Attempt to bind to the port to check if it's truly available
        // This is a more reliable way to check port availability
        std::net::TcpListener::bind(("127.0.0.1", port)).is_err()
    }

    pub async fn get_server_logs(&self, server_id: &str) -> Result<String> {
        if let Some(server) = self.servers.get(server_id) {
            // In a real implementation, you would capture and store server logs
            // For now, return a placeholder
            Ok(format!(
                "Logs for server {} (PID: {:?})",
                server_id,
                server.child.id()
            ))
        } else {
            Err(Error::PhpServer(format!("Server {} not found", server_id)))
        }
    }

    pub fn find_available_port(&self, start_port: u16) -> u16 {
        let mut port = start_port;
        let max_attempts = 65535 - start_port + 1;
        let mut attempts = 0;

        while self.is_port_in_use(port) || self.is_port_system_in_use(port) {
            attempts += 1;
            if attempts >= max_attempts {
                // If we've tried all ports from start_port to 65535, panic
                panic!("No available port found in range {}..65535", start_port);
            }

            port = port.saturating_add(1);
            if port == 65535 && attempts < max_attempts {
                // If we reach the maximum port, wrap around to a reasonable starting point
                port = 8000;
            }
        }
        port
    }

    fn is_port_system_in_use(&self, port: u16) -> bool {
        // Try to bind to the port to check if it's available
        std::net::TcpListener::bind(("127.0.0.1", port)).is_err()
    }
}

impl Drop for PhpServerManager {
    fn drop(&mut self) {
        // Clean up all servers when the manager is dropped
        let _ = self.stop_all_servers();
    }
}
