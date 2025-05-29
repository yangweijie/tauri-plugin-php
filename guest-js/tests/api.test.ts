import { describe, it, expect, beforeEach, afterEach } from 'vitest'
import {
  Framework,
  createDefaultConfig,
  createStartServerRequest,
  createCloneProjectRequest,
  getDefaultPortForFramework,
  getDefaultEntryPointForFramework,
  type Config,
  type StartServerRequest,
  type CloneProjectRequest,
  type ProjectInfo,
  type ServerStatus,
  type PhpBinaryInfo
} from '../index'

describe('Framework enum', () => {
  it('should have all expected framework values', () => {
    expect(Framework.Laravel).toBe('Laravel')
    expect(Framework.Symfony).toBe('Symfony')
    expect(Framework.CodeIgniter).toBe('CodeIgniter')
    expect(Framework.CakePHP).toBe('CakePHP')
    expect(Framework.Zend).toBe('Zend')
    expect(Framework.Yii).toBe('Yii')
    expect(Framework.Phalcon).toBe('Phalcon')
    expect(Framework.Slim).toBe('Slim')
    expect(Framework.Lumen).toBe('Lumen')
    expect(Framework.ThinkPHP).toBe('ThinkPHP')
    expect(Framework.Plain).toBe('Plain')
    expect(Framework.Unknown).toBe('Unknown')
  })
})

describe('createDefaultConfig', () => {
  it('should create a valid default configuration', () => {
    const config = createDefaultConfig()
    
    expect(config.php_versions).toEqual(['8.3.0', '8.2.0', '8.1.0'])
    expect(config.default_php_version).toBe('8.3.0')
    expect(config.download_base_url).toBe('https://github.com/NativePHP/php-bin/releases/download')
    expect(config.projects_dir).toBeUndefined()
    expect(config.server_config.default_port).toBe(8000)
    expect(config.server_config.default_host).toBe('127.0.0.1')
    expect(config.server_config.auto_reload).toBe(true)
    expect(config.server_config.document_root).toBeUndefined()
  })
})

describe('createStartServerRequest', () => {
  it('should create a valid start server request with minimal options', () => {
    const request = createStartServerRequest('/path/to/project')
    
    expect(request.project_path).toBe('/path/to/project')
    expect(request.host).toBe('127.0.0.1')
    expect(request.port).toBeUndefined()
    expect(request.php_version).toBeUndefined()
    expect(request.document_root).toBeUndefined()
  })
  
  it('should create a valid start server request with all options', () => {
    const options = {
      port: 8080,
      host: 'localhost',
      php_version: '8.3.0',
      document_root: '/path/to/project/public'
    }
    
    const request = createStartServerRequest('/path/to/project', options)
    
    expect(request.project_path).toBe('/path/to/project')
    expect(request.port).toBe(8080)
    expect(request.host).toBe('localhost')
    expect(request.php_version).toBe('8.3.0')
    expect(request.document_root).toBe('/path/to/project/public')
  })
})

describe('createCloneProjectRequest', () => {
  it('should create a valid clone project request with minimal options', () => {
    const request = createCloneProjectRequest('https://github.com/user/repo.git')
    
    expect(request.git_url).toBe('https://github.com/user/repo.git')
    expect(request.auto_setup).toBe(true)
    expect(request.destination).toBeUndefined()
    expect(request.branch).toBeUndefined()
  })
  
  it('should create a valid clone project request with all options', () => {
    const options = {
      destination: '/path/to/destination',
      branch: 'develop',
      auto_setup: false
    }
    
    const request = createCloneProjectRequest('https://github.com/user/repo.git', options)
    
    expect(request.git_url).toBe('https://github.com/user/repo.git')
    expect(request.destination).toBe('/path/to/destination')
    expect(request.branch).toBe('develop')
    expect(request.auto_setup).toBe(false)
  })
})

