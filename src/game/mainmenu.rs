use glfw::Window;

use crate::{ui::{button::Button, Ui}, gfx::{vectors::vec2::vec2, rect::Rect, image_rect::ImageRect, texture::Texture}, WINDOW_WIDTH, WINDOW_HEIGHT};

#[derive(Debug, Clone)]
pub struct MainMenu<'a> {
    background: ImageRect,
    play_button: Button<'a>,
    exit_button: Button<'a>,
}

impl<'a> MainMenu<'a> {
    pub fn new(ui: &'a Ui) -> MainMenu<'a> {
        MainMenu {
            background: ImageRect::new(Texture::from_path("textures/background.png"), 0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            play_button: ui.button("Play", vec2(220.0, 470.0), vec2(300.0, 80.0)),
            exit_button: ui.button("Exit", vec2(220.0, 560.0), vec2(300.0, 80.0))
        }
    }

    pub fn get_rects(&self) -> Vec<Rect> {
        let mut vec = self.play_button.get_rects();
        vec.append(&mut self.exit_button.get_rects());
        vec   
    }

    /// returns true if play is clicked
    pub fn run(&mut self, window: &mut Window) -> bool {
        if self.play_button.clicked(window) {
            return true;
        }

        if self.exit_button.clicked(window) {
            window.set_should_close(true);
        }

        false
    }

    pub fn draw(&self) {
        self.background.draw();
        self.play_button.draw();
        self.exit_button.draw();
    }
}
