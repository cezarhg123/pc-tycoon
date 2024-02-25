use std::str::FromStr;
use gpu_allocator::vulkan::Allocator;
use image::RgbaImage;
use rand::{Rng, SeedableRng};
use crate::primitives::rect::Rect;
use super::{drive::DriveType, motherboard::MotherboardSize, pc_part::PCPart, pc_rect::PCRect};

#[derive(Debug, Clone)]
pub struct Case {
    name: String,
    dev_name: String,
    case_size: CaseSize,

    dimensions: glm::UVec2,
    side_thickness: u32,
    motherboard_area: PCRect,
    motherboard_size: MotherboardSize,
    psu_shroud: PCRect,
    /// Area where actual PSU can be placed
    psu_area: PCRect,
    drive_areas: Vec<PCRect>,
    fan_areas: Vec<PCRect>
}

impl Default for Case {
    fn default() -> Case {
        Case {
            name: "Default Case".to_string(),
            dev_name: "default-case".to_string(),
            case_size: CaseSize::MidTower,

            dimensions: glm::UVec2::new(300, 340),
            side_thickness: 10,
            motherboard_area: PCRect::new(10, 10, 200, 230),
            motherboard_size: MotherboardSize::ATX,
            psu_shroud: PCRect::new(10, 260, 130, 70),
            psu_area: PCRect::new(10, 260, 130, 70),
            drive_areas: Vec::new(),
            fan_areas: Vec::new()
        }
    }
}

