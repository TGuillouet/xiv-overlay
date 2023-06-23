use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LayoutConfig {
    name: String,
    url: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    clickthrough: bool,
    decorated: bool
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
}
