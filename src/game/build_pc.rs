use std::io::Cursor;
use gpu_allocator::vulkan::Allocator;
use crate::{primitives::rect::Rect, WINDOW_WIDTH, WINDOW_HEIGHT, ui::{Ui, ui_element::UiElement, button::Button, as_ui_type}};

pub struct BuildPC {
    background: Rect
}

pub enum BuildPCOutput {
    GoBack,
    None
}

impl BuildPC {
    pub fn new(allocator: &mut Allocator) -> BuildPC {
        BuildPC {
            background: Rect::builder()
                .width(WINDOW_WIDTH as f32)
                .height(WINDOW_HEIGHT as f32)
                .texture(image::load(
                    Cursor::new(std::fs::read("textures/pc-builder.png").unwrap()),
                    image::ImageFormat::Png
                ).unwrap())
                .build(allocator)
        }
    }

    pub fn run(&mut self, ui: &mut Ui, allocator: &mut Allocator) -> BuildPCOutput {
        let go_back_button = ui.add_element(UiElement::new(
            "Go Back",
            true,
            true,
            Button::builder()
                .dimensions(
                    Rect::builder()
                        .width(156.0)
                        .height(84.0)
                        .right(WINDOW_WIDTH as f32)
                )
                .normal_face(crate::ui::button::ButtonFace::Color(glm::vec4(1.0, 1.0, 1.0, 0.0))) // make transparent
                .build(allocator)
        ));

        if as_ui_type::<Button>(go_back_button.borrow()).pressed_once() {
            return BuildPCOutput::GoBack;
        }

        

        BuildPCOutput::None
    }

    pub fn draw(&self) {
        self.background.draw();
    }
}
