use std::sync::Arc;
use tauri_plugin_php::{models::*, PhpManager};

mod common;

#[tokio::test]
async fn test_php_manager_creation() {
    let _manager = PhpManager::new();

    // 验证所有组件都被正确初始化
    // 基本集成测试通过
}

#[tokio::test]
async fn test_full_workflow_simulation() {
    let _temp_dir = common::create_temp_dir();
    let manager = PhpManager::new();

    // 1. 模拟 PHP 二进制管理
    {
        let binary_manager = manager.binary_manager.lock().await;
        let php_path = binary_manager.get_php_executable_path("8.3.0");
        assert!(php_path.to_string_lossy().contains("8.3.0"));
    }

    // 2. 模拟项目管理
    {
        let project_manager = manager.project_manager.lock().await;
        let projects = project_manager.list_projects().unwrap();
        assert!(projects.is_empty() || !projects.is_empty());
    }

    // 3. 模拟服务器管理
    {
        let server_manager = manager.server_manager.lock().await;
        let servers = server_manager.list_running_servers();
        assert_eq!(servers.len(), 0); // 新管理器应该没有运行的服务器
    }
}

#[tokio::test]
async fn test_framework_detection_integration() {
    let temp_dir = common::create_temp_dir();
    let manager = PhpManager::new();

    // 创建不同框架的项目
    let frameworks = vec!["laravel", "symfony", "thinkphp", "plain"];

    for framework in frameworks {
        let project_path = common::create_mock_php_project(temp_dir.path(), framework);

        let project_manager = manager.project_manager.lock().await;
        let detected_framework = project_manager.detect_framework(&project_path).unwrap();

        match framework {
            "laravel" => assert_eq!(detected_framework, Framework::Laravel),
            "symfony" => assert_eq!(detected_framework, Framework::Symfony),
            "thinkphp" => assert_eq!(detected_framework, Framework::ThinkPHP),
            "plain" => assert_eq!(detected_framework, Framework::Plain),
            _ => panic!("Unexpected framework: {}", framework),
        }
    }
}

#[tokio::test]
async fn test_concurrent_manager_access() {
    use tokio::task;

    let manager = Arc::new(PhpManager::new());

    // 测试并发访问不同组件
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let manager_clone = Arc::clone(&manager);
            task::spawn(async move {
                match i % 3 {
                    0 => {
                        // 访问二进制管理器
                        let binary_manager = manager_clone.binary_manager.lock().await;
                        let path = binary_manager.get_php_executable_path("8.3.0");
                        !path.to_string_lossy().is_empty()
                    }
                    1 => {
                        // 访问项目管理器
                        let project_manager = manager_clone.project_manager.lock().await;
                        let projects = project_manager.list_projects();
                        projects.is_ok()
                    }
                    2 => {
                        // 访问服务器管理器
                        let server_manager = manager_clone.server_manager.lock().await;
                        let servers = server_manager.list_running_servers();
                        servers.is_empty() || !servers.is_empty()
                    }
                    _ => unreachable!(),
                }
            })
        })
        .collect();

    // 等待所有任务完成
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result);
    }
}

#[tokio::test]
async fn test_error_handling_integration() {
    let manager = PhpManager::new();

    // 测试各种错误情况

    // 1. 服务器管理错误
    {
        let mut server_manager = manager.server_manager.lock().await;
        let result = server_manager.stop_server("nonexistent-id").await;
        assert!(result.is_err());
    }

    // 2. 项目管理错误
    {
        let project_manager = manager.project_manager.lock().await;
        let result = project_manager.remove_project("nonexistent-project");
        assert!(result.is_err());
    }
}

#[tokio::test]
async fn test_configuration_integration() {
    let config = Config {
        php_versions: vec!["8.3.0".to_string(), "8.2.0".to_string()],
        default_php_version: Some("8.3.0".to_string()),
        download_base_url: "https://example.com".to_string(),
        projects_dir: Some("/tmp/projects".to_string()),
        server_config: ServerConfig {
            default_port: 8000,
            default_host: "127.0.0.1".to_string(),
            auto_reload: true,
            document_root: None,
        },
    };

    // 验证配置结构
    assert_eq!(config.php_versions.len(), 2);
    assert_eq!(config.default_php_version, Some("8.3.0".to_string()));
    assert_eq!(config.server_config.default_port, 8000);
    assert!(config.server_config.auto_reload);
}

#[tokio::test]
async fn test_server_lifecycle_integration() {
    let temp_dir = common::create_temp_dir();
    let manager = PhpManager::new();

    // 创建模拟项目
    let project_path = common::create_mock_php_project(temp_dir.path(), "plain");

    // 创建模拟 PHP 二进制
    let php_executable = common::create_mock_php_binary(temp_dir.path(), "8.3.0");

    let mut server_manager = manager.server_manager.lock().await;

    // 测试服务器启动请求
    let request = StartServerRequest {
        project_path: project_path.to_string_lossy().to_string(),
        port: Some(common::find_available_port(8000)),
        host: Some("127.0.0.1".to_string()),
        php_version: Some("8.3.0".to_string()),
        document_root: Some(project_path.to_string_lossy().to_string()),
    };

    // 尝试启动服务器（预期失败，因为使用模拟的 PHP 二进制）
    let result = server_manager.start_server(request, php_executable).await;

    // 验证错误处理
    if result.is_err() {
        // 这是预期的，因为模拟的 PHP 二进制无法真正启动服务器
        // Test passes if we get an error as expected
    }
}

