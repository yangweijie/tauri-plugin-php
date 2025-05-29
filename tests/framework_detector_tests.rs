use tauri_plugin_php::framework_detector::FrameworkDetector;
use tauri_plugin_php::models::Framework;

mod common;

#[tokio::test]
async fn test_detect_laravel_framework() {
    let temp_dir = common::create_temp_dir();
    let project_path = common::create_mock_php_project(temp_dir.path(), "laravel");

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    assert_eq!(result, Framework::Laravel);
}

#[tokio::test]
async fn test_detect_symfony_framework() {
    let temp_dir = common::create_temp_dir();
    let project_path = common::create_mock_php_project(temp_dir.path(), "symfony");

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    assert_eq!(result, Framework::Symfony);
}

#[tokio::test]
async fn test_detect_thinkphp_framework() {
    let temp_dir = common::create_temp_dir();
    let project_path = common::create_mock_php_project(temp_dir.path(), "thinkphp");

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    assert_eq!(result, Framework::ThinkPHP);
}

#[tokio::test]
async fn test_detect_codeigniter_framework() {
    let temp_dir = common::create_temp_dir();
    let project_path = common::create_mock_php_project(temp_dir.path(), "codeigniter");

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    assert_eq!(result, Framework::CodeIgniter);
}

#[tokio::test]
async fn test_detect_plain_php() {
    let temp_dir = common::create_temp_dir();
    let project_path = common::create_mock_php_project(temp_dir.path(), "plain");

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    assert_eq!(result, Framework::Plain);
}

#[tokio::test]
async fn test_detect_unknown_framework() {
    let temp_dir = common::create_temp_dir();
    let empty_project = temp_dir.path().join("empty_project");
    std::fs::create_dir_all(&empty_project).unwrap();

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&empty_project).unwrap();

    assert_eq!(result, Framework::Unknown);
}

#[tokio::test]
async fn test_get_framework_info() {
    let detector = FrameworkDetector::new();

    // 测试 Laravel 框架信息
    let laravel_info = detector.get_framework_info(&Framework::Laravel);
    assert_eq!(laravel_info.name, "Laravel");
    assert_eq!(laravel_info.default_entry_point, "public/index.php");
    assert!(laravel_info.requires_composer);
    assert_eq!(laravel_info.default_port, 8000);

    // 测试 ThinkPHP 框架信息
    let thinkphp_info = detector.get_framework_info(&Framework::ThinkPHP);
    assert_eq!(thinkphp_info.name, "ThinkPHP");
    assert_eq!(thinkphp_info.default_entry_point, "public/index.php");
    assert!(thinkphp_info.requires_composer);
    assert_eq!(thinkphp_info.default_port, 8000);

    // 测试 CodeIgniter 框架信息
    let ci_info = detector.get_framework_info(&Framework::CodeIgniter);
    assert_eq!(ci_info.name, "CodeIgniter");
    assert_eq!(ci_info.default_entry_point, "index.php");
    assert!(!ci_info.requires_composer);
    assert_eq!(ci_info.default_port, 8080);
}

#[tokio::test]
async fn test_framework_priority() {
    // 测试当项目同时具有多个框架特征时的优先级
    let temp_dir = common::create_temp_dir();
    let project_path = temp_dir.path().join("mixed_project");
    std::fs::create_dir_all(&project_path).unwrap();

    // 创建 Laravel 特征
    std::fs::write(project_path.join("artisan"), "#!/usr/bin/env php").unwrap();

    // 创建 Symfony 特征
    std::fs::create_dir_all(project_path.join("bin")).unwrap();
    std::fs::write(project_path.join("bin/console"), "#!/usr/bin/env php").unwrap();

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    // Laravel 应该有更高的优先级（在检测顺序中更早）
    assert_eq!(result, Framework::Laravel);
}

#[tokio::test]
async fn test_composer_json_detection() {
    let temp_dir = common::create_temp_dir();
    let project_path = temp_dir.path().join("composer_project");
    std::fs::create_dir_all(&project_path).unwrap();

    // 只通过 composer.json 检测框架
    std::fs::write(
        project_path.join("composer.json"),
        r#"{
            "require": {
                "laravel/framework": "^10.0"
            }
        }"#,
    )
    .unwrap();

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    assert_eq!(result, Framework::Laravel);
}

#[tokio::test]
async fn test_thinkphp_version_detection() {
    let temp_dir = common::create_temp_dir();

    // 测试 ThinkPHP 5.x 检测
    let tp5_project = temp_dir.path().join("thinkphp5");
    std::fs::create_dir_all(tp5_project.join("application")).unwrap();
    std::fs::create_dir_all(tp5_project.join("public")).unwrap();
    std::fs::write(
        tp5_project.join("public/index.php"),
        "<?php\nuse think\\App;\n$app = new App();",
    )
    .unwrap();

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&tp5_project).unwrap();
    assert_eq!(result, Framework::ThinkPHP);

    // 测试 ThinkPHP 3.x 检测
    let tp3_project = temp_dir.path().join("thinkphp3");
    std::fs::create_dir_all(tp3_project.join("ThinkPHP")).unwrap();
    std::fs::create_dir_all(tp3_project.join("Application")).unwrap();

    let result = detector.detect_framework(&tp3_project).unwrap();
    assert_eq!(result, Framework::ThinkPHP);
}

#[tokio::test]
async fn test_case_sensitivity() {
    let temp_dir = common::create_temp_dir();
    let project_path = temp_dir.path().join("case_test");
    std::fs::create_dir_all(&project_path).unwrap();

    // 测试大小写敏感性（在某些文件系统上）
    std::fs::write(project_path.join("ARTISAN"), "#!/usr/bin/env php").unwrap();

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    // 在大小写不敏感的文件系统上（如 macOS），ARTISAN 会被识别为 artisan
    // 在大小写敏感的文件系统上，应该检测为 Unknown
    // 我们接受两种结果，因为这取决于文件系统
    assert!(result == Framework::Unknown || result == Framework::Laravel);
}

#[tokio::test]
async fn test_nested_project_detection() {
    let temp_dir = common::create_temp_dir();
    let base_path = temp_dir.path().join("nested");
    let project_path = base_path.join("deep").join("laravel_project");
    std::fs::create_dir_all(&project_path).unwrap();

    // 在嵌套目录中创建 Laravel 项目
    std::fs::write(project_path.join("artisan"), "#!/usr/bin/env php").unwrap();
    std::fs::create_dir_all(project_path.join("app/Http")).unwrap();
    std::fs::write(
        project_path.join("app/Http/Kernel.php"),
        "<?php\nnamespace App\\Http;\nclass Kernel {}",
    )
    .unwrap();

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    assert_eq!(result, Framework::Laravel);
}

#[tokio::test]
async fn test_partial_framework_files() {
    let temp_dir = common::create_temp_dir();
    let project_path = temp_dir.path().join("partial_laravel");
    std::fs::create_dir_all(&project_path).unwrap();

    // 只创建部分 Laravel 文件
    std::fs::write(project_path.join("artisan"), "#!/usr/bin/env php").unwrap();
    // 缺少其他关键文件

    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();

    // 应该仍然检测为 Laravel，因为 artisan 是强特征
    assert_eq!(result, Framework::Laravel);
}
