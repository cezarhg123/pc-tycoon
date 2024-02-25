use crate::game::pc::case::Case;

pub static mut CASES: Vec<Case> = Vec::new();

pub fn load_cases() {
    unsafe {
        // TODO: load from files

        CASES.push(Case::default())
    }
}

pub fn get_cases() -> &'static [Case] {
    unsafe {
        &CASES
    }
}