impl Case {
    pub fn generate_image(&self) -> RgbaImage {
        let mut image = RgbaImage::new(self.dimensions.x, self.dimensions.y);
        
        // draw case
        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                image.put_pixel(x, y, image::Rgba([59, 59, 59, 255]));
            }
        }

        // draw sides
        // left and right
        for x in 0..self.side_thickness {
            for y in 0..self.dimensions.y {
                image.put_pixel(x, y, image::Rgba([28, 28, 28, 255])); // left
                image.put_pixel(self.dimensions.x - x - 1, y, image::Rgba([28, 28, 28, 255])); // right
            }
        }

        // top and bottom
        for x in 0..self.dimensions.x {
            for y in 0..self.side_thickness {
                image.put_pixel(x, y, image::Rgba([28, 28, 28, 255])); // top
                image.put_pixel(x, self.dimensions.y - y - 1, image::Rgba([28, 28, 28, 255])); // bottom
            }
        }

        // draw psu shroud
        for x in self.psu_shroud.left()..self.psu_shroud.right() {
            for y in self.psu_shroud.top()..self.psu_shroud.bottom() {
                image.put_pixel(x, y, image::Rgba([92, 92, 92, 255]));
            }
        }

        image
    }

    /// Offset: relative to pc rect
    pub fn get_motherboard_area(&self, offset: glm::Vec2, allocator: &mut Allocator) -> Rect {
        Rect::builder()
            .left(self.motherboard_area.left() as f32 + offset.x)
            .top(self.motherboard_area.top() as f32 + offset.y)
            .width(self.motherboard_area.width() as f32)
            .height(self.motherboard_area.height() as f32)
            .color(glm::vec4(1.0, 1.0, 1.0, 0.4))
            .build(allocator)
    }

    /// Offset: relative to pc rect
    pub fn get_psu_area(&self, offset: glm::Vec2, allocator: &mut Allocator) -> Rect {
        Rect::builder()
            .left(self.psu_area.left() as f32 + offset.x)
            .top(self.psu_area.top() as f32 + offset.y)
            .width(self.psu_area.width() as f32)
            .height(self.psu_area.height() as f32)
            .color(glm::vec4(1.0, 1.0, 1.0, 0.4))
            .build(allocator)
    }

    pub fn get_drive_areas(&self, offset: glm::Vec2, allocator: &mut Allocator) -> Vec<Rect> {
        self.drive_areas.iter().map(|area| {
            Rect::builder()
                .left(area.left() as f32 + offset.x)
                .top(area.top() as f32 + offset.y)
                .width(area.width() as f32)
                .height(area.height() as f32)
                .color(glm::vec4(1.0, 1.0, 1.0, 0.4))
                .build(allocator)
        }).collect()
    }

    pub fn get_fan_areas(&self, offset: glm::Vec2, allocator: &mut Allocator) -> Vec<Rect> {
        self.fan_areas.iter().map(|area| {
            Rect::builder()
                .left(area.left() as f32 + offset.x)
                .top(area.top() as f32 + offset.y)
                .width(area.width() as f32)
                .height(area.height() as f32)
                .color(glm::vec4(1.0, 1.0, 1.0, 0.4))
                .build(allocator)
        }).collect()
    }

    pub fn motherboard_area_contains(&self, pos: glm::Vec2) -> bool {
        self.motherboard_area.contains(glm::vec2(pos.x as u32, pos.y as u32))
    }

    pub fn psu_area_contains(&self, pos: glm::Vec2) -> bool {
        self.psu_area.contains(glm::vec2(pos.x as u32, pos.y as u32))
    }

    /// Returns Some(fan index) if a fan area contains the given position
    pub fn fan_areas_contain(&self, pos: glm::Vec2) -> Option<usize> {
        for fan_area in self.fan_areas.iter().enumerate() {
            if fan_area.1.contains(glm::vec2(pos.x as u32, pos.y as u32)) {
                return Some(fan_area.0);
            }
        }

        None
    }

    // GETTERS
    pub fn case_size(&self) -> CaseSize {
        self.case_size.clone()
    }

    pub fn dimensions(&self) -> glm::UVec2 {
        self.dimensions
    }

    pub fn side_thickness(&self) -> u32 {
        self.side_thickness
    }

    pub fn motherboard_area(&self) -> PCRect {
        self.motherboard_area.clone()
    }

    pub fn motherboard_size(&self) -> MotherboardSize {
        self.motherboard_size.clone()
    }

    pub fn psu_shroud(&self) -> PCRect {
        self.psu_shroud.clone()
    }

    pub fn psu_area(&self) -> PCRect {
        self.psu_area.clone()
    }

    pub fn fan_areas(&self) -> &[PCRect] {
        &self.fan_areas
    }

    pub fn drive_areas(&self) -> &[PCRect] {
        &self.drive_areas
    }

    // SETTERS
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_dev_name(&mut self, dev_name: String) {
        self.dev_name = dev_name;
    }

    pub fn set_case_size(&mut self, case_size: CaseSize) {
        self.case_size = case_size;
    }

    pub fn set_dimensions(&mut self, dimensions: glm::UVec2) {
        self.dimensions = dimensions;
    }

    pub fn set_side_thickness(&mut self, side_thickness: u32) {
        self.side_thickness = side_thickness;
    }

    pub fn set_motherboard_area(&mut self, motherboard_area: PCRect) {
        self.motherboard_area = motherboard_area;
    }

    pub fn set_motherboard_size(&mut self, motherboard_size: MotherboardSize) {
        self.motherboard_size = motherboard_size;
    }

    pub fn set_psu_shroud(&mut self, psu_shroud: PCRect) {
        self.psu_shroud = psu_shroud;
    }

    pub fn set_psu_area(&mut self, psu_area: PCRect) {
        self.psu_area = psu_area;
    }

    pub fn add_drive_area(&mut self, drive_area: PCRect) {
        self.drive_areas.push(drive_area);
    }

    pub fn set_drive_area(&mut self, i: usize, drive_area: PCRect) {
        self.drive_areas[i] = drive_area;
    }

    pub fn add_fan_area(&mut self, fan_area: PCRect) {
        self.fan_areas.push(fan_area);
    }
    
    pub fn set_fan_area(&mut self, i: usize, fan_area: PCRect) {
        self.fan_areas[i] = fan_area;
    }
}

impl PCPart for Case {
    fn name(&self) -> &str {
        &self.name
    }

    fn dev_name(&self) -> &str {
        &self.dev_name
    }

