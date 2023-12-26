use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Profile {
    name: String,
    level: u64,
    points: u64,
    money: i64
}

impl Default for Profile {
    fn default() -> Profile {
        Profile {
            name: String::from("default"),
            level: 1,
            points: 0,
            money: 1500
        }
    }
}

pub fn load_profile(name: impl ToString) -> Result<Profile, Box<dyn std::error::Error>> {
    let name = name.to_string();

    Ok(serde_json::from_str(
        &std::fs::read_to_string(format!("saves/{}.save", name))?)
    ?)
}

pub fn save_profile(profile: &Profile) {
    std::fs::write(
        format!("saves/{}.save", profile.name),
        serde_json::to_string(profile).unwrap()
    ).unwrap();
}
