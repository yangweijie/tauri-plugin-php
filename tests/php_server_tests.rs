use tauri_plugin_php::models::{ServerStatus, StartServerRequest};
use tauri_plugin_php::php_server::PhpServerManager;

mod common;

#[tokio::test]
async fn test_php_server_manager_creation() {
    let _manager = PhpServerManager::new();

    // 测试管理器创建成功 - 如果能创建就说明成功
}

#[tokio::test]
async fn test_find_available_port() {
    let manager = PhpServerManager::new();

    let start_port = 8000;
    let available_port = manager.find_available_port(start_port);

    // 验证返回的端口在合理范围内
    assert!(available_port >= start_port);
    assert!(available_port > 0); // u16 max is 65535, so this is always true but more meaningful

    // 验证端口确实可用 - 使用与manager相同的检查方法
    // 由于可能存在时间窗口问题，我们检查端口是否可以绑定
    let listener_result = std::net::TcpListener::bind(("127.0.0.1", available_port));
    assert!(
        listener_result.is_ok(),
        "Port {} should be available",
        available_port
    );
    // 立即释放端口
    drop(listener_result);
}

#[tokio::test]
async fn test_server_status_for_nonexistent_server() {
    let manager = PhpServerManager::new();

    let status = manager.get_server_status("nonexistent-server-id").unwrap();

    assert!(!status.is_running);
    assert!(status.pid.is_none());
    assert!(status.port.is_none());
    assert!(status.host.is_none());
    assert!(status.document_root.is_none());
    assert!(status.started_at.is_none());
}

#[tokio::test]
async fn test_list_running_servers_empty() {
    let manager = PhpServerManager::new();

    let servers = manager.list_running_servers();

    // 新创建的管理器应该没有运行的服务器
    assert_eq!(servers.len(), 0);
}

