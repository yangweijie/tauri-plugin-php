use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tempfile::TempDir;

/// 创建临时测试目录
pub fn create_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

/// 创建模拟的 PHP 项目结构
pub fn create_mock_php_project(base_dir: &Path, framework: &str) -> PathBuf {
    let project_dir = base_dir.join(format!("test_project_{}", framework));
    fs::create_dir_all(&project_dir).expect("Failed to create project directory");

    match framework {
        "laravel" => create_laravel_project(&project_dir),
        "symfony" => create_symfony_project(&project_dir),
        "thinkphp" => create_thinkphp_project(&project_dir),
        "codeigniter" => create_codeigniter_project(&project_dir),
        "plain" => create_plain_php_project(&project_dir),
        _ => panic!("Unknown framework: {}", framework),
    }

    project_dir
}

fn create_laravel_project(project_dir: &Path) {
    // 创建 Laravel 特征文件
    fs::write(
        project_dir.join("artisan"),
        "#!/usr/bin/env php\n<?php\n// Laravel artisan",
    )
    .unwrap();

    // 创建目录结构
    fs::create_dir_all(project_dir.join("app/Http")).unwrap();
    fs::create_dir_all(project_dir.join("bootstrap")).unwrap();
    fs::create_dir_all(project_dir.join("config")).unwrap();
    fs::create_dir_all(project_dir.join("public")).unwrap();

    // 创建关键文件
    fs::write(
        project_dir.join("app/Http/Kernel.php"),
        "<?php\nnamespace App\\Http;\nclass Kernel {}",
    )
    .unwrap();

    fs::write(
        project_dir.join("bootstrap/app.php"),
        "<?php\n$app = new Illuminate\\Foundation\\Application();",
    )
    .unwrap();

    fs::write(
        project_dir.join("config/app.php"),
        "<?php\nreturn ['name' => 'Laravel'];",
    )
    .unwrap();

    fs::write(
        project_dir.join("public/index.php"),
        "<?php\nrequire_once __DIR__.'/../vendor/autoload.php';",
    )
    .unwrap();

    // 创建 composer.json
    fs::write(
        project_dir.join("composer.json"),
        r#"{
    "name": "laravel/laravel",
    "require": {
        "php": "^8.1",
        "laravel/framework": "^10.0"
    }
}"#,
    )
    .unwrap();

    // 创建 .env.example
    fs::write(
        project_dir.join(".env.example"),
        "APP_NAME=Laravel\nAPP_ENV=local\nAPP_KEY=\nAPP_DEBUG=true",
    )
    .unwrap();
}

fn create_symfony_project(project_dir: &Path) {
    // 创建 Symfony 特征文件
    fs::create_dir_all(project_dir.join("bin")).unwrap();
    fs::create_dir_all(project_dir.join("config")).unwrap();
    fs::create_dir_all(project_dir.join("src")).unwrap();
    fs::create_dir_all(project_dir.join("public")).unwrap();

    fs::write(
        project_dir.join("bin/console"),
        "#!/usr/bin/env php\n<?php\n// Symfony console",
    )
    .unwrap();

    fs::write(project_dir.join("config/bundles.php"), "<?php\nreturn [];").unwrap();

    fs::write(
        project_dir.join("src/Kernel.php"),
        "<?php\nnamespace App;\nclass Kernel {}",
    )
    .unwrap();

    fs::write(
        project_dir.join("public/index.php"),
        "<?php\nuse App\\Kernel;",
    )
    .unwrap();

    fs::write(
        project_dir.join("composer.json"),
        r#"{
    "name": "symfony/skeleton",
    "require": {
        "php": ">=8.1",
        "symfony/framework-bundle": "^6.0"
    }
}"#,
    )
    .unwrap();
}

