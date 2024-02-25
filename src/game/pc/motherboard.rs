use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum MotherboardSize {
    ATX,
    MicroATX
}

impl MotherboardSize {
    pub const TYPE_COUNT: u8 = 2;

    pub const ATX_SIZE: glm::UVec2 = glm::UVec2::new(244, 305);
    pub const MATX_SIZE: glm::UVec2 = glm::UVec2::new(244, 244);
}

impl FromStr for MotherboardSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "atx" => Ok(MotherboardSize::ATX),
            "microatx" => Ok(MotherboardSize::MicroATX),
            _ => Err(()),
        }
    }
}

impl ToString for MotherboardSize {
    fn to_string(&self) -> String {
        match self {
            MotherboardSize::ATX => "ATX".to_string(),
            MotherboardSize::MicroATX => "MicroATX".to_string()
        }
    }
}
