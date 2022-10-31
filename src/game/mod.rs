mod mainmenu;
mod ingame;
mod save;
mod inventory;
mod pc;
mod pc_components;

use glfw::Window;
use crate::{gfx::{color_rect::ColorRect, image_rect::ImageRect, vectors::{vec2::vec2, vec3::vec3}, text::Text, rect::Rect}, ui::Ui};
use self::{mainmenu::MainMenu, ingame::InGame, save::{Save, load_save}};

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    MainMenu,
    InGame,
    PcBuilder,
    Inventory,
    Market,
    Contracts
}

#[derive(Debug)]
pub struct Game<'a> {
    active_save: Save,
    game_state: GameState,
    ui: &'a Ui<'a>,
    mainmenu: Option<MainMenu<'a>>,
    ingame: Option<InGame<'a>>
}

impl<'a> Game<'a> {
    pub fn new(ui: &'a Ui) -> Game<'a> {
        Game {
            active_save: load_save("save1"),
            game_state: GameState::MainMenu,
            ui,
            mainmenu: Some(MainMenu::new(ui)),
            ingame: None
        }
    }

    fn mainmenu(&mut self, window: &mut Window) {
        if self.game_state != GameState::MainMenu {
            return;
        }

        let mut delete_mainmenu = false;

        match &mut self.mainmenu {
            Some(mainmenu) => {
                if mainmenu.run(window) {
                    self.game_state = GameState::InGame;
                    delete_mainmenu = true;

                    self.ingame = Some(InGame::new(&self.active_save, self.ui));
                }
            }
            None => {}
        }

        if delete_mainmenu {
            self.mainmenu = None;
        }
    }

    fn ingame(&mut self, window: &Window) {
        if self.game_state != GameState::InGame {
            return;
        }

        match &mut self.ingame {
            Some(ingame) => {
                match ingame.run(window) {
                    GameState::PcBuilder => {}
                    GameState::Inventory => {
                        self.game_state = GameState::Inventory;
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }

    pub fn run(&mut self, window: &mut Window) {
        self.mainmenu(window);
        self.ingame(window);
    }

    pub fn draw(&self) {
        match &self.mainmenu {
            Some(mainmenu) => {
                mainmenu.draw();
            }
            None => {}
        }

        match &self.ingame {
            Some(ingame) => {
                ingame.draw();
            }
            None => {}
        }
    }
}
