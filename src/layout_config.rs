use serde::{Serialize, Deserialize};

use crate::app_config::AppConfig;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    pub fn from_file(file_path: impl Into<String>) -> Result<LayoutConfig, serde_yaml::Error> {
        let file_content = std::fs::read_to_string(&file_path.into());
        
        match file_content {
            Ok(content) => serde_yaml::from_str(content.as_str()),
            Err(_) => panic!("Could not parse the configuration")
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
    
    pub fn width(&self) -> i32 {
        self.width
    }
    
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn is_decoraded(&self) -> bool {
        self.decorated
    }

    pub fn is_clickthrough(&self) -> bool {
        self.clickthrough
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

pub fn load_layouts(config: &AppConfig) -> Vec<LayoutConfig> {
    let layout_config_path = config.layouts_config_path();
    let mut layout_configs: Vec<LayoutConfig> = Vec::new();
    let read_result = std::fs::read_dir(layout_config_path).expect("Could not read the layouts path");
    for file_result in read_result {
        match file_result {
            Ok(file) => {
                if let Ok(config) = LayoutConfig::from_file(file.path().to_str().unwrap().to_string()) {
                    layout_configs.push(
                        config
                    );
                }
            },
            Err(_) => {},
        }
    }
    layout_configs
}