pub mod main_menu;
pub mod ingame;
pub mod profile;

use gpu_allocator::vulkan::Allocator;
use crate::ui::Ui;
use self::{main_menu::{MainMenu, MainMenuOutput}, ingame::InGame};

pub struct Game {
    ui: Ui,
    state: GameState
}

pub enum GameState {
    MainMenu(MainMenu),
    InGame(InGame)
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
                        self.state = GameState::InGame(InGame::new(allocator));
                        self.ui.clear_elements();
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
            GameState::InGame(in_game) => {
                
                false
            }
        }
    }

    pub fn draw(&self) {
        match &self.state {
            GameState::MainMenu(menu) => {
                menu.draw();
            }
            GameState::InGame(in_game) => {
                in_game.draw();
            }
        }

        self.ui.draw();
    }
}
