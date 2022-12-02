use glfw::Window;

use crate::{gfx::{image_rect::ImageRect, texture::Texture, text::Text, vectors::{vec3::vec3, vec2::vec2}}, WINDOW_WIDTH, WINDOW_HEIGHT, ui::{Ui, button::Button}};

use super::{save::Save, GameState};

#[derive(Debug, Clone)]
pub struct InGame<'a> {
    background: ImageRect,
    build_button: Button<'a>,
    inventory_button: Button<'a>,
    market_button: Button<'a>,
    contracts_button: Button<'a>,
    money: Text<'a>,
    lvl: Text<'a>,
    points: Text<'a>
}

impl<'a> InGame<'a> {
    pub fn new(active_save: &Save, ui: &'a Ui) -> InGame<'a> {
        InGame {
            background: ImageRect::new(Texture::from_path("textures/ingame-background.png"), 0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            build_button: ui.button("Build PC", vec2(750.0, 300.0), vec2(400.0, 100.0)),
            inventory_button: ui.button("Inventory", vec2(750.0, 420.0), vec2(400.0, 100.0)),
            market_button: ui.button("Market", vec2(750.0, 540.0), vec2(400.0, 100.0)),
            contracts_button: ui.button("Contracts", vec2(750.0, 660.0), vec2(400.0, 100.0)),
            money: ui.text(format!("${}", active_save.money).as_str(), 50.0, vec3(0, 0, 0), Some(vec2(30.0, 267.0))),
            lvl: ui.text(format!("LVL {}", active_save.lvl).as_str(), 50.0, vec3(0, 0, 0), Some(vec2(230.0, 267.0))),
            points: ui.text(format!("{}/1000", active_save.points).as_str(), 50.0, vec3(0, 0, 0), Some(vec2(450.0, 267.0)))
        }
    }

    pub fn run(&mut self, window: &Window) -> GameState {
        if self.build_button.clicked(window) {
            return GameState::PcBuilder;
        }

        if self.inventory_button.clicked(window) {
            return GameState::Inventory;
        }

        if self.market_button.clicked(window) {
            return GameState::Market;
        }

        if self.contracts_button.clicked(window) {
            return GameState::Contracts;
        }

        GameState::InGame
    }

    pub fn draw(&self) {
        self.background.draw();
        self.build_button.draw();
        self.inventory_button.draw();
        self.market_button.draw();
        self.contracts_button.draw();
        self.money.draw();
        self.lvl.draw();
        self.points.draw();
    }
}