describe('getDefaultPortForFramework', () => {
  it('should return correct default ports for frameworks', () => {
    expect(getDefaultPortForFramework(Framework.Laravel)).toBe(8000)
    expect(getDefaultPortForFramework(Framework.Lumen)).toBe(8000)
    expect(getDefaultPortForFramework(Framework.CakePHP)).toBe(8765)
    expect(getDefaultPortForFramework(Framework.Symfony)).toBe(8080)
    expect(getDefaultPortForFramework(Framework.CodeIgniter)).toBe(8080)
    expect(getDefaultPortForFramework(Framework.ThinkPHP)).toBe(8080)
    expect(getDefaultPortForFramework(Framework.Plain)).toBe(8080)
    expect(getDefaultPortForFramework(Framework.Unknown)).toBe(8080)
  })
})

describe('getDefaultEntryPointForFramework', () => {
  it('should return correct default entry points for frameworks', () => {
    expect(getDefaultEntryPointForFramework(Framework.Laravel)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Symfony)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Lumen)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Phalcon)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Slim)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Zend)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.ThinkPHP)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.CakePHP)).toBe('webroot/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Yii)).toBe('web/index.php')
    expect(getDefaultEntryPointForFramework(Framework.CodeIgniter)).toBe('index.php')
    expect(getDefaultEntryPointForFramework(Framework.Plain)).toBe('index.php')
    expect(getDefaultEntryPointForFramework(Framework.Unknown)).toBe('index.php')
  })
})

describe('Type definitions', () => {
  it('should have correct Config type structure', () => {
    const config: Config = {
      php_versions: ['8.3.0'],
      default_php_version: '8.3.0',
      download_base_url: 'https://example.com',
      projects_dir: '/path/to/projects',
      server_config: {
        default_port: 8000,
        default_host: '127.0.0.1',
        auto_reload: true,
        document_root: '/path/to/root'
      }
    }
    
    expect(config.php_versions).toEqual(['8.3.0'])
    expect(config.default_php_version).toBe('8.3.0')
    expect(config.download_base_url).toBe('https://example.com')
    expect(config.projects_dir).toBe('/path/to/projects')
    expect(config.server_config.default_port).toBe(8000)
    expect(config.server_config.default_host).toBe('127.0.0.1')
    expect(config.server_config.auto_reload).toBe(true)
    expect(config.server_config.document_root).toBe('/path/to/root')
  })
  
  it('should have correct StartServerRequest type structure', () => {
    const request: StartServerRequest = {
      project_path: '/path/to/project',
      port: 8000,
      host: '127.0.0.1',
      php_version: '8.3.0',
      document_root: '/path/to/root'
    }
    
    expect(request.project_path).toBe('/path/to/project')
    expect(request.port).toBe(8000)
    expect(request.host).toBe('127.0.0.1')
    expect(request.php_version).toBe('8.3.0')
    expect(request.document_root).toBe('/path/to/root')
  })
  
  it('should have correct CloneProjectRequest type structure', () => {
    const request: CloneProjectRequest = {
      git_url: 'https://github.com/user/repo.git',
      destination: '/path/to/destination',
      branch: 'main',
      auto_setup: true
    }
    
    expect(request.git_url).toBe('https://github.com/user/repo.git')
    expect(request.destination).toBe('/path/to/destination')
    expect(request.branch).toBe('main')
    expect(request.auto_setup).toBe(true)
  })
  
  it('should have correct ProjectInfo type structure', () => {
    const projectInfo: ProjectInfo = {
      name: 'test-project',
      path: '/path/to/project',
      framework: Framework.Laravel,
      git_url: 'https://github.com/user/repo.git',
      php_version: '8.3.0',
      entry_point: 'public/index.php'
    }
    
    expect(projectInfo.name).toBe('test-project')
    expect(projectInfo.path).toBe('/path/to/project')
    expect(projectInfo.framework).toBe(Framework.Laravel)
    expect(projectInfo.git_url).toBe('https://github.com/user/repo.git')
    expect(projectInfo.php_version).toBe('8.3.0')
    expect(projectInfo.entry_point).toBe('public/index.php')
  })
  
  it('should have correct ServerStatus type structure', () => {
    const status: ServerStatus = {
      is_running: true,
      pid: 12345,
      port: 8000,
      host: '127.0.0.1',
      document_root: '/path/to/root',
      started_at: '2024-01-01T00:00:00Z'
    }
    
    expect(status.is_running).toBe(true)
    expect(status.pid).toBe(12345)
    expect(status.port).toBe(8000)
    expect(status.host).toBe('127.0.0.1')
    expect(status.document_root).toBe('/path/to/root')
    expect(status.started_at).toBe('2024-01-01T00:00:00Z')
  })
  
  it('should have correct PhpBinaryInfo type structure', () => {
    const binaryInfo: PhpBinaryInfo = {
      version: '8.3.0',
      path: '/path/to/php',
      is_downloaded: true,
      download_url: 'https://example.com/php.zip',
      size: 50000000
    }
    
    expect(binaryInfo.version).toBe('8.3.0')
    expect(binaryInfo.path).toBe('/path/to/php')
    expect(binaryInfo.is_downloaded).toBe(true)
    expect(binaryInfo.download_url).toBe('https://example.com/php.zip')
    expect(binaryInfo.size).toBe(50000000)
  })
})

