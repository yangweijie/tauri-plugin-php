import { invoke } from '@tauri-apps/api/core'

export interface PhpBinaryInfo {
  version: string
  path: string
  is_downloaded: boolean
  download_url?: string
  size?: number
}

export interface ServerStatus {
  is_running: boolean
  pid?: number
  port?: number
  host?: string
  document_root?: string
  started_at?: string
}

export interface ProjectInfo {
  name: string
  path: string
  framework?: Framework
  git_url?: string
  php_version?: string
  entry_point?: string
}

export enum Framework {
  Laravel = 'Laravel',
  Symfony = 'Symfony',
  CodeIgniter = 'CodeIgniter',
  CakePHP = 'CakePHP',
  Zend = 'Zend',
  Yii = 'Yii',
  Phalcon = 'Phalcon',
  Slim = 'Slim',
  Lumen = 'Lumen',
  ThinkPHP = 'ThinkPHP',
  Plain = 'Plain',
  Unknown = 'Unknown'
}

export interface StartServerRequest {
  project_path: string
  port?: number
  host?: string
  php_version?: string
  document_root?: string
}

export interface CloneProjectRequest {
  git_url: string
  destination?: string
  branch?: string
  auto_setup: boolean
}

export interface Config {
  php_versions: string[]
  default_php_version?: string
  download_base_url: string
  projects_dir?: string
  server_config: ServerConfig
}

export interface ServerConfig {
  default_port: number
  default_host: string
  auto_reload: boolean
  document_root?: string
}

export interface FrameworkInfo {
  name: string
  default_entry_point: string
  requires_composer: boolean
  setup_commands: string[]
  default_port: number
}

/**
 * Downloads a PHP binary for the specified version
 */
export async function downloadPhpBinary(version: string): Promise<PhpBinaryInfo> {
  return await invoke('plugin:php|download_php_binary', { version })
}

/**
 * Starts a PHP development server
 */
export async function startPhpServer(request: StartServerRequest): Promise<string> {
  return await invoke('plugin:php|start_php_server', { request })
}

/**
 * Stops a running PHP server
 */
export async function stopPhpServer(serverId: string): Promise<void> {
  return await invoke('plugin:php|stop_php_server', { serverId })
}

/**
 * Gets the status of a PHP server
 */
export async function getServerStatus(serverId: string): Promise<ServerStatus> {
  return await invoke('plugin:php|get_server_status', { serverId })
}

/**
 * Clones a project from a Git repository
 */
export async function cloneProject(request: CloneProjectRequest): Promise<ProjectInfo> {
  return await invoke('plugin:php|clone_project', { request })
}

/**
 * Detects the framework used in a project
 */
export async function detectFramework(projectPath: string): Promise<Framework> {
  return await invoke('plugin:php|detect_framework', { projectPath })
}

/**
 * Gets the PHP version information
 */
export async function getPhpVersion(version: string): Promise<string> {
  return await invoke('plugin:php|get_php_version', { version })
}

/**
 * Lists all available PHP versions
 */
export async function listAvailablePhpVersions(): Promise<PhpBinaryInfo[]> {
  return await invoke('plugin:php|list_available_php_versions')
}

/**
 * Sets the plugin configuration
 */
export async function setPhpConfig(config: Config): Promise<void> {
  return await invoke('plugin:php|set_php_config', { config })
}

/**
 * Gets the plugin configuration
 */
export async function getPhpConfig(): Promise<Config> {
  return await invoke('plugin:php|get_php_config')
}

/**
 * Lists all running PHP servers
 */
export async function listRunningServers(): Promise<Array<[string, ServerStatus]>> {
  return await invoke('plugin:php|list_running_servers')
}

/**
 * Lists all projects
 */
export async function listProjects(): Promise<ProjectInfo[]> {
  return await invoke('plugin:php|list_projects')
}

/**
 * Removes a project
 */
export async function removeProject(projectName: string): Promise<void> {
  return await invoke('plugin:php|remove_project', { projectName })
}

/**
 * Removes a PHP binary
 */
export async function removePhpBinary(version: string): Promise<void> {
  return await invoke('plugin:php|remove_php_binary', { version })
}

/**
 * Gets server logs
 */
export async function getServerLogs(serverId: string): Promise<string> {
  return await invoke('plugin:php|get_server_logs', { serverId })
}

/**
 * Finds an available port starting from the specified port
 */
export async function findAvailablePort(startPort: number): Promise<number> {
  return await invoke('plugin:php|find_available_port', { startPort })
}

/**
 * Stops all running servers
 */
export async function stopAllServers(): Promise<void> {
  return await invoke('plugin:php|stop_all_servers')
}

/**
 * Gets framework information
 */
export async function getFrameworkInfo(framework: Framework): Promise<FrameworkInfo> {
  return await invoke('plugin:php|get_framework_info', { framework })
}

/**
 * Validates if a project path exists
 */
export async function validateProjectPath(path: string): Promise<boolean> {
  return await invoke('plugin:php|validate_project_path', { path })
}

/**
 * Gets project information from a path
 */
export async function getProjectInfo(projectPath: string): Promise<ProjectInfo> {
  return await invoke('plugin:php|get_project_info', { projectPath })
}

// Utility functions

/**
 * Creates a default configuration
 */
export function createDefaultConfig(): Config {
  return {
    php_versions: ['8.3.0', '8.2.0', '8.1.0'],
    default_php_version: '8.3.0',
    download_base_url: 'https://github.com/NativePHP/php-bin/releases/download',
    projects_dir: undefined,
    server_config: {
      default_port: 8000,
      default_host: '127.0.0.1',
      auto_reload: true,
      document_root: undefined
    }
  }
}

/**
 * Creates a default server start request
 */
export function createStartServerRequest(projectPath: string, options?: Partial<StartServerRequest>): StartServerRequest {
  return {
    project_path: projectPath,
    port: options?.port,
    host: options?.host || '127.0.0.1',
    php_version: options?.php_version,
    document_root: options?.document_root
  }
}

/**
 * Creates a default clone project request
 */
export function createCloneProjectRequest(gitUrl: string, options?: Partial<CloneProjectRequest>): CloneProjectRequest {
  return {
    git_url: gitUrl,
    destination: options?.destination,
    branch: options?.branch,
    auto_setup: options?.auto_setup ?? true
  }
}

/**
 * Gets the default port for a framework
 */
export function getDefaultPortForFramework(framework: Framework): number {
  switch (framework) {
    case Framework.Laravel:
    case Framework.Lumen:
      return 8000
    case Framework.CakePHP:
      return 8765
    default:
      return 8080
  }
}

/**
 * Gets the default entry point for a framework
 */
export function getDefaultEntryPointForFramework(framework: Framework): string {
  switch (framework) {
    case Framework.Laravel:
    case Framework.Symfony:
    case Framework.Lumen:
    case Framework.Phalcon:
    case Framework.Slim:
    case Framework.Zend:
    case Framework.ThinkPHP:
      return 'public/index.php'
    case Framework.CakePHP:
      return 'webroot/index.php'
    case Framework.Yii:
      return 'web/index.php'
    case Framework.CodeIgniter:
    case Framework.Plain:
    default:
      return 'index.php'
  }
}
