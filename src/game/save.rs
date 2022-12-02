use std::{io::{Write, Read}, str::FromStr};

use super::player_inventory::PlayerInventory;


#[derive(Debug, Clone)]
pub struct Save {
    pub name: String,
    pub money: u32,
    pub lvl: u32,
    pub points: u32,
    pub inventory: PlayerInventory
}

pub fn save_game(save: &Save) {
    let mut string = String::new();
    string.push_str(format!("money:{}\n", save.money).as_str());
    string.push_str(format!("level:{}\n", save.lvl).as_str());
    string.push_str(format!("points:{}\n", save.points).as_str());

    let mut file = std::fs::File::create(format!("saves/{}.save", save.name)).unwrap();
    file.write_all(string.as_bytes()).unwrap();

    let mut file = std::fs::File::create(format!("saves/{}.json", save.name)).unwrap();
    file.write(serde_json::to_vec_pretty(&save.inventory).unwrap().as_slice()).unwrap();
}

fn get_profile_attribute_value<T: FromStr>(string: &String, attribute: &str) -> Result<T, T::Err> {
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
        money: get_profile_attribute_value(&string, "money").unwrap(),
        lvl: get_profile_attribute_value(&string, "level").unwrap(),
        points: get_profile_attribute_value(&string, "points").unwrap(),
        inventory: serde_json::from_str(std::fs::read_to_string(format!("saves/{}.json", name)).unwrap().as_str()).unwrap()
    }
}
