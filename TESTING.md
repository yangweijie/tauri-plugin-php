# 测试指南

本文档描述了 Tauri PHP 插件的测试策略、测试类型和运行方法。

## 测试概览

我们的测试套件包括以下几个层次：

- **单元测试**: 测试单个组件和函数
- **集成测试**: 测试组件之间的交互
- **端到端测试**: 测试完整的工作流程
- **基准测试**: 性能测试
- **JavaScript 测试**: 前端 API 绑定测试

## 测试结构

```
tauri-plugin-php/
├── tests/                     # 集成测试
│   ├── common/                # 测试工具和辅助函数
│   │   └── mod.rs
│   ├── framework_detector_tests.rs
│   ├── php_binary_tests.rs
│   ├── php_server_tests.rs
│   ├── project_manager_tests.rs
│   └── integration_tests.rs
├── benches/                   # 基准测试
│   └── performance_tests.rs
├── guest-js/tests/           # JavaScript 测试
│   └── api.test.ts
├── scripts/                  # 测试脚本
│   ├── test.sh              # Unix/Linux/macOS
│   └── test.bat             # Windows
└── .github/workflows/        # CI/CD 配置
    └── test.yml
```

## 运行测试

### 快速开始

```bash
# 运行所有测试
./scripts/test.sh

# 或在 Windows 上
scripts\test.bat
```

### 详细选项

```bash
# 只运行 Rust 测试
./scripts/test.sh --rust-only

# 只运行 JavaScript 测试
./scripts/test.sh --js-only

# 运行包含基准测试的完整测试套件
./scripts/test.sh --with-benchmarks

# 运行包含安全审计的测试
./scripts/test.sh --with-audit

# 运行包含覆盖率报告的测试
./scripts/test.sh --with-coverage

# 运行所有测试（包括基准测试、审计和覆盖率）
./scripts/test.sh --all
```

### 手动运行

#### Rust 测试

```bash
# 代码格式检查
cargo fmt --all -- --check

# Clippy 检查
cargo clippy --all-targets --all-features -- -D warnings

# 单元测试
cargo test --lib

# 集成测试
cargo test --test integration_tests
cargo test --test framework_detector_tests
cargo test --test php_binary_tests
cargo test --test php_server_tests
cargo test --test project_manager_tests

# 文档测试
cargo test --doc

# 基准测试
cargo bench
```

#### JavaScript 测试

```bash
cd guest-js

# 安装依赖
npm ci

# 运行测试
npm test

# 运行测试并生成覆盖率报告
npm run test:coverage

# 运行测试 UI
npm run test:ui

# 构建检查
npm run build
```

## 测试类型详解

### 1. 框架检测测试 (`framework_detector_tests.rs`)

测试各种 PHP 框架的自动识别功能：

- Laravel 项目检测
- Symfony 项目检测
- ThinkPHP 项目检测
- CodeIgniter 项目检测
- 原生 PHP 项目检测
- 框架优先级测试
- 边界情况处理

**示例测试**:
```rust
#[tokio::test]
async fn test_detect_thinkphp_framework() {
    let temp_dir = common::create_temp_dir();
    let project_path = common::create_mock_php_project(temp_dir.path(), "thinkphp");
    
    let detector = FrameworkDetector::new();
    let result = detector.detect_framework(&project_path).unwrap();
    
    assert_eq!(result, Framework::ThinkPHP);
}
```

### 2. PHP 二进制管理测试 (`php_binary_tests.rs`)

测试 PHP 二进制文件的下载、管理和版本控制：

- PHP 可执行文件路径生成
- 版本列表管理
- 下载 URL 格式验证
- 文件权限设置
- 并发操作安全性

### 3. PHP 服务器管理测试 (`php_server_tests.rs`)

测试 PHP 开发服务器的启动、停止和监控：

- 端口可用性检查
- 服务器状态管理
- 并发服务器支持
- 错误处理
- 配置验证

### 4. 项目管理测试 (`project_manager_tests.rs`)

测试项目克隆、配置和管理功能：

- Git URL 解析
- 项目名称提取
- 自动配置逻辑
- 环境文件处理
- 目录结构创建

### 5. 集成测试 (`integration_tests.rs`)

测试组件之间的交互和完整工作流程：

- 完整的项目设置流程
- 多组件协作
- 并发访问测试
- 错误传播
- 状态一致性

### 6. JavaScript API 测试 (`guest-js/tests/api.test.ts`)

测试前端 TypeScript API 绑定：

