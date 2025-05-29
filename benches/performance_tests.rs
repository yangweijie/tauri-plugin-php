use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use tauri_plugin_php::{framework_detector::FrameworkDetector, models::Framework, PhpManager};
use tempfile::TempDir;

fn create_temp_project(framework: &str) -> (TempDir, PathBuf) {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_path = temp_dir.path().join("test_project");
    std::fs::create_dir_all(&project_path).unwrap();

    match framework {
        "laravel" => {
            std::fs::write(project_path.join("artisan"), "#!/usr/bin/env php").unwrap();
            std::fs::create_dir_all(project_path.join("app/Http")).unwrap();
            std::fs::write(
                project_path.join("app/Http/Kernel.php"),
                "<?php\nnamespace App\\Http;\nclass Kernel {}",
            )
            .unwrap();
        }
        "symfony" => {
            std::fs::create_dir_all(project_path.join("bin")).unwrap();
            std::fs::write(project_path.join("bin/console"), "#!/usr/bin/env php").unwrap();
            std::fs::create_dir_all(project_path.join("config")).unwrap();
            std::fs::write(project_path.join("config/bundles.php"), "<?php\nreturn [];").unwrap();
        }
        "thinkphp" => {
            std::fs::write(project_path.join("think"), "#!/usr/bin/env php").unwrap();
            std::fs::create_dir_all(project_path.join("config")).unwrap();
            std::fs::write(project_path.join("config/app.php"), "<?php\nreturn [];").unwrap();
        }
        _ => {
            std::fs::write(project_path.join("index.php"), "<?php echo 'Hello';").unwrap();
        }
    }

    (temp_dir, project_path)
}

fn benchmark_framework_detection(c: &mut Criterion) {
    let detector = FrameworkDetector::new();

    c.bench_function("detect_laravel", |b| {
        let (_temp_dir, project_path) = create_temp_project("laravel");
        b.iter(|| black_box(detector.detect_framework(&project_path).unwrap()))
    });

    c.bench_function("detect_symfony", |b| {
        let (_temp_dir, project_path) = create_temp_project("symfony");
        b.iter(|| black_box(detector.detect_framework(&project_path).unwrap()))
    });

    c.bench_function("detect_thinkphp", |b| {
        let (_temp_dir, project_path) = create_temp_project("thinkphp");
        b.iter(|| black_box(detector.detect_framework(&project_path).unwrap()))
    });

    c.bench_function("detect_plain_php", |b| {
        let (_temp_dir, project_path) = create_temp_project("plain");
        b.iter(|| black_box(detector.detect_framework(&project_path).unwrap()))
    });
}

fn benchmark_framework_info(c: &mut Criterion) {
    let detector = FrameworkDetector::new();

    c.bench_function("get_framework_info", |b| {
        b.iter(|| {
            black_box(detector.get_framework_info(&Framework::Laravel));
            black_box(detector.get_framework_info(&Framework::Symfony));
            black_box(detector.get_framework_info(&Framework::ThinkPHP));
            black_box(detector.get_framework_info(&Framework::Plain));
        })
    });
}

fn benchmark_php_manager_creation(c: &mut Criterion) {
    c.bench_function("create_php_manager", |b| {
        b.iter(|| black_box(PhpManager::new()))
    });
}

fn benchmark_php_binary_operations(c: &mut Criterion) {
    use tauri_plugin_php::php_binary::PhpBinaryManager;

    let manager = PhpBinaryManager::new();

    c.bench_function("get_php_executable_path", |b| {
        b.iter(|| black_box(manager.get_php_executable_path("8.3.0")))
    });

    c.bench_function("list_installed_versions", |b| {
        b.iter(|| black_box(manager.list_installed_versions().unwrap()))
    });
}

fn benchmark_server_operations(c: &mut Criterion) {
    use tauri_plugin_php::php_server::PhpServerManager;

    let manager = PhpServerManager::new();

    c.bench_function("find_available_port", |b| {
        b.iter(|| black_box(manager.find_available_port(8000)))
    });

    c.bench_function("list_running_servers", |b| {
        b.iter(|| black_box(manager.list_running_servers()))
    });

    c.bench_function("get_server_status", |b| {
        b.iter(|| black_box(manager.get_server_status("test-id").unwrap()))
    });
}

