pub mod main_menu;

use gpu_allocator::vulkan::Allocator;
use crate::ui::Ui;
use self::main_menu::{MainMenu, MainMenuOutput};

pub struct Game {
    ui: Ui,
    state: GameState
}

pub enum GameState {
    MainMenu(MainMenu)
}

impl Game {
    pub fn new(allocator: &mut Allocator) -> Game {
        Game {
            ui: Ui::new(),
            state: GameState::MainMenu(MainMenu::new(allocator))
        }
    }

    pub fn handle_events(&mut self, events: impl Iterator<Item = glfw::WindowEvent>, allocator: &mut Allocator) {
        self.ui.handle_events(events, allocator);
    }

    pub fn run(&mut self, allocator: &mut Allocator) -> bool {
        match &mut self.state {
            GameState::MainMenu(menu) => { 
                match menu.run(&mut self.ui, allocator) {
                    MainMenuOutput::Play => {
                        false
                    }
                    MainMenuOutput::Exit => {
                        true
                    }
                    MainMenuOutput::None => {
                        false
                    }
                } 
            }
        }
    }

    pub fn draw(&self) {
        match &self.state {
            GameState::MainMenu(menu) => {
                menu.draw();
            }
        }

        self.ui.draw();
    }
}
