use crate::models::Framework;
use crate::{Error, Result};
use std::fs;
use std::path::Path;

pub struct FrameworkDetector;

impl Default for FrameworkDetector {
    fn default() -> Self {
        FrameworkDetector
    }
}

impl FrameworkDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_framework(&self, project_path: &Path) -> Result<Framework> {
        // Check for Laravel
        if self.is_laravel_project(project_path) {
            return Ok(Framework::Laravel);
        }

        // Check for Symfony
        if self.is_symfony_project(project_path) {
            return Ok(Framework::Symfony);
        }

        // Check for CodeIgniter
        if self.is_codeigniter_project(project_path) {
            return Ok(Framework::CodeIgniter);
        }

        // Check for CakePHP
        if self.is_cakephp_project(project_path) {
            return Ok(Framework::CakePHP);
        }

        // Check for Zend/Laminas
        if self.is_zend_project(project_path) {
            return Ok(Framework::Zend);
        }

        // Check for Yii
        if self.is_yii_project(project_path) {
            return Ok(Framework::Yii);
        }

        // Check for ThinkPHP
        if self.is_thinkphp_project(project_path) {
            return Ok(Framework::ThinkPHP);
        }

        // Check for Phalcon
        if self.is_phalcon_project(project_path) {
            return Ok(Framework::Phalcon);
        }

        // Check for Slim
        if self.is_slim_project(project_path) {
            return Ok(Framework::Slim);
        }

        // Check for Lumen
        if self.is_lumen_project(project_path) {
            return Ok(Framework::Lumen);
        }

        // Check for plain PHP
        if self.is_php_project(project_path) {
            return Ok(Framework::Plain);
        }

