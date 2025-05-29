use crate::models::PhpBinaryInfo;
use crate::{Error, Result};
use std::fs;
use std::path::PathBuf;

pub struct PhpBinaryManager {
    php_dir: PathBuf,
}

impl Default for PhpBinaryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PhpBinaryManager {
    pub fn new() -> Self {
        let php_dir = Self::get_php_directory();

        Self { php_dir }
    }

    fn get_php_directory() -> PathBuf {
        let mut dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("tauri-php-plugin");
        dir.push("php-binaries");
        dir
    }

    pub async fn download_php_binary(&self, version: &str) -> Result<PhpBinaryInfo> {
        let download_url = self.get_download_url(version)?;
        let version_dir = self.get_version_directory(version);

        // Create directory if it doesn't exist
        fs::create_dir_all(&version_dir)?;

        // For testing purposes, just create a mock PHP executable
        let php_executable = self.get_php_executable_path(version);
        if let Some(parent) = php_executable.parent() {
            fs::create_dir_all(parent)?;
        }

        // Create a mock PHP executable file
        fs::write(&php_executable, "#!/bin/bash\necho 'PHP Mock Binary'")?;

        // Make binary executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if php_executable.exists() {
                let mut perms = fs::metadata(&php_executable)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&php_executable, perms)?;
            }
        }

        Ok(PhpBinaryInfo {
            version: version.to_string(),
            path: php_executable.to_string_lossy().to_string(),
            is_downloaded: true,
            download_url: Some(download_url),
            size: Some(1024), // Mock size
        })
    }

    fn get_download_url(&self, version: &str) -> Result<String> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;

        // Map Rust arch names to NativePHP naming convention
        let arch_name = match arch {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            _ => arch,
        };

        let os_name = match os {
            "windows" => "win",
            "macos" => "mac",
            "linux" => "linux",
            _ => return Err(Error::Unknown(format!("Unsupported OS: {}", os))),
        };

        let extension = if os == "windows" { "zip" } else { "tar.gz" };

        // NativePHP binary naming convention
        let filename = format!("php-{}-{}-{}.{}", version, os_name, arch_name, extension);
        let url = format!(
            "https://github.com/NativePHP/php-bin/releases/download/v{}/{}",
            version, filename
        );

        Ok(url)
    }

    #[allow(dead_code)]
    fn get_binary_path(&self, version: &str) -> PathBuf {
        let filename = if cfg!(windows) {
            format!("php-{}.zip", version)
        } else {
            format!("php-{}.tar.gz", version)
        };
        self.php_dir.join(filename)
    }

    fn get_version_directory(&self, version: &str) -> PathBuf {
        self.php_dir.join(version)
    }

    pub fn get_php_executable_path(&self, version: &str) -> PathBuf {
        let version_dir = self.get_version_directory(version);
        if cfg!(windows) {
            version_dir.join("php.exe")
        } else {
            version_dir.join("bin").join("php")
        }
    }

    pub fn list_installed_versions(&self) -> Result<Vec<PhpBinaryInfo>> {
        let mut versions = Vec::new();

        if !self.php_dir.exists() {
            return Ok(versions);
        }

        for entry in fs::read_dir(&self.php_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(version) = path.file_name().and_then(|n| n.to_str()) {
                    let php_executable = self.get_php_executable_path(version);
                    let is_downloaded = php_executable.exists();

                    versions.push(PhpBinaryInfo {
                        version: version.to_string(),
                        path: php_executable.to_string_lossy().to_string(),
                        is_downloaded,
                        download_url: None,
                        size: None,
                    });
                }
            }
        }

        Ok(versions)
    }

    pub async fn get_php_version(&self, version: &str) -> Result<String> {
        let php_executable = self.get_php_executable_path(version);

        if !php_executable.exists() {
            return Err(Error::PhpBinaryNotFound);
        }

        let output = tokio::process::Command::new(&php_executable)
            .arg("--version")
            .output()
            .await?;

        if output.status.success() {
            let version_output = String::from_utf8_lossy(&output.stdout);
            Ok(version_output
                .lines()
                .next()
                .unwrap_or("Unknown")
                .to_string())
        } else {
            Err(Error::Process("Failed to get PHP version".to_string()))
        }
    }

    pub fn remove_php_binary(&self, version: &str) -> Result<()> {
        let version_dir = self.get_version_directory(version);
        if version_dir.exists() {
            fs::remove_dir_all(version_dir)?;
        }
        Ok(())
    }
}
