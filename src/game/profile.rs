#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub level: u64,
    pub points: u64,
    pub money: i64
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

pub fn load_profile(path: impl ToString) -> Option<Profile> {
    let path = path.to_string();

    let data = std::fs::read_to_string(path).ok()?;
    let mut lines = data.lines();

    Some(Profile {
        name: lines.next()?.to_string(),
        level: lines.next()?.parse().ok()?,
        points: lines.next()?.parse().ok()?,
        money: lines.next()?.parse().ok()?
    })
}

pub fn save_profile(profile: &Profile) {
    std::fs::write(
        format!("saves/{}.save", profile.name),
        format!("{}\n{}\n{}\n{}\n", profile.name, profile.level, profile.points, profile.money)
    ).unwrap();
}