fn create_thinkphp_project(project_dir: &Path) {
    // 创建 ThinkPHP 6.x 结构
    fs::create_dir_all(project_dir.join("app")).unwrap();
    fs::create_dir_all(project_dir.join("config")).unwrap();
    fs::create_dir_all(project_dir.join("public")).unwrap();
    fs::create_dir_all(project_dir.join("runtime")).unwrap();

    // 创建 think 命令文件
    fs::write(
        project_dir.join("think"),
        "#!/usr/bin/env php\n<?php\nuse think\\Console;",
    )
    .unwrap();

    // 创建配置文件 (使用 ThinkPHP 特有的配置文件名)
    fs::write(
        project_dir.join("config/database.php"),
        "<?php\nreturn ['default' => 'mysql'];",
    )
    .unwrap();

    // 创建入口文件
    fs::write(
        project_dir.join("public/index.php"),
        "<?php\nuse think\\App;\n$app = new App();",
    )
    .unwrap();

    // 创建 composer.json
    fs::write(
        project_dir.join("composer.json"),
        r#"{
    "name": "topthink/think",
    "require": {
        "php": ">=7.2.5",
        "topthink/framework": "^6.0"
    }
}"#,
    )
    .unwrap();

    // 创建 .example.env
    fs::write(
        project_dir.join(".example.env"),
        "APP_DEBUG = true\nDATABASE_TYPE = mysql",
    )
    .unwrap();
}

fn create_codeigniter_project(project_dir: &Path) {
    // 创建 CodeIgniter 结构
    fs::create_dir_all(project_dir.join("system")).unwrap();
    fs::create_dir_all(project_dir.join("application/config")).unwrap();

    fs::write(
        project_dir.join("system/CodeIgniter.php"),
        "<?php\nclass CodeIgniter {}",
    )
    .unwrap();

    fs::write(
        project_dir.join("application/config/config.php"),
        "<?php\n$config['base_url'] = '';",
    )
    .unwrap();

    fs::write(
        project_dir.join("index.php"),
        "<?php\ndefine('BASEPATH', 'system/');\nrequire_once BASEPATH.'CodeIgniter.php';",
    )
    .unwrap();
}

fn create_plain_php_project(project_dir: &Path) {
    fs::write(
        project_dir.join("index.php"),
        "<?php\necho 'Hello, World!';",
    )
    .unwrap();

    fs::write(project_dir.join("about.php"), "<?php\necho 'About page';").unwrap();
}

/// 创建模拟的 PHP 二进制文件
pub fn create_mock_php_binary(base_dir: &Path, version: &str) -> PathBuf {
    let php_dir = base_dir.join("php-binaries").join(version);
    let php_executable = if cfg!(windows) {
        php_dir.join("php.exe")
    } else {
        php_dir.join("bin").join("php")
    };

    fs::create_dir_all(php_executable.parent().unwrap()).unwrap();
    fs::write(
        &php_executable,
        "#!/usr/bin/env php\n<?php echo 'PHP Binary Mock';",
    )
    .unwrap();

    // 在 Unix 系统上设置可执行权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&php_executable).unwrap().permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&php_executable, permissions).unwrap();
    }

    php_executable
}

/// 创建模拟的 Git 仓库
pub fn create_mock_git_repo(repo_dir: &Path, framework: &str) {
    fs::create_dir_all(repo_dir).unwrap();
    // 简单的 .git 目录模拟
    fs::create_dir_all(repo_dir.join(".git")).unwrap();

    // 根据框架创建一些特征文件
    match framework {
        "laravel" => {
            fs::write(repo_dir.join("artisan"), "").unwrap();
            fs::write(
                repo_dir.join("composer.json"),
                "{\"require\":{\"laravel/framework\":\"^10.0\"}}",
            )
            .unwrap();
        }
        "symfony" => {
            fs::create_dir_all(repo_dir.join("bin")).unwrap();
            fs::write(repo_dir.join("bin/console"), "").unwrap();
            fs::write(
                repo_dir.join("composer.json"),
                "{\"require\":{\"symfony/symfony\":\"^6.0\"}}",
            )
            .unwrap();
        }
        _ => {
            // 默认创建 index.php
            fs::write(repo_dir.join("index.php"), "").unwrap();
        }
    }
}

/// 检查端口是否可用
pub fn is_port_available(port: u16) -> bool {
    std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}

