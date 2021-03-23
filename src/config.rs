use crate::log::LogLevel;

pub struct FarmConfig {
    pub ruby_build_mirror: reqwest::Url,
    pub log_level: LogLevel,
    pub farm_path: Option<String>,
}

impl Default for FarmConfig {
    fn default() -> Self {
        Self {
            ruby_build_mirror: reqwest::Url::parse("https://cache.ruby-lang.org/pub/ruby/")
                .unwrap(),
            log_level: LogLevel::Info,
            farm_path: std::env::var("FARM_MULTISHELL_PATH").ok(),
        }
    }
}

impl FarmConfig {
    pub fn base_dir(&self) -> std::path::PathBuf {
        // TODO: support base directory
        ensure_dir_exists(
            std::env::current_dir()
                .expect("Can't get current directory")
                .join(".farm"),
        )
    }

    pub fn versions_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists(self.base_dir().join("versions"))
    }

    pub fn default_version_dir(&self) -> std::path::PathBuf {
        self.aliases_dir().join("default")
    }

    pub fn aliases_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists(self.base_dir().join("aliases"))
    }
}

fn ensure_dir_exists<T: AsRef<std::path::Path>>(path: T) -> T {
    std::fs::create_dir_all(path.as_ref()).ok();
    path
}