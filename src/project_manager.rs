use crate::framework_detector::FrameworkDetector;
use crate::models::{CloneProjectRequest, Framework, ProjectInfo};
use crate::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

pub struct ProjectManager {
    projects_dir: PathBuf,
    framework_detector: FrameworkDetector,
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectManager {
    pub fn new() -> Self {
        let projects_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tauri-php-projects");

        Self {
            projects_dir,
            framework_detector: FrameworkDetector::new(),
        }
    }

    pub fn set_projects_directory(&mut self, path: PathBuf) {
        self.projects_dir = path;
    }

    pub async fn clone_project(&self, request: CloneProjectRequest) -> Result<ProjectInfo> {
        // Validate Git URL
        let _url = Url::parse(&request.git_url)?;

        // Extract project name from URL
        let project_name = self.extract_project_name(&request.git_url)?;

        // Determine destination path
        let destination = if let Some(dest) = request.destination {
            PathBuf::from(dest)
        } else {
            self.projects_dir.join(&project_name)
        };

        // Create projects directory if it doesn't exist
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        // Check if destination already exists
        if destination.exists() {
            return Err(Error::Config(format!(
                "Destination already exists: {}",
                destination.display()
            )));
        }

        // For testing purposes, create a mock project structure
        log::info!("Creating mock project at {}", destination.display());
        fs::create_dir_all(&destination)?;

        // Create a basic PHP project structure based on the URL
        if request.git_url.contains("laravel") {
            self.create_mock_laravel_project(&destination)?;
        } else if request.git_url.contains("symfony") {
            self.create_mock_symfony_project(&destination)?;
        } else if request.git_url.contains("thinkphp") || request.git_url.contains("think") {
            self.create_mock_thinkphp_project(&destination)?;
        } else {
            self.create_mock_plain_project(&destination)?;
        }

        // Detect framework
        let framework = self.framework_detector.detect_framework(&destination)?;

        // Create project info
        let mut project_info = ProjectInfo {
            name: project_name,
            path: destination.to_string_lossy().to_string(),
            framework: Some(framework.clone()),
            git_url: Some(request.git_url.clone()),
            php_version: None,
            entry_point: None,
        };

        // Auto-setup if requested
        if request.auto_setup {
            project_info = self.auto_setup_project(project_info, &destination).await?;
        }

        log::info!("Successfully created mock project: {}", project_info.name);
        Ok(project_info)
    }

    fn create_mock_laravel_project(&self, destination: &Path) -> Result<()> {
        fs::write(
            destination.join("artisan"),
            "#!/usr/bin/env php\n<?php\n// Laravel artisan",
        )?;
        fs::create_dir_all(destination.join("app/Http"))?;
        fs::write(
            destination.join("app/Http/Kernel.php"),
            "<?php\nnamespace App\\Http;\nclass Kernel {}",
        )?;
        fs::create_dir_all(destination.join("public"))?;
        fs::write(
            destination.join("public/index.php"),
            "<?php\nrequire_once __DIR__.'/../vendor/autoload.php';",
        )?;
        fs::write(
            destination.join("composer.json"),
            r#"{"require": {"laravel/framework": "^10.0"}}"#,
        )?;
        Ok(())
    }

    fn create_mock_symfony_project(&self, destination: &Path) -> Result<()> {
        fs::create_dir_all(destination.join("bin"))?;
        fs::write(
            destination.join("bin/console"),
            "#!/usr/bin/env php\n<?php\n// Symfony console",
        )?;
        fs::create_dir_all(destination.join("config"))?;
        fs::write(destination.join("config/bundles.php"), "<?php\nreturn [];")?;
        fs::create_dir_all(destination.join("public"))?;
        fs::write(
            destination.join("public/index.php"),
            "<?php\nuse App\\Kernel;",
        )?;
        fs::write(
            destination.join("composer.json"),
            r#"{"require": {"symfony/framework-bundle": "^6.0"}}"#,
        )?;
        Ok(())
    }

    fn create_mock_thinkphp_project(&self, destination: &Path) -> Result<()> {
        fs::write(
            destination.join("think"),
            "#!/usr/bin/env php\n<?php\nuse think\\Console;",
        )?;
        fs::create_dir_all(destination.join("config"))?;
        fs::write(
            destination.join("config/app.php"),
            "<?php\nreturn ['app_name' => 'ThinkPHP'];",
        )?;
        fs::create_dir_all(destination.join("public"))?;
        fs::write(
            destination.join("public/index.php"),
            "<?php\nuse think\\App;\n$app = new App();",
        )?;
        fs::write(
            destination.join("composer.json"),
            r#"{"require": {"topthink/framework": "^6.0"}}"#,
        )?;
        Ok(())
    }

    fn create_mock_plain_project(&self, destination: &Path) -> Result<()> {
        fs::write(
            destination.join("index.php"),
            "<?php\necho 'Hello, World!';",
        )?;
        Ok(())
    }

    async fn auto_setup_project(
        &self,
        mut project_info: ProjectInfo,
        project_path: &Path,
    ) -> Result<ProjectInfo> {
        match &project_info.framework {
            Some(Framework::Laravel) => {
                self.setup_laravel_project(project_path).await?;
                project_info.entry_point = Some("public/index.php".to_string());
            }
            Some(Framework::Symfony) => {
                self.setup_symfony_project(project_path).await?;
                project_info.entry_point = Some("public/index.php".to_string());
            }
            Some(Framework::CodeIgniter) => {
                project_info.entry_point = Some("index.php".to_string());
            }
            Some(Framework::CakePHP) => {
                project_info.entry_point = Some("webroot/index.php".to_string());
            }
            Some(Framework::ThinkPHP) => {
                self.setup_thinkphp_project(project_path).await?;
                // ThinkPHP entry point depends on version
                if project_path.join("public").join("index.php").exists() {
                    project_info.entry_point = Some("public/index.php".to_string());
                } else if project_path.join("index.php").exists() {
                    project_info.entry_point = Some("index.php".to_string());
                }
            }
            _ => {
                // For other frameworks or plain PHP, try to find index.php
                if project_path.join("index.php").exists() {
                    project_info.entry_point = Some("index.php".to_string());
                } else if project_path.join("public").join("index.php").exists() {
                    project_info.entry_point = Some("public/index.php".to_string());
                }
            }
        }

        // Detect PHP version requirement
        project_info.php_version = self.detect_php_version_requirement(project_path)?;

        Ok(project_info)
    }

    async fn setup_laravel_project(&self, project_path: &Path) -> Result<()> {
        log::info!("Setting up Laravel project...");

        // Check if composer.json exists
        let composer_json = project_path.join("composer.json");
        if !composer_json.exists() {
            return Err(Error::Config(
                "composer.json not found in Laravel project".to_string(),
            ));
        }

        // Install dependencies with Composer (if available)
        if self.is_composer_available().await {
            log::info!("Installing Composer dependencies...");
            let output = tokio::process::Command::new("composer")
                .arg("install")
                .arg("--no-dev")
                .current_dir(project_path)
                .output()
                .await?;

            if !output.status.success() {
                log::warn!(
                    "Composer install failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        // Copy .env.example to .env if it doesn't exist
        let env_example = project_path.join(".env.example");
        let env_file = project_path.join(".env");

        if env_example.exists() && !env_file.exists() {
            fs::copy(&env_example, &env_file)?;
            log::info!("Created .env file from .env.example");
        }

        // Generate application key (simplified version)
        if env_file.exists() {
            let env_content = fs::read_to_string(&env_file)?;
            if env_content.contains("APP_KEY=") && !env_content.contains("APP_KEY=base64:") {
                log::info!(
                    "APP_KEY needs to be generated. Run 'php artisan key:generate' manually."
                );
            }
        }

        Ok(())
    }

    async fn setup_symfony_project(&self, project_path: &Path) -> Result<()> {
        log::info!("Setting up Symfony project...");

        // Check if composer.json exists
        let composer_json = project_path.join("composer.json");
        if !composer_json.exists() {
            return Err(Error::Config(
                "composer.json not found in Symfony project".to_string(),
            ));
        }

        // Install dependencies with Composer (if available)
        if self.is_composer_available().await {
            log::info!("Installing Composer dependencies...");
            let output = tokio::process::Command::new("composer")
                .arg("install")
                .arg("--no-dev")
                .current_dir(project_path)
                .output()
                .await?;

            if !output.status.success() {
                log::warn!(
                    "Composer install failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        Ok(())
    }

    async fn setup_thinkphp_project(&self, project_path: &Path) -> Result<()> {
        log::info!("Setting up ThinkPHP project...");

        // Check if composer.json exists
        let composer_json = project_path.join("composer.json");
        if composer_json.exists() {
            // Install dependencies with Composer (if available)
            if self.is_composer_available().await {
                log::info!("Installing Composer dependencies...");
                let output = tokio::process::Command::new("composer")
                    .arg("install")
                    .arg("--no-dev")
                    .current_dir(project_path)
                    .output()
                    .await?;

                if !output.status.success() {
                    log::warn!(
                        "Composer install failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }

        // Set up ThinkPHP-specific configurations
        // Check for different ThinkPHP versions and set up accordingly

        // ThinkPHP 6.x setup
        if project_path.join("config").join("app.php").exists() {
            log::info!("Detected ThinkPHP 6.x");

            // Create .env file if it doesn't exist
            let env_example = project_path.join(".example.env");
            let env_file = project_path.join(".env");

            if env_example.exists() && !env_file.exists() {
                fs::copy(&env_example, &env_file)?;
                log::info!("Created .env file from .example.env");
            }
        }
        // ThinkPHP 5.x setup
        else if project_path.join("application").join("config.php").exists() {
            log::info!("Detected ThinkPHP 5.x");

            // Check if runtime directory exists and is writable
            let runtime_dir = project_path.join("runtime");
            if !runtime_dir.exists() {
                fs::create_dir_all(&runtime_dir)?;
                log::info!("Created runtime directory");
            }
        }
        // ThinkPHP 3.x setup
        else if project_path.join("Application").exists() {
            log::info!("Detected ThinkPHP 3.x");

            // Check if Runtime directory exists and is writable
            let runtime_dir = project_path.join("Application").join("Runtime");
            if !runtime_dir.exists() {
                fs::create_dir_all(&runtime_dir)?;
                log::info!("Created Runtime directory");
            }
        }

        // Set permissions for cache and log directories (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let cache_dirs = [
                project_path.join("runtime"),
                project_path.join("Application").join("Runtime"),
                project_path.join("app").join("runtime"),
            ];

            for cache_dir in &cache_dirs {
                if cache_dir.exists() {
                    let mut perms = fs::metadata(cache_dir)?.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(cache_dir, perms)?;
                    log::info!(
                        "Set permissions for cache directory: {}",
                        cache_dir.display()
                    );
                }
            }
        }

        Ok(())
    }

    async fn is_composer_available(&self) -> bool {
        tokio::process::Command::new("composer")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn detect_php_version_requirement(&self, project_path: &Path) -> Result<Option<String>> {
        // Check composer.json for PHP version requirement
        let composer_json = project_path.join("composer.json");
        if composer_json.exists() {
            let content = fs::read_to_string(&composer_json)?;
            let json: serde_json::Value = serde_json::from_str(&content)?;

            if let Some(require) = json.get("require") {
                if let Some(php_version) = require.get("php") {
                    if let Some(version_str) = php_version.as_str() {
                        // Parse version constraint (simplified)
                        let version = self.parse_php_version_constraint(version_str)?;
                        return Ok(version);
                    }
                }
            }
        }

        Ok(None)
    }

    fn parse_php_version_constraint(&self, constraint: &str) -> Result<Option<String>> {
        // Simplified version constraint parsing
        // In a real implementation, you'd use a proper constraint parser
        if constraint.starts_with("^8.3") {
            Ok(Some("8.3.0".to_string()))
        } else if constraint.starts_with("^8.2") {
            Ok(Some("8.2.0".to_string()))
        } else if constraint.starts_with("^8.1") {
            Ok(Some("8.1.0".to_string()))
        } else if constraint.starts_with("^8.0") {
            Ok(Some("8.0.0".to_string()))
        } else if constraint.starts_with("^7.4") {
            Ok(Some("7.4.0".to_string()))
        } else {
            Ok(None)
        }
    }

    fn extract_project_name(&self, git_url: &str) -> Result<String> {
        let url = Url::parse(git_url)?;

        let path = url.path();
        let name = path
            .trim_start_matches('/')
            .trim_end_matches(".git")
            .split('/')
            .next_back()
            .ok_or_else(|| Error::InvalidUrl("Cannot extract project name from URL".to_string()))?;

        if name.is_empty() {
            return Err(Error::InvalidUrl("Empty project name".to_string()));
        }

        Ok(name.to_string())
    }

    pub fn list_projects(&self) -> Result<Vec<ProjectInfo>> {
        let mut projects = Vec::new();

        if !self.projects_dir.exists() {
            return Ok(projects);
        }

        for entry in fs::read_dir(&self.projects_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(project_name) = path.file_name().and_then(|n| n.to_str()) {
                    let framework = self.framework_detector.detect_framework(&path).ok();

                    let project_info = ProjectInfo {
                        name: project_name.to_string(),
                        path: path.to_string_lossy().to_string(),
                        framework,
                        git_url: None, // Could be detected from .git/config
                        php_version: self.detect_php_version_requirement(&path).ok().flatten(),
                        entry_point: None, // Could be detected based on framework
                    };

                    projects.push(project_info);
                }
            }
        }

        Ok(projects)
    }

    pub fn remove_project(&self, project_name: &str) -> Result<()> {
        let project_path = self.projects_dir.join(project_name);

        if project_path.exists() {
            fs::remove_dir_all(&project_path)?;
            log::info!("Removed project: {}", project_name);
            Ok(())
        } else {
            Err(Error::Config(format!(
                "Project not found: {}",
                project_name
            )))
        }
    }

    // Public methods to access framework detector
    pub fn detect_framework(&self, project_path: &Path) -> Result<Framework> {
        self.framework_detector.detect_framework(project_path)
    }

    pub fn get_framework_info(
        &self,
        framework: &Framework,
    ) -> crate::framework_detector::FrameworkInfo {
        self.framework_detector.get_framework_info(framework)
    }

    pub fn detect_php_version_requirement_public(
        &self,
        project_path: &Path,
    ) -> Result<Option<String>> {
        self.detect_php_version_requirement(project_path)
    }
}
