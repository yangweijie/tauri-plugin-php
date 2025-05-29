# Tauri Plugin PHP

一个为 Tauri 应用程序提供 PHP 开发服务器管理功能的插件。

## 功能特性

- 🚀 **PHP 二进制管理**: 自动下载和管理多个 PHP 版本
- 🌐 **PHP 开发服务器**: 启动、停止和监控 PHP 开发服务器
- 📁 **项目管理**: 从 Git 仓库克隆项目并自动配置
- 🔍 **框架识别**: 自动识别 Laravel、Symfony、CodeIgniter 等主流 PHP 框架
- ⚙️ **自动配置**: 根据框架类型自动配置项目环境

## 支持的框架

- Laravel
- Symfony
- CodeIgniter
- CakePHP
- Zend/Laminas
- Yii
- Phalcon
- Slim
- Lumen
- ThinkPHP (3.x/5.x/6.x)
- 原生 PHP

## 安装

### Rust 依赖

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
tauri-plugin-php = "0.1.0"
# 或者从 Git 安装:
tauri-plugin-php = { git = "https://github.com/tauri-apps/tauri-plugin-php", branch = "main" }
```

### JavaScript 绑定

```bash
pnpm add @tauri-apps/plugin-php
# 或者
npm add @tauri-apps/plugin-php
# 或者
yarn add @tauri-apps/plugin-php
```

## 使用方法

### 注册插件

在你的 Tauri 应用的 `src-tauri/src/lib.rs` 或 `src-tauri/src/main.rs` 中：

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_php::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 权限配置

在你的 `src-tauri/capabilities/default.json` 中添加权限：

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "php:default"
  ]
}
```

### JavaScript API

```typescript
import {
  downloadPhpBinary,
  startPhpServer,
  stopPhpServer,
  cloneProject,
  detectFramework,
  Framework
} from '@tauri-apps/plugin-php'

// 下载 PHP 二进制文件
await downloadPhpBinary('8.3.0')

// 启动 PHP 服务器
const serverId = await startPhpServer({
  project_path: '/path/to/your/project',
  port: 8000,
  php_version: '8.3.0'
})

// 克隆项目
const projectInfo = await cloneProject({
  git_url: 'https://github.com/laravel/laravel.git',
  auto_setup: true
})

// 检测框架
const framework = await detectFramework('/path/to/project')
console.log(framework) // Framework.Laravel

// 停止服务器
await stopPhpServer(serverId)
```

## API 参考

### PHP 二进制管理

- `downloadPhpBinary(version: string)`: 下载指定版本的 PHP 二进制文件
- `listAvailablePhpVersions()`: 列出所有可用的 PHP 版本
- `getPhpVersion(version: string)`: 获取 PHP 版本信息
- `removePhpBinary(version: string)`: 删除 PHP 二进制文件

### PHP 服务器管理

- `startPhpServer(request: StartServerRequest)`: 启动 PHP 开发服务器
- `stopPhpServer(serverId: string)`: 停止指定的服务器
- `getServerStatus(serverId: string)`: 获取服务器状态
- `listRunningServers()`: 列出所有运行中的服务器
- `stopAllServers()`: 停止所有服务器

### 项目管理

- `cloneProject(request: CloneProjectRequest)`: 从 Git 仓库克隆项目
- `detectFramework(projectPath: string)`: 检测项目使用的框架
- `listProjects()`: 列出所有项目
- `removeProject(projectName: string)`: 删除项目

### 配置管理

- `setPhpConfig(config: Config)`: 设置插件配置
- `getPhpConfig()`: 获取插件配置

## 配置选项

```typescript
interface Config {
  php_versions: string[]           // 可用的 PHP 版本列表
  default_php_version?: string     // 默认 PHP 版本
  download_base_url: string        // PHP 二进制下载基础 URL
  projects_dir?: string            // 项目存储目录
  server_config: ServerConfig      // 服务器配置
}

interface ServerConfig {
  default_port: number             // 默认端口
  default_host: string             // 默认主机
  auto_reload: boolean             // 自动重载
  document_root?: string           // 文档根目录
}
```

## 示例

### 完整的工作流程

```typescript
import {
  downloadPhpBinary,
  cloneProject,
  startPhpServer,
  getServerStatus,
  Framework
} from '@tauri-apps/plugin-php'

async function setupAndRunProject() {
  try {
    // 1. 下载 PHP 8.3.0
    console.log('下载 PHP 二进制文件...')
    await downloadPhpBinary('8.3.0')

    // 2. 克隆 Laravel 项目
    console.log('克隆项目...')
    const project = await cloneProject({
      git_url: 'https://github.com/laravel/laravel.git',
      destination: './my-laravel-app',
      auto_setup: true
    })

    // 3. 启动服务器
    console.log('启动 PHP 服务器...')
    const serverId = await startPhpServer({
      project_path: project.path,
      port: 8000,
      php_version: '8.3.0'
    })

    // 4. 检查服务器状态
    const status = await getServerStatus(serverId)
    console.log('服务器状态:', status)

    if (status.is_running) {
      console.log(`服务器运行在 http://${status.host}:${status.port}`)
    }

  } catch (error) {
    console.error('错误:', error)
  }
}

setupAndRunProject()
```

### ThinkPHP 项目示例

```typescript
import {
  downloadPhpBinary,
  cloneProject,
  startPhpServer,
  detectFramework,
  Framework
} from '@tauri-apps/plugin-php'

async function setupThinkPHPProject() {
  try {
    // 1. 下载 PHP 8.1.0 (ThinkPHP 6.x 推荐)
    console.log('下载 PHP 二进制文件...')
    await downloadPhpBinary('8.1.0')

    // 2. 克隆 ThinkPHP 项目
    console.log('克隆 ThinkPHP 项目...')
    const project = await cloneProject({
      git_url: 'https://github.com/top-think/think.git',
      destination: './my-thinkphp-app',
      auto_setup: true  // 自动安装 Composer 依赖和配置环境
    })

    // 3. 验证框架检测
    const framework = await detectFramework(project.path)
    console.log('检测到框架:', framework) // Framework.ThinkPHP

    // 4. 启动服务器
    console.log('启动 ThinkPHP 服务器...')
    const serverId = await startPhpServer({
      project_path: project.path,
      port: 8000,
      php_version: '8.1.0',
      document_root: project.path + '/public'  // ThinkPHP 6.x 使用 public 目录
    })

    console.log(`ThinkPHP 应用运行在 http://127.0.0.1:8000`)

  } catch (error) {
    console.error('错误:', error)
  }
}

setupThinkPHPProject()
```

## 开发

### 构建插件

```bash
cargo build
```

### 构建 JavaScript 绑定

```bash
cd guest-js
npm install
npm run build
```

### 运行测试

```bash
# 运行所有测试
./scripts/test.sh

# 或在 Windows 上
scripts\test.bat

# 手动运行 Rust 测试
cargo test

# 手动运行 JavaScript 测试
cd guest-js && npm test
```

更多测试信息请参阅 [TESTING.md](TESTING.md)。

## 许可证

本项目采用 Apache-2.0 或 MIT 双重许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### 0.1.0

- 初始版本
- 支持 PHP 二进制下载和管理
- 支持 PHP 开发服务器启动和管理
- 支持项目克隆和框架识别
- 支持多种主流 PHP 框架（包括 ThinkPHP）