fn benchmark_project_operations(c: &mut Criterion) {
    use tauri_plugin_php::project_manager::ProjectManager;

    let manager = ProjectManager::new();

    c.bench_function("list_projects", |b| {
        b.iter(|| black_box(manager.list_projects().unwrap()))
    });
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    use std::sync::Arc;
    use tokio::runtime::Runtime;

    let rt = Runtime::new().unwrap();
    let manager = Arc::new(PhpManager::new());

    c.bench_function("concurrent_access", |b| {
        b.iter(|| {
            rt.block_on(async {
                let manager_clone = Arc::clone(&manager);
                let handles: Vec<_> = (0..10)
                    .map(|i| {
                        let manager = Arc::clone(&manager_clone);
                        tokio::spawn(async move {
                            match i % 3 {
                                0 => {
                                    let binary_manager = manager.binary_manager.lock().await;
                                    black_box(binary_manager.get_php_executable_path("8.3.0"))
                                }
                                1 => {
                                    let project_manager = manager.project_manager.lock().await;
                                    black_box(project_manager.list_projects().unwrap());
                                    PathBuf::default()
                                }
                                2 => {
                                    let server_manager = manager.server_manager.lock().await;
                                    black_box(server_manager.list_running_servers());
                                    PathBuf::default()
                                }
                                _ => unreachable!(),
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.await.unwrap();
                }
            })
        })
    });
}

fn benchmark_memory_operations(c: &mut Criterion) {
    c.bench_function("memory_intensive_operations", |b| {
        b.iter(|| {
            // 创建大量临时对象来测试内存性能
            let managers: Vec<_> = (0..100).map(|_| PhpManager::new()).collect();
            black_box(managers)
        })
    });
}

fn benchmark_file_operations(c: &mut Criterion) {
    let detector = FrameworkDetector::new();

    c.bench_function("multiple_framework_detection", |b| {
        let projects: Vec<_> = ["laravel", "symfony", "thinkphp", "plain"]
            .iter()
            .map(|&framework| create_temp_project(framework))
            .collect();

        b.iter(|| {
            for (_temp_dir, project_path) in &projects {
                black_box(detector.detect_framework(project_path).unwrap());
            }
        })
    });
}

fn benchmark_string_operations(c: &mut Criterion) {
    c.bench_function("version_parsing", |b| {
        let versions = vec!["8.3.0", "8.2.15", "8.1.27", "7.4.33"];
        b.iter(|| {
            for version in &versions {
                let parts: Vec<&str> = version.split('.').collect();
                black_box(parts);
            }
        })
    });

    c.bench_function("url_parsing", |b| {
        let urls = vec![
            "https://github.com/user/repo.git",
            "https://gitlab.com/group/project.git",
            "git@github.com:user/repo.git",
        ];
        b.iter(|| {
            for url in &urls {
                let name = url
                    .trim_end_matches(".git")
                    .split('/')
                    .next_back()
                    .unwrap_or("unknown");
                black_box(name);
            }
        })
    });
}

fn benchmark_serialization(c: &mut Criterion) {
    use tauri_plugin_php::models::*;

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

    c.bench_function("config_serialization", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&config).unwrap();
            black_box(json)
        })
    });

    c.bench_function("config_deserialization", |b| {
        let json = serde_json::to_string(&config).unwrap();
        b.iter(|| {
            let parsed: Config = serde_json::from_str(&json).unwrap();
            black_box(parsed)
        })
    });
}

criterion_group!(
    benches,
    benchmark_framework_detection,
    benchmark_framework_info,
    benchmark_php_manager_creation,
    benchmark_php_binary_operations,
    benchmark_server_operations,
    benchmark_project_operations,
    benchmark_concurrent_operations,
    benchmark_memory_operations,
    benchmark_file_operations,
    benchmark_string_operations,
    benchmark_serialization
);

criterion_main!(benches);