describe('Edge cases and validation', () => {
  it('should handle empty project path', () => {
    const request = createStartServerRequest('')
    expect(request.project_path).toBe('')
  })
  
  it('should handle invalid git URL', () => {
    const request = createCloneProjectRequest('invalid-url')
    expect(request.git_url).toBe('invalid-url')
  })
  
  it('should handle undefined optional fields', () => {
    const config = createDefaultConfig()
    config.projects_dir = undefined
    config.server_config.document_root = undefined
    
    expect(config.projects_dir).toBeUndefined()
    expect(config.server_config.document_root).toBeUndefined()
  })
  
  it('should handle all framework types in utility functions', () => {
    const frameworks = Object.values(Framework)
    
    frameworks.forEach(framework => {
      expect(typeof getDefaultPortForFramework(framework)).toBe('number')
      expect(typeof getDefaultEntryPointForFramework(framework)).toBe('string')
      expect(getDefaultPortForFramework(framework)).toBeGreaterThan(0)
      expect(getDefaultEntryPointForFramework(framework)).toBeTruthy()
    })
  })
})

describe('Framework-specific configurations', () => {
  it('should have correct ThinkPHP configuration', () => {
    expect(getDefaultPortForFramework(Framework.ThinkPHP)).toBe(8080)
    expect(getDefaultEntryPointForFramework(Framework.ThinkPHP)).toBe('public/index.php')
  })
  
  it('should distinguish between similar frameworks', () => {
    // Laravel vs Lumen
    expect(getDefaultPortForFramework(Framework.Laravel)).toBe(8000)
    expect(getDefaultPortForFramework(Framework.Lumen)).toBe(8000)
    expect(getDefaultEntryPointForFramework(Framework.Laravel)).toBe('public/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Lumen)).toBe('public/index.php')
    
    // Different entry points
    expect(getDefaultEntryPointForFramework(Framework.CakePHP)).toBe('webroot/index.php')
    expect(getDefaultEntryPointForFramework(Framework.Yii)).toBe('web/index.php')
    expect(getDefaultEntryPointForFramework(Framework.CodeIgniter)).toBe('index.php')
  })
})

describe('Configuration validation', () => {
  it('should validate PHP version format', () => {
    const validVersions = ['8.3.0', '8.2.15', '8.1.27', '7.4.33']
    const versionRegex = /^\d+\.\d+\.\d+$/
    
    validVersions.forEach(version => {
      expect(versionRegex.test(version)).toBe(true)
    })
  })
  
  it('should validate port ranges', () => {
    const validPorts = [80, 443, 8000, 8080, 8765, 65535]
    
    validPorts.forEach(port => {
      expect(port).toBeGreaterThan(0)
      expect(port).toBeLessThanOrEqual(65535)
    })
  })
  
  it('should validate host formats', () => {
    const validHosts = ['127.0.0.1', 'localhost', '0.0.0.0', '::1']
    
    validHosts.forEach(host => {
      expect(typeof host).toBe('string')
      expect(host.length).toBeGreaterThan(0)
    })
  })
})
