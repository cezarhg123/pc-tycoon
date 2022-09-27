mod save;

use imgui_glfw_rs::glfw::Window;

use crate::rect::Rect;

pub use self::save::{Save, save_save, load_save};

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    MainMenu,
    InGame
}

pub struct Game {
    active_save: Save,
    game_state: GameState,
    background: Rect,
}

impl Game {
    pub fn new() -> Self {
        Game {
            active_save: Save {
                name: "save1".to_string(),
                money: 1500,
                level: 1,
                points: 0
            },
            game_state: GameState::MainMenu,
            background: Rect::new(0, 0, 1920, 1080, "textures/background.png")
        }
    }

    pub fn start(&mut self) {
    }

    fn main_menu(&mut self, window: &mut Window) {
        if self.game_state == GameState::MainMenu {
            
        }
    }

    fn ingame(&mut self) {
        if self.game_state == GameState::InGame {
        }
    }

    pub fn run(&mut self, window: &mut Window) {
        self.background.draw();
        self.main_menu(window);
        self.ingame();
    }
}
