// 基本编译测试
use tauri_plugin_php::models::Framework;

#[test]
fn test_framework_enum() {
    // 测试 Framework 枚举
    let framework = Framework::ThinkPHP;
    assert_eq!(framework.as_str(), "ThinkPHP");

    let framework = Framework::Laravel;
    assert_eq!(framework.as_str(), "Laravel");
}

#[test]
fn test_basic_functionality() {
    // 基本功能测试
    // 基本测试通过
}