- 类型定义验证
- 工具函数测试
- 配置对象创建
- 枚举值验证
- 边界情况处理

**示例测试**:
```typescript
describe('getDefaultEntryPointForFramework', () => {
  it('should return correct default entry points for frameworks', () => {
    expect(getDefaultEntryPointForFramework(Framework.ThinkPHP)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Laravel)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.CakePHP)).toBe('webroot/index.php')
  })
})
```

## 基准测试

基准测试用于监控性能并识别性能回归：

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准测试
cargo bench framework_detection
cargo bench concurrent_operations
```

基准测试包括：

- 框架检测性能
- 并发操作性能
- 内存使用测试
- 文件操作性能
- 序列化/反序列化性能

## 测试工具和辅助函数

### 通用测试工具 (`tests/common/mod.rs`)

提供了一系列测试辅助函数：

```rust
// 创建临时测试目录
let temp_dir = common::create_temp_dir();

// 创建模拟的 PHP 项目
let project_path = common::create_mock_php_project(temp_dir.path(), "laravel");

// 创建模拟的 PHP 二进制文件
let php_executable = common::create_mock_php_binary(temp_dir.path(), "8.3.0");

// 创建模拟的 Git 仓库
common::create_mock_git_repo(&repo_dir, "symfony");

// 等待条件满足
let success = common::wait_for_condition(|| check_condition(), 5000).await;

// 检查端口可用性
let available = common::is_port_available(8000);
```

## 持续集成

我们使用 GitHub Actions 进行持续集成，配置文件位于 `.github/workflows/test.yml`。

CI 流程包括：

1. **多平台测试**: Ubuntu, Windows, macOS
2. **多 Rust 版本**: stable, beta
3. **代码质量检查**: rustfmt, clippy
4. **安全审计**: cargo-audit, npm audit
5. **覆盖率报告**: 使用 cargo-llvm-cov
6. **基准测试**: 性能回归检测

### 触发条件

- 推送到 `main` 或 `develop` 分支
- 向 `main` 分支提交 Pull Request

## 测试最佳实践

### 1. 测试命名

使用描述性的测试名称：

```rust
#[tokio::test]
async fn test_detect_thinkphp_framework() { /* ... */ }

#[tokio::test]
async fn test_concurrent_server_operations() { /* ... */ }
```

### 2. 测试隔离

每个测试应该是独立的，使用临时目录和资源：

```rust
#[tokio::test]
async fn test_example() {
    let temp_dir = common::create_temp_dir();
    // 测试逻辑...
    // temp_dir 会在测试结束后自动清理
}
```

### 3. 错误测试

确保测试错误情况：

```rust
#[tokio::test]
async fn test_invalid_project_path() {
    let manager = ProjectManager::new();
    let result = manager.remove_project("nonexistent-project");
    assert!(result.is_err());
}
```

### 4. 异步测试

使用 `#[tokio::test]` 进行异步测试：

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = some_async_function().await;
    assert!(result.is_ok());
}
```

## 覆盖率报告

生成覆盖率报告：

```bash
# 安装 cargo-llvm-cov
cargo install cargo-llvm-cov

# 生成 HTML 覆盖率报告
cargo llvm-cov --all-features --workspace --html

# 生成 LCOV 格式报告
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

覆盖率报告将生成在 `target/llvm-cov/html/` 目录中。

## 故障排除

### 常见问题

1. **测试超时**: 增加 `max_wait_seconds` 参数
2. **端口冲突**: 使用 `find_available_port` 函数
3. **文件权限**: 确保测试文件具有正确的权限
4. **依赖缺失**: 运行 `cargo build` 和 `npm ci`

### 调试测试

```bash
# 运行特定测试并显示输出
cargo test test_name -- --nocapture

# 运行测试并显示详细信息
cargo test -- --nocapture --test-threads=1

# 运行 JavaScript 测试并显示详细信息
cd guest-js && npm test -- --reporter=verbose
```

## 贡献测试

在添加新功能时，请确保：

1. 为新功能编写相应的测试
2. 更新现有测试以反映变更
3. 确保所有测试通过
4. 添加适当的文档和注释
5. 考虑边界情况和错误处理

### 测试检查清单

- [ ] 单元测试覆盖新功能
- [ ] 集成测试验证组件交互
- [ ] JavaScript 测试更新（如果适用）
- [ ] 错误情况测试
- [ ] 性能测试（如果适用）
- [ ] 文档更新
- [ ] CI 测试通过
