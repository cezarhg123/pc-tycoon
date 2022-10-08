use std::{io::{Write, Read}, str::FromStr};

#[derive(Debug, Clone)]
pub struct Save {
    pub name: String,
    pub money: i32,
    pub level: u32,
    pub points: u32
}

pub fn save_game(save: &Save) {
    let mut string = String::new();
    string.push_str(format!("money:{}\n", save.money).as_str());
    string.push_str(format!("level:{}\n", save.level).as_str());
    string.push_str(format!("points:{}\n", save.points).as_str());

    let mut file = std::fs::File::create(format!("saves/{}.save", save.name)).unwrap();
    file.write_all(string.as_bytes()).unwrap();
}

fn get_attribute_value<T: FromStr>(string: &String, attribute: &str) -> Result<T, T::Err> {
    string.split("\n")
        .find(|attrib| attrib.contains(attribute))
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .parse()
}

pub fn load_save(name: &str) -> Save {
    //TODO: REMOVE UNWRAP AND ALERT ON SCREEN THAT SAVE DOESNT EXIST
    let mut file = std::fs::File::open(format!("saves/{}.save", name)).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    
    Save {
        name: name.to_string(),
        money: get_attribute_value(&string, "money").unwrap(),
        level: get_attribute_value(&string, "level").unwrap(),
        points: get_attribute_value(&string, "points").unwrap()
    }
}