    fn random(seed: u64) -> Self
        where Self: Sized {

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        
        let case_name = format!(
            "{brand} {model} {model_num}",
            brand=CASE_BRANDS[rng.gen::<usize>() % CASE_BRANDS.len()],
            model=CASE_MODELS[rng.gen::<usize>() % CASE_MODELS.len()],
            model_num=(rng.gen::<u32>() % 100) * 10
        );

        let case_size = rng.gen::<u8>() % CaseSize::TYPE_COUNT;
        let case_size = match case_size {
            0 => CaseSize::MidTower,
            1 => CaseSize::FullTower,
            _ => unreachable!(),
        };

        let mut dimensions;
        match case_size {
            CaseSize::MidTower => {
                dimensions = CaseSize::MIDTOWER_SIZE;
            },
            CaseSize::FullTower => {
                dimensions = CaseSize::FULLTOWER_SIZE;
            },
        }

        dimensions.x += rng.gen::<u32>() % 120;
        dimensions.y += rng.gen::<u32>() % 120;

        let side_thickness = rng.gen::<u32>() % 14 + 2;

        let mut motherboard_area = PCRect::new(side_thickness, side_thickness, 0, 0);
        let motherboard_size;
        match rng.gen::<u8>() % MotherboardSize::TYPE_COUNT {
            0 => {
                motherboard_size = MotherboardSize::ATX;
                motherboard_area.set_width(MotherboardSize::ATX_SIZE.x);
                motherboard_area.set_height(MotherboardSize::ATX_SIZE.y);
            }
            1 => {
                motherboard_size = MotherboardSize::MicroATX;
                motherboard_area.set_width(MotherboardSize::MATX_SIZE.x);
                motherboard_area.set_height(MotherboardSize::MATX_SIZE.y);
            }
            _ => unreachable!(),
        }

        let mut psu_shroud = PCRect::new(side_thickness, 0, 0, 86);
        let mut psu_area = PCRect::new(side_thickness, 0, 0, 86);

        let shroud_max_width = dimensions.x - side_thickness * 2;
        match rng.gen::<u8>() % 3 {
            0 => {
                psu_shroud.set_width(shroud_max_width / 3);

                psu_area.set_width(shroud_max_width / 3);
            }
            1 => {
                psu_shroud.set_width(shroud_max_width / 2);

                psu_area.set_width(shroud_max_width / 2);
            }
            2 => {
                psu_shroud.set_width(shroud_max_width);

                psu_area.set_width(shroud_max_width / 2);
            }
            _ => unreachable!()
        }
        psu_shroud.set_bottom(dimensions.y - side_thickness);
        psu_area.set_bottom(psu_shroud.bottom());

        let mut drive_areas = Vec::new();

        // if psu shroud is as big as the case, then add 2 drives in it
        if psu_shroud.width() == shroud_max_width {
            let mut drive_area = PCRect::new(0, 0, DriveType::HDD_SIZE_FLAT.x, DriveType::HDD_SIZE_FLAT.y);
            drive_area.set_right(dimensions.x - side_thickness - 30);
            drive_area.set_bottom(dimensions.y - side_thickness);
            drive_areas.push(drive_area.clone());
            drive_area.set_bottom(drive_area.bottom() - 25);
            drive_areas.push(drive_area);
        } else { // else, make a tower of drives
            let max_height = dimensions.y - (side_thickness * 2);
            let max_drives = max_height / (DriveType::HDD_SIZE_FLAT.y + 10);

            let mut last_top = dimensions.y - side_thickness;

            for _ in 0..rng.gen::<u8>() % (max_drives - 2) as u8 + 2 {
                let mut drive_area = PCRect::new(dimensions.x - side_thickness - DriveType::HDD_SIZE_FLAT.x - 30, 0, DriveType::HDD_SIZE_FLAT.x, DriveType::HDD_SIZE_FLAT.y);

                drive_area.set_bottom(last_top);
                last_top = drive_area.top() - 10;

                drive_areas.push(drive_area);
            }
        }

        let mut fan_areas = Vec::new();

        // front fans
        let max_height = dimensions.y - (side_thickness * 2);
        // if random bool is true, then fans are 140mm, else 120mm
        let (max_fans, front_fan_size) = if rng.gen_bool(0.5) {
            ((max_height - 30) / 140, 140)
        } else {
            ((max_height - 30) / 120, 120)
        };

        let mut last_top = side_thickness + 10;

        for _ in 0..rng.gen::<u8>() % (max_fans - 1) as u8 + 1 {
            let fan_area = PCRect::new(dimensions.x - side_thickness - 20, last_top, 20, front_fan_size);
            last_top = fan_area.bottom() + 10;
            fan_areas.push(fan_area);
        }

        // back fan
        fan_areas.push(PCRect::new(side_thickness, side_thickness, 20, 120));

        // top fans
        let max_width = dimensions.y - (side_thickness * 2);
        let (max_fans, top_fan_size) = if rng.gen_bool(0.5) {
            ((max_width - 50) / 140, 140)
        } else {
            ((max_width - 50) / 120, 120)
        };

        let mut last_left = side_thickness + 20;

        for _ in 0..rng.gen::<u8>() % (max_fans - 1) as u8 + 1 {
            let fan_area = PCRect::new(last_left, side_thickness, top_fan_size, 20);
            last_left = fan_area.right() + 10;
            fan_areas.push(fan_area);
        }

        Case {
            dev_name: case_name.to_lowercase().replace(" ", "-"),
            name: case_name,
            case_size,
            dimensions,
            side_thickness,
            motherboard_area,
            motherboard_size,
            psu_shroud,
            psu_area,
            drive_areas,
            fan_areas
        }
    }

