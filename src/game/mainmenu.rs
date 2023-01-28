use std::io::Cursor;
use glium::{Display, Frame, glutin::event::WindowEvent};
use crate::{gfx::Rect, math::{vec2::{vec2, Vec2}, vec3::vec3}, DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT, log::log, ui::button::{Button, ButtonBuilder}, close};

#[derive(Debug, PartialEq)]
pub enum MainMenuOutput {
    Play,
    None
}

#[derive(Debug)]
pub struct MainMenu {
    background: Rect,
    play_button: Button,
    exit_button: Button
}

impl MainMenu {
    pub fn new(display: &Display) -> MainMenu {
        let mut background = Rect::new(vec2(960.0, 540.0), vec2(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32), display);
        let image = image::load(Cursor::new(std::fs::read("textures/background.png").unwrap()), image::ImageFormat::Png).unwrap();
        background.set_texture(&image, display);
        
        MainMenu {
            background,
            play_button: ButtonBuilder {
                position: vec2(374.0, 600.0),
                size: vec2(300.0, 100.0),
                text: Some("Play".to_string()),
                font_size: Some(72.0),
                text_color: None,
                colors: (vec3(0.2, 0.2, 0.2), None),
                textures: (None, None)
            }.build(display),
            exit_button: ButtonBuilder {
                position: vec2(374.0, 450.0),
                size: vec2(300.0, 100.0),
                text: Some("Exit".to_string()),
                font_size: Some(72.0),
                text_color: None,
                colors: (vec3(0.2, 0.2, 0.2), None),
                textures: (None, None)
            }.build(display)
        }
    }

    pub fn handle_event(&mut self, cursor_pos: Vec2<f32>, event: &WindowEvent, display: &Display) -> bool {
        let play_button_out = self.play_button.handle_event(cursor_pos, event, display);
        let exit_button_out = self.exit_button.handle_event(cursor_pos, event, display);

        play_button_out || exit_button_out
    }

    pub fn run(&mut self) -> MainMenuOutput {
        if self.exit_button.clicked() {
            close();
        }

        if self.play_button.clicked() {
            MainMenuOutput::Play
        } else {
            MainMenuOutput::None
        }
    }

    pub fn draw(&mut self, target: &mut Frame, display: &Display) {
        self.background.draw(target);
        self.play_button.draw(target, display);
        self.exit_button.draw(target, display);
    }
}