/// 异步等待条件满足
pub async fn wait_for_condition<F>(timeout_ms: u64, condition: F) -> bool
where
    F: Fn() -> bool,
{
    let start_time = std::time::Instant::now();
    while start_time.elapsed().as_millis() < timeout_ms as u128 {
        if condition() {
            return true;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    false
}

pub fn find_available_port(start_port: u16) -> u16 {
    for port in start_port..=65535 {
        if is_port_available(port) {
            return port;
        }
    }
    panic!("No available port found in range {}..65535", start_port);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_temp_dir() {
        let temp_dir = create_temp_dir();
        assert!(temp_dir.path().exists());
        assert!(temp_dir.path().is_dir());
    }

    #[test]
    fn test_create_mock_php_project_laravel() {
        let temp_dir = create_temp_dir();
        let project_path = create_mock_php_project(temp_dir.path(), "laravel");
        assert!(project_path.exists());
        assert!(project_path.join("artisan").exists());
        assert!(project_path.join("app/Http/Kernel.php").exists());
        assert!(project_path.join("composer.json").exists());
    }

    #[test]
    fn test_create_mock_php_project_symfony() {
        let temp_dir = create_temp_dir();
        let project_path = create_mock_php_project(temp_dir.path(), "symfony");
        assert!(project_path.exists());
        assert!(project_path.join("bin/console").exists());
        assert!(project_path.join("config/bundles.php").exists());
        assert!(project_path.join("composer.json").exists());
    }

    #[test]
    fn test_create_mock_php_project_thinkphp() {
        let temp_dir = create_temp_dir();
        let project_path = create_mock_php_project(temp_dir.path(), "thinkphp");
        assert!(project_path.exists());
        assert!(project_path.join("think").exists());
        assert!(project_path.join("config/database.php").exists());
        assert!(project_path.join("composer.json").exists());
    }

    #[test]
    fn test_create_mock_php_project_codeigniter() {
        let temp_dir = create_temp_dir();
        let project_path = create_mock_php_project(temp_dir.path(), "codeigniter");
        assert!(project_path.exists());
        assert!(project_path.join("system/CodeIgniter.php").exists());
        assert!(project_path.join("application/config/config.php").exists());
    }

    #[test]
    fn test_create_mock_php_project_plain() {
        let temp_dir = create_temp_dir();
        let project_path = create_mock_php_project(temp_dir.path(), "plain");
        assert!(project_path.exists());
        assert!(project_path.join("index.php").exists());
    }

    #[test]
    #[should_panic(expected = "Unknown framework")]
    fn test_create_mock_php_project_unknown() {
        let temp_dir = create_temp_dir();
        create_mock_php_project(temp_dir.path(), "unknown_framework");
    }

    #[test]
    fn test_create_mock_php_binary() {
        let temp_dir = create_temp_dir();
        let php_executable = create_mock_php_binary(temp_dir.path(), "8.3.0");
        assert!(php_executable.exists());
        assert!(php_executable.is_file());
    }

    #[test]
    fn test_create_mock_git_repo() {
        let temp_dir = create_temp_dir();
        let repo_dir = temp_dir.path().join("test_repo");
        create_mock_git_repo(&repo_dir, "laravel");
        assert!(repo_dir.exists());
        assert!(repo_dir.is_dir());
        assert!(repo_dir.join(".git").exists());
        assert!(repo_dir.join("artisan").exists());
    }

    #[test]
    fn test_is_port_available() {
        // 找到一个可用端口
        let port = find_available_port(8000);
        assert!(is_port_available(port));

        // 绑定端口以使其不可用
        let listener = std::net::TcpListener::bind(("127.0.0.1", port)).unwrap();
        assert!(!is_port_available(port));
        drop(listener); // 释放端口
    }

    #[tokio::test]
    async fn test_wait_for_condition() {
        let counter = Rc::new(RefCell::new(0));
        let counter_clone = Rc::clone(&counter);
        let condition = move || {
            *counter_clone.borrow_mut() += 1;
            *counter_clone.borrow() > 2
        };
        let result = wait_for_condition(500, condition).await;
        assert!(result);
        assert!(*counter.borrow() > 2);

        let counter_fail = Rc::new(RefCell::new(0));
        let counter_fail_clone = Rc::clone(&counter_fail);
        let condition_fail = move || {
            *counter_fail_clone.borrow_mut() += 1;
            false
        };
        let result_fail = wait_for_condition(100, condition_fail).await;
        assert!(!result_fail);
        assert!(*counter_fail.borrow() > 0);
    }
}
