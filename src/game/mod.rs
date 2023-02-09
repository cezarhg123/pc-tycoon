use glium::{Display, Frame, glutin::event::WindowEvent};
use crate::{log::log, math::vec2::{Vec2, vec2}, get_window_height};

pub mod pc_components;
pub mod profile;

pub struct Game {
    cursor_pos: Vec2<f32>,
    // main_menu: Option<MainMenu>,
    // select_save: Option<SelectSave>
}

impl Game {
    pub fn new(display: &Display) -> Game {
        log("loading main menu");
        // let main_menu = Some(MainMenu::new(display));
        log("loaded main menu");
        Game {
            cursor_pos: vec2(0.0, 0.0),
            // main_menu,
            // select_save: None
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent, display: &Display) -> bool {
        false
    }

    pub fn run(&mut self, display: &Display) {
    }

    pub fn draw(&mut self, target: &mut Frame, display: &Display) {
    }
}