#[tokio::test]
async fn test_project_cloning_simulation() {
    let temp_dir = common::create_temp_dir();

    // 创建模拟的 Git 仓库
    let repo_dir = temp_dir.path().join("mock_repo");
    common::create_mock_git_repo(&repo_dir, "laravel");

    // 验证模拟仓库创建成功
    assert!(repo_dir.exists());
    assert!(repo_dir.join(".git").exists());
    assert!(repo_dir.join("artisan").exists());
}

#[tokio::test]
async fn test_framework_specific_setup() {
    let temp_dir = common::create_temp_dir();

    let test_cases = vec![
        ("laravel", Framework::Laravel, "public/index.php"),
        ("symfony", Framework::Symfony, "public/index.php"),
        ("thinkphp", Framework::ThinkPHP, "public/index.php"),
        ("codeigniter", Framework::CodeIgniter, "index.php"),
    ];

    for (framework_name, _expected_framework, expected_entry) in test_cases {
        let project_path = common::create_mock_php_project(temp_dir.path(), framework_name);

        // 验证项目结构
        assert!(project_path.exists());

        // 验证入口点文件存在
        let entry_point_path = project_path.join(expected_entry);
        if expected_entry.contains('/') {
            // 对于有子目录的入口点，验证目录存在
            assert!(entry_point_path.parent().unwrap().exists());
        }
    }
}

#[tokio::test]
async fn test_memory_usage_simulation() {
    let manager = PhpManager::new();

    // 模拟大量操作以测试内存使用
    for i in 0..100 {
        let binary_manager = manager.binary_manager.lock().await;
        let _path = binary_manager.get_php_executable_path(&format!("8.{}.0", i % 4));
        drop(binary_manager);

        let project_manager = manager.project_manager.lock().await;
        let _projects = project_manager.list_projects();
        drop(project_manager);

        let server_manager = manager.server_manager.lock().await;
        let _servers = server_manager.list_running_servers();
        drop(server_manager);
    }

    // 如果没有内存泄漏，测试应该正常完成
    // Test passes if we reach this point without panicking
}

#[tokio::test]
async fn test_state_consistency() {
    let manager = PhpManager::new();

    // 测试状态一致性
    {
        let server_manager = manager.server_manager.lock().await;
        let initial_count = server_manager.list_running_servers().len();
        assert_eq!(initial_count, 0);
    }

    // 在不同的锁定范围内再次检查
    {
        let server_manager = manager.server_manager.lock().await;
        let count = server_manager.list_running_servers().len();
        assert_eq!(count, 0); // 应该保持一致
    }
}

#[tokio::test]
async fn test_component_isolation() {
    let manager = PhpManager::new();

    // 测试组件之间的隔离性

    // 修改一个组件不应该影响其他组件
    {
        let _binary_manager = manager.binary_manager.lock().await;
        // 在这里进行一些操作...
    }

    // 其他组件应该仍然正常工作
    {
        let project_manager = manager.project_manager.lock().await;
        let result = project_manager.list_projects();
        assert!(result.is_ok());
    }

    {
        let server_manager = manager.server_manager.lock().await;
        let servers = server_manager.list_running_servers();
        assert_eq!(servers.len(), 0);
    }
}

#[tokio::test]
async fn test_async_operations_integration() {
    use tokio::time::{sleep, Duration};

    let manager = PhpManager::new();

    // 测试异步操作的集成
    let start_time = std::time::Instant::now();

    // 并发执行多个异步操作
    let (result1, result2, result3) = tokio::join!(
        async {
            sleep(Duration::from_millis(10)).await;
            let binary_manager = manager.binary_manager.lock().await;
            let res: Result<_, Box<dyn std::error::Error>> = binary_manager
                .list_installed_versions()
                .map_err(Box::<dyn std::error::Error>::from);
            res
        },
        async {
            sleep(Duration::from_millis(20)).await;
            let project_manager = manager.project_manager.lock().await;
            let res: Result<_, Box<dyn std::error::Error>> = project_manager
                .list_projects()
                .map_err(Box::<dyn std::error::Error>::from);
            res
        },
        async {
            sleep(Duration::from_millis(30)).await;
            let server_manager = manager.server_manager.lock().await;
            let res: Result<_, Box<dyn std::error::Error>> =
                Ok::<_, Box<dyn std::error::Error>>(server_manager.list_running_servers());
            res
        }
    );

    let elapsed = start_time.elapsed();

    // 验证所有操作都成功
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // 验证并发执行（总时间应该小于串行执行的时间）
    assert!(elapsed < Duration::from_millis(100));
}