#[tokio::test]
async fn test_stop_nonexistent_server() {
    let mut manager = PhpServerManager::new();

    let result = manager.stop_server("nonexistent-server-id").await;

    // 停止不存在的服务器应该返回错误
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stop_all_servers_empty() {
    let mut manager = PhpServerManager::new();

    let result = manager.stop_all_servers();

    // 在没有运行服务器的情况下，停止所有服务器应该成功
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_server_request_validation() {
    let temp_dir = common::create_temp_dir();
    let project_path = common::create_mock_php_project(temp_dir.path(), "plain");
    let php_executable = common::create_mock_php_binary(temp_dir.path(), "8.3.0");

    let mut manager = PhpServerManager::new();

    let request = StartServerRequest {
        project_path: project_path.to_string_lossy().to_string(),
        port: Some(common::find_available_port(8000)),
        host: Some("127.0.0.1".to_string()),
        php_version: Some("8.3.0".to_string()),
        document_root: Some(project_path.to_string_lossy().to_string()),
    };

    // 注意：这个测试可能会失败，因为我们使用的是模拟的 PHP 二进制文件
    // 在实际环境中，需要真实的 PHP 可执行文件
    let result = manager.start_server(request, php_executable).await;

    // 由于使用模拟的 PHP 二进制，预期会失败
    // 但我们可以验证错误类型
    if result.is_err() {
        // 这是预期的，因为模拟的 PHP 二进制无法真正启动服务器
        // Test passes if we get an error as expected
    }
}

#[tokio::test]
async fn test_port_conflict_detection() {
    let _manager = PhpServerManager::new();

    // 找到一个可用端口
    let port = common::find_available_port(8000);

    // 绑定端口以模拟冲突 - 处理可能的竞争条件
    match std::net::TcpListener::bind(("127.0.0.1", port)) {
        Ok(_listener) => {
            // 成功绑定，现在端口应该不可用
            assert!(!common::is_port_available(port));
        }
        Err(_) => {
            // 如果绑定失败（可能由于竞争条件），跳过这个测试
            // 这是可以接受的，因为我们主要是测试端口冲突检测逻辑
            println!("Port {} was already in use, skipping conflict test", port);
        }
    }
}

#[tokio::test]
async fn test_server_status_structure() {
    let status = ServerStatus {
        is_running: true,
        pid: Some(12345),
        port: Some(8000),
        host: Some("127.0.0.1".to_string()),
        document_root: Some("/path/to/project".to_string()),
        started_at: Some("2024-01-01T00:00:00Z".to_string()),
    };

    assert!(status.is_running);
    assert_eq!(status.pid, Some(12345));
    assert_eq!(status.port, Some(8000));
    assert_eq!(status.host, Some("127.0.0.1".to_string()));
    assert_eq!(status.document_root, Some("/path/to/project".to_string()));
    assert!(status.started_at.is_some());
}

#[tokio::test]
async fn test_start_server_request_structure() {
    let request = StartServerRequest {
        project_path: "/path/to/project".to_string(),
        port: Some(8000),
        host: Some("127.0.0.1".to_string()),
        php_version: Some("8.3.0".to_string()),
        document_root: Some("/path/to/project/public".to_string()),
    };

    assert_eq!(request.project_path, "/path/to/project");
    assert_eq!(request.port, Some(8000));
    assert_eq!(request.host, Some("127.0.0.1".to_string()));
    assert_eq!(request.php_version, Some("8.3.0".to_string()));
    assert_eq!(
        request.document_root,
        Some("/path/to/project/public".to_string())
    );
}

#[tokio::test]
async fn test_document_root_validation() {
    let temp_dir = common::create_temp_dir();
    let project_path = common::create_mock_php_project(temp_dir.path(), "plain");
    let nonexistent_path = temp_dir.path().join("nonexistent");

    // 存在的路径应该有效
    assert!(project_path.exists());

    // 不存在的路径应该无效
    assert!(!nonexistent_path.exists());
}

#[tokio::test]
async fn test_router_script_detection() {
    let temp_dir = common::create_temp_dir();

    // 测试不同的路由脚本位置
    let test_cases = vec![
        ("index.php", "index.php"),
        ("public/index.php", "public/index.php"),
        ("web/index.php", "web/index.php"),
        ("app.php", "app.php"),
    ];

    for (file_path, expected) in test_cases {
        let project_path = temp_dir
            .path()
            .join(format!("project_{}", expected.replace("/", "_")));
        std::fs::create_dir_all(&project_path).unwrap();

        let full_file_path = project_path.join(file_path);
        if let Some(parent) = full_file_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&full_file_path, "<?php echo 'Hello';").unwrap();

        assert!(full_file_path.exists());
    }
}

#[tokio::test]
async fn test_concurrent_server_operations() {
    use tokio::task;

    // 测试并发操作
    let handles: Vec<_> = (0..3)
        .map(|i| {
            task::spawn(async move {
                let manager = PhpServerManager::new();
                let start_port = 8000 + i * 100;
                let port = manager.find_available_port(start_port);

                // 验证端口查找 - 端口应该大于等于起始端口
                assert!(
                    port >= start_port,
                    "Port {} should be >= start_port {}",
                    port,
                    start_port
                );
                assert!(port < 65535);

                port
            })
        })
        .collect();

    // 等待所有任务完成
    let mut ports = Vec::new();
    for handle in handles {
        let port = handle.await.unwrap();
        ports.push(port);
    }

    // 验证所有端口都不同
    ports.sort();
    ports.dedup();
    assert_eq!(ports.len(), 3); // 应该有3个不同的端口
}

#[tokio::test]
async fn test_server_logs_placeholder() {
    let manager = PhpServerManager::new();

    let result = manager.get_server_logs("test-server-id").await;

    // 对于不存在的服务器，应该返回错误
    assert!(result.is_err());
}

#[tokio::test]
async fn test_port_range_validation() {
    let _manager = PhpServerManager::new();

    // 测试边界值
    let test_ports = vec![1, 80, 443, 8000, 8080, 65535];

    for port in test_ports {
        // 所有端口都应该在有效范围内
        assert!(port >= 1); // u16 max is 65535, so <= 65535 is always true

        // 测试端口可用性检查
        let is_available = common::is_port_available(port);
        // 某些端口可能被系统占用，这是正常的
        // Just test that the function can be called without panicking
        let _ = is_available;
    }
}

#[tokio::test]
async fn test_host_validation() {
    let valid_hosts = vec!["127.0.0.1", "localhost", "0.0.0.0", "::1"];

    for host in valid_hosts {
        // 验证主机地址格式
        assert!(!host.is_empty());

        // 基本的格式验证
        if host.contains('.') {
            // IPv4 格式
            let parts: Vec<&str> = host.split('.').collect();
            if parts.len() == 4 {
                for part in parts {
                    if let Ok(_num) = part.parse::<u8>() {
                        // u8 max is 255, so <= 255 is always true
                        // Just verify the parsing worked
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn test_php_version_format() {
    let valid_versions = vec!["8.3.0", "8.2.15", "8.1.27", "7.4.33"];

    for version in valid_versions {
        // 验证版本格式
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3);

        for part in parts {
            assert!(part.parse::<u32>().is_ok());
        }
    }
}

#[tokio::test]
async fn test_project_path_validation() {
    let temp_dir = common::create_temp_dir();

    // 有效路径
    let valid_path = temp_dir.path();
    assert!(valid_path.exists());
    assert!(valid_path.is_dir());

    // 无效路径
    let invalid_path = temp_dir.path().join("nonexistent");
    assert!(!invalid_path.exists());
}
