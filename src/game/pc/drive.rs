use std::str::FromStr;
use super::pc_rect::PCRect;

pub struct Drive {

}

#[derive(Debug, Clone, Copy)]
pub enum DriveType {
    HDD,
    SSD,
    M2
}

impl DriveType {
    pub const TYPE_COUNT: u8 = 3;
    
    /// Length and width
    pub const HDD_SIZE: glm::UVec2 = glm::UVec2::new(145, 100);
    /// Width and thickness
    pub const HDD_SIZE_FLAT: glm::UVec2 = glm::UVec2::new(100, 20);
    /// Length and width
    pub const SSD_SIZE: glm::UVec2 = glm::UVec2::new(100, 70);
    /// Width and thickness
    pub const SSD_SIZE_FLAT: glm::UVec2 = glm::UVec2::new(70, 10);
}

impl FromStr for DriveType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hdd" => Ok(DriveType::HDD),
            "ssd" => Ok(DriveType::SSD),
            "m.2" => Ok(DriveType::M2),
            _ => Err("Invalid drive type".to_string())
        }
    }
}

impl ToString for DriveType {
    fn to_string(&self) -> String {
        match self {
            DriveType::HDD => "HDD".to_string(),
            DriveType::SSD => "SSD".to_string(),
            DriveType::M2 => "M.2".to_string()
        }
    }
}
