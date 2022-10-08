pub mod pc_components;
mod save;
mod pcbuilder;
mod pc;

use imgui_glfw_rs::{glfw::{Window, self}, imgui::{Ui, self}};
use crate::{rect::Rect, str_to_imstr, WINDOW_WIDTH, WINDOW_HEIGHT, f64_array_to_f32};

use self::pcbuilder::PCBuilder;
pub use self::save::*;

#[derive(Debug, Clone, PartialEq)]
enum GameState {
    MainMenu,
    InGame,
    PcBuilder,
    Inventory,
    Market,
    Research
}

pub struct Game {
    pub active_save: Save,
    game_state: GameState,
    background: Rect,
    rects: Vec<Rect>,
    pc_builder: Option<PCBuilder>
}

impl Game {
    pub fn new() -> Self {
        Game {
            active_save: Save {
                name: "save1".to_string(),
                money: 1500,
                level: 1,
                points: 0
            },
            game_state: GameState::MainMenu,
            background: Rect::new(0, 0, 1920, 1080, "textures/background.png"),
            rects: Vec::new(),
            pc_builder: None
        }
    }

    fn main_menu(&mut self, window: &mut Window, ui: &Ui) {
        if self.game_state != GameState::MainMenu {
            return;
        }
        ui.window(str_to_imstr("main menu"))
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .movable(false)
        .position([133.0, 200.0], imgui::Condition::Always)
        .size([490.0, 900.0], imgui::Condition::Always)
        .build(|| {
            //SPACING BECAUSE BEAUTY
            for _ in 0..20 {
                ui.new_line();
            }
            if ui.button(str_to_imstr("Play"), [470.0, 80.0]) {
                //set gamestate to ingame and change background
                self.game_state = GameState::InGame;
                self.background = Rect::new(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, "textures/ingame-background.png");
            }
            
            ui.new_line();ui.new_line();ui.new_line();

            if ui.button(str_to_imstr("Exit"), [470.0, 80.0]) {
                window.set_should_close(true);
            }
        });
    }

    fn ingame(&mut self, ui: &Ui) {
        if self.game_state != GameState::InGame {
            return;
        }

        //shows money, level and points
        ui.window(str_to_imstr("stats"))
        .movable(false)
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .build(|| {
            for _ in 0..7 {
                ui.spacing();
            }
            ui.set_window_font_scale(2.4);
            ui.text(format!("$: {}  lvl: {}  points: {}/1000", self.active_save.money, self.active_save.level, self.active_save.points));
        });

        //buttons
        ui.window(str_to_imstr("buttons"))
        .movable(false)
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .build(|| {
            ui.set_window_font_scale(1.8);
            //set cursor pos x - 116.5
            //button width - 400

            for _ in 0..2 {
                ui.new_line();
            }

            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            if ui.button(str_to_imstr("Build PC"), [400.0, 120.0]) {
                self.game_state = GameState::PcBuilder;
                self.background = Rect::new(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, "textures/pc-builder.png");
                self.pc_builder = Some(PCBuilder {
                    back_button: Rect::new(1751, 0, 229, 96, "textures/transparent.png")
                });
            }
            
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            if ui.button(str_to_imstr("Inventory"), [400.0, 120.0]) {
                self.game_state = GameState::Inventory;
            }

            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            if ui.button(str_to_imstr("Market"), [400.0, 120.0]) {
                self.game_state = GameState::Market;
            }
            
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            if ui.button(str_to_imstr("Research"), [400.0, 120.0]) {
                self.game_state = GameState::Research;
            }
        });
    }

    fn pc_builder(&mut self, window: &mut Window) {
        if self.game_state != GameState::PcBuilder {
            return;
        }

        if self.pc_builder.is_some() {
            self.pc_builder.as_ref().unwrap().back_button.draw();
            if self.pc_builder.as_ref().unwrap().back_button.contains(f64_array_to_f32(window.get_cursor_pos())) && window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button1) == glfw::Action::Press{
                self.game_state = GameState::InGame;
                self.pc_builder = None;
                self.background = Rect::new(0, 0, 1920, 1080, "textures/ingame-background.png");
            }
        }
    }

    pub fn run(&mut self, window: &mut Window, ui: &Ui) {
        self.background.draw();
        
        for rect in &self.rects {
            rect.draw();
        }

        self.main_menu(window, ui);
        self.ingame(ui);
        self.pc_builder(window);
    }
}
