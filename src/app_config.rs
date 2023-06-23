use std::path::Path;

pub struct AppConfig {
    layouts_config_path: String
}

impl Default for AppConfig {
    fn default() -> Self {
        let home_dir = std::env::var("HOME").unwrap_or("./".to_string());
        AppConfig { layouts_config_path: format!("{}/.config/xiv-overlay/", home_dir) }
    }
}

impl AppConfig {
    pub fn layouts_config_path(&self) -> &Path {
        Path::new(&self.layouts_config_path)
    }
}