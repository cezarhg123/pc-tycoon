use glium::{Display, Frame, glutin::event::WindowEvent};
use crate::{log::log, math::vec2::{Vec2, vec2}, get_window_height, get_ui_mut};
use self::{main_menu::{MainMenu, MainMenuOutput}, select_save::SelectSave, profile::Profile};

pub mod pc_components;
pub mod profile;
pub mod main_menu;
pub mod select_save;

pub enum GameState {
    MainMenu(MainMenu),
    SelectSave(SelectSave)
}

pub struct Game {
    state: GameState,
    profile: Option<Profile>
}

impl Game {
    pub fn new(display: &Display) -> Game {
        log("loading main menu");
        let main_menu = MainMenu::new(display);
        log("loaded main menu");
        Game {
            state: GameState::MainMenu(main_menu),
            profile: None
        }
    }

    pub fn run(&mut self, display: &Display) {
        // using rust's good-ass enums
        // https://www.youtube.com/watch?v=Ux5cQbO_ybw
        match &mut self.state {
            GameState::MainMenu(main_menu) => {
                if main_menu.run() == MainMenuOutput::Play {
                    log("loading select save");
                    get_ui_mut().clear();
                    self.state = GameState::SelectSave(SelectSave::new(display));
                    log("loaded select save");
                }
            }
            GameState::SelectSave(select_save) => {
                self.profile = select_save.run();
            }
        }
    }

    pub fn draw(&mut self, target: &mut Frame) {
        match &self.state {
            GameState::MainMenu(main_menu) => {
                main_menu.draw(target);
            }
            GameState::SelectSave(select_save) => {
                select_save.draw(target);
            }
        }
    }
}
