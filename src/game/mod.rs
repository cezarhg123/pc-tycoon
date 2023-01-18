use glium::{Display, Frame};

use self::mainmenu::MainMenu;

pub mod pc_components;
pub mod mainmenu;

pub struct Game {
    main_menu: Option<MainMenu>
}

impl Game {
    pub fn new(display: &Display) -> Game {
        Game {
            main_menu: Some(MainMenu::new(display))
        }
    }

    pub fn draw(&self, target: &mut Frame) {
        match &self.main_menu {
            Some(main_menu) => {
                main_menu.draw(target);
            }
            None => {}
        }
    }
}
