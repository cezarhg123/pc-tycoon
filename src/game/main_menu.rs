use std::io::Cursor;
use gpu_allocator::vulkan::Allocator;
use crate::{primitives::{rect::Rect, text::Text}, ui::{Ui, ui_element::UiElement, button::Button, as_ui_type}, WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct MainMenu {
    background: Rect
}

pub enum MainMenuOutput {
    Play,
    Exit,
    None
}

impl MainMenu {
    pub fn new(allocator: &mut Allocator) -> MainMenu {
        MainMenu {
            background: Rect::builder()
                .width(WINDOW_WIDTH as f32)
                .height(WINDOW_HEIGHT as f32)
                .texture(image::load(
                    Cursor::new(std::fs::read("textures/background.png").unwrap()),
                    image::ImageFormat::Png
                ).unwrap())
                .build(allocator)
        }
    }

    pub fn run(&self, ui: &mut Ui, allocator: &mut Allocator) -> MainMenuOutput {
        let play_button = ui.add_element(UiElement::new(
            "Play",
            true,
            true,
            Button::builder()
                .dimensions(Rect::builder().left(200.0).top(400.0).width(320.0).height(120.0))
                .text(Text::builder().text("Play").font_size(100.0))
                .build(allocator)
        ));

        let exit_button = ui.add_element(UiElement::new(
            "Exit",
            true,
            true,
            Button::builder()
                .dimensions(Rect::builder().left(200.0).top(600.0).width(320.0).height(120.0))
                .text(Text::builder().text("Exit").font_size(100.0))
                .build(allocator)
        ));

        if as_ui_type::<Button>(play_button.borrow()).pressed_once() {
            MainMenuOutput::Play
        } else if as_ui_type::<Button>(exit_button.borrow()).pressed_once() {
            MainMenuOutput::Exit
        } else {
            MainMenuOutput::None
        }
    }

    pub fn draw(&self) {
        self.background.draw();
    }
}