        Ok(Framework::Unknown)
    }

    fn is_laravel_project(&self, project_path: &Path) -> bool {
        // Check for Laravel-specific files and directories
        let laravel_indicators = [
            "artisan",
            "app/Http/Kernel.php",
            "bootstrap/app.php",
            "config/app.php",
        ];

        for indicator in &laravel_indicators {
            if project_path.join(indicator).exists() {
                return true;
            }
        }

        // Check composer.json for Laravel dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("laravel/framework") {
                return true;
            }
        }

        false
    }

    fn is_symfony_project(&self, project_path: &Path) -> bool {
        // Check for Symfony-specific files
        let symfony_indicators = [
            "bin/console",
            "config/bundles.php",
            "symfony.lock",
            "src/Kernel.php",
        ];

        for indicator in &symfony_indicators {
            if project_path.join(indicator).exists() {
                return true;
            }
        }

        // Check composer.json for Symfony dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("symfony/framework-bundle")
                || composer_content.contains("symfony/symfony")
            {
                return true;
            }
        }

        false
    }

    fn is_codeigniter_project(&self, project_path: &Path) -> bool {
        // Check for CodeIgniter-specific files (must have system directory)
        let ci_indicators = ["system/CodeIgniter.php", "application/config/config.php"];

        for indicator in &ci_indicators {
            if project_path.join(indicator).exists() {
                return true;
            }
        }

        // Check for CodeIgniter 4
        if project_path.join("app").exists() && project_path.join("system").exists() {
            if let Ok(content) = fs::read_to_string(project_path.join("index.php")) {
                if content.contains("CodeIgniter") {
                    return true;
                }
            }
        }

        false
    }

    fn is_cakephp_project(&self, project_path: &Path) -> bool {
        // Check for CakePHP-specific files
        let cake_indicators = [
            "bin/cake",
            "config/app.php",
            "src/Application.php",
            "webroot/index.php",
        ];

        for indicator in &cake_indicators {
            if project_path.join(indicator).exists() {
                return true;
            }
        }

        // Check composer.json for CakePHP dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("cakephp/cakephp") {
                return true;
            }
        }

        false
    }

    fn is_zend_project(&self, project_path: &Path) -> bool {
        // Check for Zend/Laminas-specific files
        let zend_indicators = [
            "config/application.config.php",
            "config/modules.config.php",
            "module/Application",
        ];

        for indicator in &zend_indicators {
            if project_path.join(indicator).exists() {
                return true;
            }
        }

        // Check composer.json for Zend/Laminas dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("zendframework/") || composer_content.contains("laminas/")
            {
                return true;
            }
        }

        false
    }

    fn is_yii_project(&self, project_path: &Path) -> bool {
        // Check for Yii-specific files
        let yii_indicators = [
            "yii",
            "config/web.php",
            "web/index.php",
            "commands/MigrateController.php",
        ];

        for indicator in &yii_indicators {
            if project_path.join(indicator).exists() {
                return true;
            }
        }

        // Check composer.json for Yii dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("yiisoft/yii2") {
                return true;
            }
        }

        false
    }

    fn is_thinkphp_project(&self, project_path: &Path) -> bool {
        // Check for ThinkPHP-specific files
        let thinkphp_indicators = ["think", "public/index.php", "app"];

        for indicator in &thinkphp_indicators {
            if project_path.join(indicator).exists() {
                // Additional check for ThinkPHP specific content
                if *indicator == "think" {
                    if let Ok(content) = fs::read_to_string(project_path.join(indicator)) {
                        if content.contains("think\\Console") || content.contains("think/Console") {
                            return true;
                        }
                    }
                }
                if *indicator == "public/index.php" {
                    if let Ok(content) = fs::read_to_string(project_path.join(indicator)) {
                        if content.contains("think\\App") || content.contains("think/App") {
                            return true;
                        }
                    }
                }
            }
        }

        // Check composer.json for ThinkPHP dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("topthink/framework")
                || composer_content.contains("topthink/think")
            {
                return true;
            }
        }

        // Check for ThinkPHP 5.x structure
        if project_path.join("application").exists() && project_path.join("public").exists() {
            return true;
        }

        // Check for ThinkPHP 3.x structure
        if project_path.join("ThinkPHP").exists() && project_path.join("Application").exists() {
            return true;
        }

        false
    }

    fn is_phalcon_project(&self, project_path: &Path) -> bool {
        // Check for Phalcon-specific files
        let phalcon_indicators = ["app/config/config.php", "public/index.php"];

        for indicator in &phalcon_indicators {
            if project_path.join(indicator).exists() {
                if let Ok(content) = fs::read_to_string(project_path.join(indicator)) {
                    if content.contains("Phalcon") {
                        return true;
                    }
                }
            }
        }

        // Check composer.json for Phalcon dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("phalcon/") {
                return true;
            }
        }

        false
    }

    fn is_slim_project(&self, project_path: &Path) -> bool {
        // Check composer.json for Slim dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("slim/slim") {
                return true;
            }
        }

        // Check for typical Slim structure
        if project_path.join("public").exists() && project_path.join("src").exists() {
            if let Ok(content) = fs::read_to_string(project_path.join("public/index.php")) {
                if content.contains("Slim\\App") || content.contains("Slim/App") {
                    return true;
                }
            }
        }

        false
    }

    fn is_lumen_project(&self, project_path: &Path) -> bool {
        // Check for Lumen-specific files
        let lumen_indicators = ["bootstrap/app.php", "public/index.php"];

        for indicator in &lumen_indicators {
            if project_path.join(indicator).exists() {
                if let Ok(content) = fs::read_to_string(project_path.join(indicator)) {
                    if content.contains("Laravel\\Lumen") {
                        return true;
                    }
                }
            }
        }

        // Check composer.json for Lumen dependency
        if let Ok(composer_content) = self.read_composer_json(project_path) {
            if composer_content.contains("laravel/lumen") {
                return true;
            }
        }

        false
    }

    fn is_php_project(&self, project_path: &Path) -> bool {
        // A project is considered a generic PHP project if it contains any .php file
        // in its root or immediate subdirectories.
        let mut has_php_file = false;
        if let Ok(entries) = fs::read_dir(project_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "php") {
                    has_php_file = true;
                    break;
                }
            }
        }

        has_php_file
    }

    fn read_composer_json(&self, project_path: &Path) -> Result<String> {
        let composer_path = project_path.join("composer.json");
        fs::read_to_string(composer_path).map_err(|e| Error::Io(e.to_string()))
    }

    pub fn get_framework_info(&self, framework: &Framework) -> FrameworkInfo {
        match framework {
            Framework::Laravel => FrameworkInfo {
                name: "Laravel",
                default_entry_point: "public/index.php",
                requires_composer: true,
                setup_commands: vec![
                    "composer install",
                    "cp .env.example .env",
                    "php artisan key:generate",
                ],
                default_port: 8000,
            },
            Framework::Symfony => FrameworkInfo {
                name: "Symfony",
                default_entry_point: "public/index.php",
                requires_composer: true,
                setup_commands: vec!["composer install"],
                default_port: 8000,
            },
            Framework::CodeIgniter => FrameworkInfo {
                name: "CodeIgniter",
                default_entry_point: "index.php",
                requires_composer: false,
                setup_commands: vec![],
                default_port: 8080,
            },
            Framework::CakePHP => FrameworkInfo {
                name: "CakePHP",
                default_entry_point: "webroot/index.php",
                requires_composer: true,
                setup_commands: vec!["composer install"],
                default_port: 8765,
            },
            Framework::Zend => FrameworkInfo {
                name: "Zend/Laminas",
                default_entry_point: "public/index.php",
                requires_composer: true,
                setup_commands: vec!["composer install"],
                default_port: 8080,
            },
            Framework::Yii => FrameworkInfo {
                name: "Yii",
                default_entry_point: "web/index.php",
                requires_composer: true,
                setup_commands: vec!["composer install"],
                default_port: 8080,
            },
            Framework::Phalcon => FrameworkInfo {
                name: "Phalcon",
                default_entry_point: "public/index.php",
                requires_composer: false,
                setup_commands: vec![],
                default_port: 8080,
            },
            Framework::Slim => FrameworkInfo {
                name: "Slim",
                default_entry_point: "public/index.php",
                requires_composer: true,
                setup_commands: vec!["composer install"],
                default_port: 8080,
            },
            Framework::Lumen => FrameworkInfo {
                name: "Lumen",
                default_entry_point: "public/index.php",
                requires_composer: true,
                setup_commands: vec!["composer install"],
                default_port: 8000,
            },
            Framework::ThinkPHP => FrameworkInfo {
                name: "ThinkPHP",
                default_entry_point: "public/index.php",
                requires_composer: true,
                setup_commands: vec!["composer install"],
                default_port: 8000,
            },
            Framework::Plain => FrameworkInfo {
                name: "Plain PHP",
                default_entry_point: "index.php",
                requires_composer: false,
                setup_commands: vec![],
                default_port: 8000,
            },
            Framework::Unknown => FrameworkInfo {
                name: "Unknown",
                default_entry_point: "index.php",
                requires_composer: false,
                setup_commands: vec![],
                default_port: 8000,
            },
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrameworkInfo {
    pub name: &'static str,
    pub default_entry_point: &'static str,
    pub requires_composer: bool,
    pub setup_commands: Vec<&'static str>,
    pub default_port: u16,
}
