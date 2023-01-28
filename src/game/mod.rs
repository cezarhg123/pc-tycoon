use glium::{Display, Frame, glutin::event::WindowEvent};

use crate::{log::log, math::vec2::{Vec2, vec2}, get_window_height};

use self::mainmenu::MainMenu;

pub mod pc_components;
pub mod mainmenu;

pub struct Game {
    cursor_pos: Vec2<f32>,
    main_menu: Option<MainMenu>
}

impl Game {
    pub fn new(display: &Display) -> Game {
        log("loading main menu");
        let main_menu = Some(MainMenu::new(display));
        Game {
            cursor_pos: vec2(0.0, 0.0),
            main_menu
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent, display: &Display) -> bool {
        if let WindowEvent::CursorMoved {position, ..} = event {
            let y = get_window_height() as f32 - (position.y as f32);
            self.cursor_pos = vec2(position.x as f32, y);
            // println!("{:#?}", self.cursor_pos);
        }

        if let Some(ref mut main_menu) = self.main_menu {
            if main_menu.handle_event(self.cursor_pos, event, display) {
                return true;
            }
        }

        false
    }

    pub fn draw(&mut self, target: &mut Frame, display: &Display) {
        if let Some(main_menu) = &mut self.main_menu {
            main_menu.draw(target, display);
        }
    }
}
