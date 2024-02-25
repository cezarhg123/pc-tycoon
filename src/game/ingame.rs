use std::io::Cursor;
use gpu_allocator::vulkan::Allocator;
use crate::{primitives::{rect::Rect, text::Text}, WINDOW_WIDTH, WINDOW_HEIGHT, ui::{Ui, ui_element::UiElement, button::Button, ui_text::UiText, as_ui_type}};
use super::profile::Profile;

pub struct InGame {
    background: Rect,
}

pub enum InGameOutput {
    BuildPC,
    Inventory,
    Market,
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

    pub fn run(&mut self, ui: &mut Ui, allocator: &mut Allocator, profile: &Profile) -> InGameOutput {
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

        let money_text = ui.add_element(
            UiElement::new(
                "Player Money",
                true,
                true,
                UiText::new(
                    Text::builder()
                        .left(13.0)
                        .top(266.0)
                        .font_size(48.0)
                        .font_color(glm::vec3(0.0, 0.0, 0.0))
                        .text(format!("€{}", profile.money))
                        .build(allocator)
                )
            )
        );

        let level_text = ui.add_element(
            UiElement::new(
                "Player Level",
                true,
                true,
                UiText::new(
                    Text::builder()
                        .left(249.0)
                        .top(266.0)
                        .font_size(48.0)
                        .font_color(glm::vec3(0.0, 0.0, 0.0))
                        .text(format!("LVL {}", profile.level))
                        .build(allocator)
                )
            )
        );

        let points = ui.add_element(
            UiElement::new(
                "Player Points",
                true,
                true,
                UiText::new(
                    Text::builder()
                        .left(0.0)
                        .top(266.0)
                        .font_size(48.0)
                        .font_color(glm::vec3(0.0, 0.0, 0.0))
                        .text(format!("{}/{}", profile.points, 1000))
                        .build(allocator)
                )
            )
        );
        points.borrow_mut().ui_object.set_right(620.0);
        
        if as_ui_type::<Button>(build_pc_button.borrow()).pressed_once() {
            InGameOutput::BuildPC
        } else if as_ui_type::<Button>(inventory_button.borrow()).pressed_once() {
            InGameOutput::Inventory
        } else if as_ui_type::<Button>(market_button.borrow()).pressed_once() {
            InGameOutput::Market
        } else {
            InGameOutput::None
        }
    }

    pub fn draw(&self) {
        self.background.draw();
    }
}