    fn save(&self) {
        let mut data = String::new();

        data.push_str(&self.name);
        data.push('\n');
        data.push_str(&self.dev_name);
        data.push('\n');
        data.push_str(self.case_size.to_string().as_str());
        data.push('\n');
        data.push_str(self.dimensions.x.to_string().as_str());
        data.push('\n');
        data.push_str(self.dimensions.y.to_string().as_str());
        data.push('\n');
        data.push_str(self.side_thickness.to_string().as_str());
        data.push('\n');
        data.push_str(self.motherboard_area.left().to_string().as_str());
        data.push('\n');
        data.push_str(self.motherboard_area.top().to_string().as_str());
        data.push('\n');
        data.push_str(self.motherboard_area.width().to_string().as_str());
        data.push('\n');
        data.push_str(self.motherboard_area.height().to_string().as_str());
        data.push('\n');
        data.push_str(self.motherboard_size.to_string().as_str());
        data.push('\n');
        data.push_str(self.psu_shroud.left().to_string().as_str());
        data.push('\n');
        data.push_str(self.psu_shroud.top().to_string().as_str());
        data.push('\n');
        data.push_str(self.psu_shroud.width().to_string().as_str());
        data.push('\n');
        data.push_str(self.psu_shroud.height().to_string().as_str());
        data.push('\n');
        data.push_str(self.psu_area.left().to_string().as_str());
        data.push('\n');
        data.push_str(self.psu_area.top().to_string().as_str());
        data.push('\n');
        data.push_str(self.psu_area.width().to_string().as_str());
        data.push('\n');
        data.push_str(self.psu_area.height().to_string().as_str());
        data.push('\n');
        //drives
        data.push_str(";\n");
        for drive in &self.drive_areas {
            data.push_str(drive.left().to_string().as_str());
            data.push(',');
            data.push_str(drive.top().to_string().as_str());
            data.push(',');
            data.push_str(drive.width().to_string().as_str());
            data.push(',');
            data.push_str(drive.height().to_string().as_str());
            data.push('\n');
        }
        data.push_str(";\n");
        //fans
        data.push_str(";\n");
        for fan in &self.fan_areas {
            data.push_str(fan.left().to_string().as_str());
            data.push(',');
            data.push_str(fan.top().to_string().as_str());
            data.push(',');
            data.push_str(fan.width().to_string().as_str());
            data.push(',');
            data.push_str(fan.height().to_string().as_str());
            data.push('\n');
        }
        data.push_str(";\n");

        std::fs::write(
            format!("parts/cases/{}.json", self.dev_name),
            data
        ).unwrap();
    }

