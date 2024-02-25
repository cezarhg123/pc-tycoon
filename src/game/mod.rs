pub mod main_menu;
pub mod ingame;
pub mod profile;
pub mod build_pc;
pub mod pc;

use gpu_allocator::vulkan::Allocator;
use crate::ui::Ui;
use self::{main_menu::{MainMenu, MainMenuOutput}, ingame::{InGame, InGameOutput}, profile::Profile, build_pc::{BuildPC, BuildPCOutput}};

pub struct Game {
    ui: Ui,
    state: GameState,
    profile: Profile
}

pub enum GameState {
    MainMenu(MainMenu),
    InGame(InGame),
    BuildPC(BuildPC)
}

impl Game {
    pub fn new(allocator: &mut Allocator) -> Game {
        Game {
            ui: Ui::new(),
            state: GameState::MainMenu(MainMenu::new(allocator)),
            profile: Profile::default()
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
                match in_game.run(&mut self.ui, allocator, &self.profile) {
                    InGameOutput::BuildPC => {
                        self.state = GameState::BuildPC(BuildPC::new(allocator));
                        self.ui.clear_elements();
                    },
                    InGameOutput::Inventory => todo!(),
                    InGameOutput::Market => todo!(),
                    InGameOutput::None => {}
                }

                false
            }
            GameState::BuildPC(build_pc) => {
                match build_pc.run(&mut self.ui, allocator) {
                    BuildPCOutput::GoBack => {
                        self.state = GameState::InGame(InGame::new(allocator));
                        self.ui.clear_elements();
                    },
                    BuildPCOutput::None => {},
                }
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
            GameState::BuildPC(build_pc) => {
                build_pc.draw();
            }
        }

        self.ui.draw();
    }
}
