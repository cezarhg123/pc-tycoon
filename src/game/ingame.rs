use std::io::Cursor;
use glium::{Display, Frame};
use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::vec2, vec4::vec4, vec3::vec3}, get_window_width, get_window_height, log::{log, save_log}, get_ui_mut, ui::{button::{ButtonBuilder, ButtonTextType, ButtonFace}, textline::TextLineBuilder, uielement::UiOutput}, get_ui};
use super::profile::Profile;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InGameOutput {
    None,
    BuildPC,
    Inventory,
    Market
}

pub struct InGame {
    background: Rect
}

impl InGame {
    pub fn new(display: &Display, profile: &Profile) -> InGame {
        let background = RectBuilder {
            position: vec2(get_window_width() as f32 / 2.0, get_window_height() as f32 / 2.0),
            size: vec2(get_window_width() as f32, get_window_height() as f32),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: Some(image::load(
                Cursor::new(std::fs::read("textures/ingame-background.png").unwrap_or_else(|e| {
                    log(format!("NH Err: {}", e.to_string()));
                    save_log();
                    panic!();
                })),
                image::ImageFormat::Png
            ).unwrap_or_else(|e| {
                log(format!("NH Err: {}", e.to_string()));
                save_log();
                panic!();
            }))
        }.build(display);

        // creating buttons
        let build_pc_button = get_ui_mut().add_element(ButtonBuilder {
            id: "build_pc_button".to_string(),
            custom_data: Vec::new(),
            position: vec2(948.0, 780.0),
            size: vec2(600.0, 100.0),
            text: Some(ButtonTextType::Single(TextLineBuilder {
                id: "Build PC".to_string(),
                custom_data: Vec::new(),
                text: "Build PC".to_string(),
                font_size: 64.0,
                color: vec3(1.0, 1.0, 1.0),
                bold: false,
                position: vec2(948.0, 780.0)
            }.build(display))),
            normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
            hovered_face: None,
            clicked_face: None
        }.build(display));

        let inventory_button = get_ui_mut().add_element(ButtonBuilder {
            id: "inventory_button".to_string(),
            custom_data: Vec::new(),
            position: vec2(948.0, 660.0),
            size: vec2(600.0, 100.0),
            text: Some(ButtonTextType::Single(TextLineBuilder {
                id: "Inventory".to_string(),
                custom_data: Vec::new(),
                text: "Inventory".to_string(),
                font_size: 64.0,
                color: vec3(1.0, 1.0, 1.0),
                bold: false,
                position: vec2(948.0, 660.0)
            }.build(display))),
            normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
            hovered_face: None,
            clicked_face: None
        }.build(display));

        let market_button = get_ui_mut().add_element(ButtonBuilder {
            id: "market_button".to_string(),
            custom_data: Vec::new(),
            position: vec2(948.0, 540.0),
            size: vec2(600.0, 100.0),
            text: Some(ButtonTextType::Single(TextLineBuilder {
                id: "Market".to_string(),
                custom_data: Vec::new(),
                text: "Market".to_string(),
                font_size: 64.0,
                color: vec3(1.0, 1.0, 1.0),
                bold: false,
                position: vec2(948.0, 540.0)
            }.build(display))),
            normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
            hovered_face: None,
            clicked_face: None
        }.build(display));

        // creating profile texts that show stats
        let money_text = get_ui_mut().add_element(TextLineBuilder {
            id: "Money".to_string(),
            custom_data: Vec::new(),
            text: format!("${}", profile.money),
            font_size: 56.0,
            color: vec3(0.0, 0.0, 0.0),
            bold: true,
            position: vec2(91.0, 791.0)
        }.build(display));

        let level_text = get_ui_mut().add_element(TextLineBuilder {
            id: "Level".to_string(),
            custom_data: Vec::new(),
            text: format!("LVL: {}", profile.level),
            font_size: 56.0,
            color: vec3(0.0, 0.0, 0.0),
            bold: true,
            position: vec2(267.0, 791.0)
        }.build(display));

        let points_text = get_ui_mut().add_element(TextLineBuilder {
            id: "Points".to_string(),
            custom_data: Vec::new(),
            text: format!("{}/{}", profile.points, profile.goal),
            font_size: 56.0,
            color: vec3(0.0, 0.0, 0.0),
            bold: true,
            position: vec2(505.0, 791.0)
        }.build(display));

        InGame {
            background
        }
    }

    pub fn run(&mut self) -> InGameOutput {
        if get_ui().get_element("build_pc_button").unwrap().output() == UiOutput::LeftClicked {
            return InGameOutput::BuildPC;
        } if get_ui().get_element("inventory_button").unwrap().output() == UiOutput::LeftClicked {
            return InGameOutput::Inventory;
        } if get_ui().get_element("market_button").unwrap().output() == UiOutput::LeftClicked {
            return InGameOutput::Market;
        }

        InGameOutput::None
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);

        get_ui().get_element("build_pc_button").unwrap().draw(target);
        get_ui().get_element("inventory_button").unwrap().draw(target);
        get_ui().get_element("market_button").unwrap().draw(target);
        get_ui().get_element("Money").unwrap().draw(target);
        get_ui().get_element("Level").unwrap().draw(target);
        get_ui().get_element("Points").unwrap().draw(target);
    }
}
