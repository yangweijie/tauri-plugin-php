use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub php_versions: Vec<String>,
    pub default_php_version: Option<String>,
    pub download_base_url: String,
    pub projects_dir: Option<String>,
    pub server_config: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            php_versions: vec![
                "8.3.0".to_string(),
                "8.2.0".to_string(),
                "8.1.0".to_string(),
            ],
            default_php_version: Some("8.3.0".to_string()),
            download_base_url: "https://github.com/NativePHP/php-bin/releases/download".to_string(),
            projects_dir: None,
            server_config: ServerConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub default_port: u16,
    pub default_host: String,
    pub auto_reload: bool,
    pub document_root: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            default_port: 8000,
            default_host: "127.0.0.1".to_string(),
            auto_reload: true,
            document_root: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhpBinaryInfo {
    pub version: String,
    pub path: String,
    pub is_downloaded: bool,
    pub download_url: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub is_running: bool,
    pub pid: Option<u32>,
    pub port: Option<u16>,
    pub host: Option<String>,
    pub document_root: Option<String>,
    pub started_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
    pub framework: Option<Framework>,
    pub git_url: Option<String>,
    pub php_version: Option<String>,
    pub entry_point: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Framework {
    Laravel,
    Symfony,
    CodeIgniter,
    CakePHP,
    Zend,
    Yii,
    Phalcon,
    Slim,
    Lumen,
    ThinkPHP,
    Plain,
    Unknown,
}

impl Framework {
    pub fn as_str(&self) -> &'static str {
        match self {
            Framework::Laravel => "Laravel",
            Framework::Symfony => "Symfony",
            Framework::CodeIgniter => "CodeIgniter",
            Framework::CakePHP => "CakePHP",
            Framework::Zend => "Zend",
            Framework::Yii => "Yii",
            Framework::Phalcon => "Phalcon",
            Framework::Slim => "Slim",
            Framework::Lumen => "Lumen",
            Framework::ThinkPHP => "ThinkPHP",
            Framework::Plain => "Plain PHP",
            Framework::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartServerRequest {
    pub project_path: String,
    pub port: Option<u16>,
    pub host: Option<String>,
    pub php_version: Option<String>,
    pub document_root: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneProjectRequest {
    pub git_url: String,
    pub destination: Option<String>,
    pub branch: Option<String>,
    pub auto_setup: bool,
}