    fn load(path: String) -> Option<Self>
        where Self: Sized {
        let data = std::fs::read_to_string(path).ok()?;
        let mut lines = data.lines();

        let name = lines.next()?.to_string();
        let dev_name = lines.next()?.to_string();
        let case_size = CaseSize::from_str(lines.next()?).ok()?;
        let dimensions = glm::UVec2::new(
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?
        );
        let side_thickness = lines.next()?.parse::<u32>().ok()?;
        let motherboard_area = PCRect::new(
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?
        );
        let motherboard_size = MotherboardSize::from_str(lines.next()?).ok()?;
        let psu_shroud = PCRect::new(
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?
        );
        let psu_area = PCRect::new(
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?,
            lines.next()?.parse().ok()?
        );
        
        let mut drive_areas = Vec::new();
        if lines.next()? == ";" {
            while let Some(line) = lines.next() {
                if line == ";" {
                    break;
                }

                let mut parts = line.split(',');
                drive_areas.push(PCRect::new(
                    parts.next()?.parse().ok()?,
                    parts.next()?.parse().ok()?,
                    parts.next()?.parse().ok()?,
                    parts.next()?.parse().ok()?
                ));
            }
        }

        let mut fan_areas = Vec::new();
        if lines.next()? == ";" {
            while let Some(line) = lines.next() {
                if line == ";" {
                    break;
                }

                let mut parts = line.split(',');
                fan_areas.push(PCRect::new(
                    parts.next()?.parse().ok()?,
                    parts.next()?.parse().ok()?,
                    parts.next()?.parse().ok()?,
                    parts.next()?.parse().ok()?
                ));
            }
        }
        
        Some(Case {
            name,
            dev_name,
            case_size,
            dimensions,
            side_thickness,
            motherboard_area,
            motherboard_size,
            psu_shroud,
            psu_area,
            drive_areas,
            fan_areas
        })
    }
}

#[test]
fn case_test() {
    let seed = 10;
    for i in seed..seed + 10 {
        let case = Case::random(i);
        case.save();

        let case = Case::load(format!("parts/cases/{}.case", case.dev_name)).unwrap();

        let mut image = case.generate_image();
        for x in 0..image.width() {
            for y in 0..image.height() {
                if case.motherboard_area_contains(glm::vec2(x as f32, y as f32)) {
                    let pixel = image.get_pixel_mut(x, y);
                    pixel.0[1] += 40;
                }

                for drive_area in &case.drive_areas {
                    if drive_area.contains(glm::vec2(x, y)) {
                        let pixel = image.get_pixel_mut(x, y);
                        pixel.0[0] += 40;
                    }
                }

                for fan_area in &case.fan_areas {
                    if fan_area.contains(glm::vec2(x, y)) {
                        let pixel = image.get_pixel_mut(x, y);
                        pixel.0[2] += 40;
                    }
                }
            }
        }

        image.save(&format!("tests/case/{}.png", case.dev_name)).unwrap();
    }
}

#[derive(Debug, Clone)]
pub enum CaseSize {
    MidTower,
    FullTower
}

impl CaseSize {
    /// number of case sizes
    const TYPE_COUNT: u8 = 2;

    const MIDTOWER_SIZE: glm::UVec2 = glm::UVec2::new(400, 400);
    const FULLTOWER_SIZE: glm::UVec2 = glm::UVec2::new(550, 550);
}

impl FromStr for CaseSize {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "midtower" => Ok(CaseSize::MidTower),
            "fulltower" => Ok(CaseSize::FullTower),
            _ => Err("Invalid case size".to_string()),
        }
    }
}

impl ToString for CaseSize {
    fn to_string(&self) -> String {
        match self {
            CaseSize::MidTower => "MidTower".to_string(),
            CaseSize::FullTower => "FullTower".to_string()
        }
    }
}

#[test]
fn case_images_test() {
    Case::default().generate_image().save("test.png").unwrap();
}

const CASE_BRANDS: [&str; 3] = [
    "PowerMax",
    "CyberComp",
    "Micropute"
];

const CASE_MODELS: [&str; 3] = [
    "Arc",
    "Flow",
    "Vortex"
];
