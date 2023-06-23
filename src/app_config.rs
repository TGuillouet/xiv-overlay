pub struct AppConfig {
    layouts_config_path: String
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig { layouts_config_path: "~/.config/xiv-overlay/".to_string() }
    }
}

impl AppConfig {
    pub fn layouts_config_path(&self) -> String {
        self.layouts_config_path.clone()
    }
}