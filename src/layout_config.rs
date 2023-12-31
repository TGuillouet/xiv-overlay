use serde::{Serialize, Deserialize};

use crate::{app_config::AppConfig, errors::OverlayConfigParseError};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LayoutConfig {
    name: String,
    url: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    clickthrough: bool,
    decorated: bool,
    active: bool
}

impl LayoutConfig {
    pub fn from_file(file_path: impl Into<String>) -> Result<LayoutConfig, OverlayConfigParseError> {
        let path: &String = &file_path.into();
        let file_content = std::fs::read_to_string(path);
        
        match file_content {
            Ok(content) => {
                let path_cloned = path.clone();
                #[allow(clippy::bind_instead_of_map)]
                serde_yaml::from_str(content.as_str()).or_else(move |_| Err(OverlayConfigParseError(path_cloned)))
            },
            Err(error) => {
                let path_cloned = path.clone();
                error!("{}", format!("The configuration file {} cound not be opened !", path_cloned));
                error!("IO Error: {}", error);
                Err(OverlayConfigParseError(path_cloned))
            }
        }
    }
}

impl From<LayoutConfig> for String {
    fn from(value: LayoutConfig) -> String {
        serde_yaml::to_string(&value).expect("Could not transform the overlay to yaml")
    }
}

impl LayoutConfig {

    pub fn get_file_name(&self) -> String {
        format!("{}.yaml", self.name.replace(' ', "-").to_lowercase())
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
    
    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = url.into()
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x
    }
    
    pub fn y(&self) -> i32 {
        self.y
    }
    
    pub fn set_y(&mut self, y: i32) {
        self.y = y
    } 
    
    pub fn width(&self) -> i32 {
        self.width
    }
    
    pub fn set_width(&mut self, width: i32) {
        self.width = width
    } 
    
    pub fn height(&self) -> i32 {
        self.height
    }
    
    pub fn set_height(&mut self, height: i32) {
        self.height = height
    }
    
    pub fn is_decoraded(&self) -> bool {
        self.decorated
    }
    
    pub fn set_is_decorated(&mut self, is_decorated: bool) {
        self.decorated = is_decorated
    }
    
    pub fn is_clickthrough(&self) -> bool {
        self.clickthrough
    }
    
    pub fn set_is_clickthrough(&mut self, is_clickthrough: bool) {
        self.clickthrough = is_clickthrough
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.active = is_active
    }
}

pub fn load_layouts() -> Vec<LayoutConfig> {
    let app_config = AppConfig::default();
    let layout_config_path = app_config.layouts_config_path();
    let mut layout_configs: Vec<LayoutConfig> = Vec::new();

    let read_result = std::fs::read_dir(layout_config_path).expect("Could not read the layouts path");
    for file in read_result.flatten() {
        if let Ok(config) = LayoutConfig::from_file(file.path().to_str().unwrap().to_string()) {
            layout_configs.push(
                config
            );
        }
    }

    layout_configs
}

pub fn save_overlay(overlay: LayoutConfig) -> Result<(), std::io::Error> {
    let app_config = AppConfig::default();
    
    let overlay_path = app_config
        .layouts_config_path()
        .to_path_buf()
        .join(overlay.get_file_name());
    
    let overlay_str: String = overlay.into();
    std::fs::write(overlay_path, overlay_str.as_bytes())
}

pub fn remove_overlay_file(overlay_file_name: String) -> Result<(), std::io::Error> {
    let app_config = AppConfig::default();
    
    let overlay_path = app_config
        .layouts_config_path()
        .to_path_buf()
        .join(overlay_file_name);

    std::fs::remove_file(overlay_path)
}

pub fn get_layout_by_name(overlay_name: &str) -> Result<LayoutConfig, String> {
    let overlays = load_layouts();
    for overlay in overlays.iter() {
        if overlay.name() == *overlay_name {
            return Ok(overlay.clone())
        }
    }

    Err("Could not find the overlay !".to_owned())
}