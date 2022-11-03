mod mainmenu;
mod ingame;
mod save;
mod player_inventory;
mod pc;
mod pc_components;
mod inventory;

use glfw::{Window, WindowEvent};
use crate::{gfx::{color_rect::ColorRect, image_rect::ImageRect, vectors::{vec2::vec2, vec3::vec3}, text::Text, rect::Rect}, ui::Ui};
use self::{mainmenu::MainMenu, ingame::InGame, save::{Save, load_save, save_game}, player_inventory::PlayerInventory, inventory::Inventory};

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    MainMenu,
    InGame,
    PcBuilder,
    Inventory,
    Market,
    Contracts
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scroll {
    Up,
    Down
}

#[derive(Debug)]
pub struct Game<'a> {
    active_save: Save,
    game_state: GameState,
    ui: &'a Ui<'a>,
    pub scrolls: Vec<Scroll>,
    mainmenu: Option<MainMenu<'a>>,
    ingame: Option<InGame<'a>>,
    inventory: Option<Inventory<'a>>
}

impl<'a> Game<'a> {
    pub fn new(ui: &'a Ui) -> Game<'a> {
        Game {
            active_save: load_save("save1"),
            game_state: GameState::MainMenu,
            ui,
            scrolls: Vec::new(),
            mainmenu: Some(MainMenu::new(ui)),
            ingame: None,
            inventory: None
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

        let mut delete_ingame = false;

        match &mut self.ingame {
            Some(ingame) => {
                match ingame.run(window) {
                    GameState::PcBuilder => {}
                    GameState::Inventory => {
                        self.game_state = GameState::Inventory;
                        self.inventory = Some(Inventory::new(&self.active_save, self.ui));
                        delete_ingame = true;
                    }
                    _ => {}
                }
            }
            None => {}
        }

        if delete_ingame {
            self.ingame = None;
        }
    }

    fn inventory(&mut self, window: &Window) {
        if self.game_state != GameState::Inventory {
            return;
        }

        let mut delete_inventory = false;

        match &mut self.inventory {
            Some(inventory) => {
                if inventory.run(window, &mut self.scrolls, self.ui) {
                    self.game_state = GameState::InGame;
                    self.ingame = Some(InGame::new(&self.active_save, self.ui));
                    delete_inventory = true;
                }
            }
            None => {}
        }

        if delete_inventory {
            self.inventory = None;
        }
    }

    pub fn scroll(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::Scroll(_, y) => {
                if y > 0.0 {
                    self.scrolls.push(Scroll::Up);
                } else if y < 0.0 {
                    self.scrolls.push(Scroll::Down);
                }
            }
            _ => {
                self.scrolls.pop();
            }
        }
    }

    pub fn run(&mut self, window: &mut Window) {
        self.mainmenu(window);
        self.ingame(window);
        self.inventory(window);
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

        match &self.inventory {
            Some(inventory) => {
                inventory.draw();
            }
            None => {}
        }
    }
}
