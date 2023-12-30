use std::io::Cursor;
use gpu_allocator::vulkan::Allocator;
use crate::{primitives::{rect::Rect, text::Text}, WINDOW_WIDTH, WINDOW_HEIGHT, ui::{Ui, ui_element::UiElement, button::Button}};

pub struct InGame {
    background: Rect,
}

pub enum InGameOutput {
    None
}

impl InGame {
    pub fn new(allocator: &mut Allocator) -> InGame {
        InGame {
            background: Rect::builder()
                .texture(
                    image::load(
                        Cursor::new(std::fs::read("textures/ingame-background.png").unwrap()),
                        image::ImageFormat::Png
                    ).unwrap()
                )
                .width(WINDOW_WIDTH as f32)
                .height(WINDOW_HEIGHT as f32)
                .build(allocator)
        }
    }

    pub fn run(&mut self, ui: &mut Ui, allocator: &mut Allocator) -> InGameOutput {
        let build_pc_button = ui.add_element(
            UiElement::new(
                "Build PC",
                true,
                true,
                Button::builder()
                    .dimensions(Rect::builder().left(646.0).top(248.0).width(605.0).height(120.0))
                    .text(Text::builder().text("Build PC").font_size(40.0))
                    .build(allocator)
            )
        );

        let inventory_button = ui.add_element(
            UiElement::new(
                "Inventory",
                true,
                true,
                Button::builder()
                    .dimensions(Rect::builder().left(646.0).top(375.0).width(605.0).height(120.0))
                    .text(Text::builder().text("Inventory").font_size(40.0))
                    .build(allocator)
            )
        );

        let market_button = ui.add_element(
            UiElement::new(
                "Market",
                true,
                true,
                Button::builder()
                    .dimensions(Rect::builder().left(646.0).top(502.0).width(605.0).height(120.0))
                    .text(Text::builder().text("Market").font_size(40.0))
                    .build(allocator)
            )
        );
        
        InGameOutput::None
    }

    pub fn draw(&self) {
        self.background.draw();
    }
}
