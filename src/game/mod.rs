pub mod pc_components;
mod mainmenu;
mod ingame;
mod save;
mod player_inventory;
mod pc;
mod inventory;
mod market;
mod pcbuilder;

use glfw::{Window, WindowEvent};
use crate::ui::Ui;
use self::{mainmenu::MainMenu, ingame::InGame, save::{Save, load_save, save_game}, inventory::Inventory, market::Market, pcbuilder::PCBuilder};

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
    pc_builder: Option<PCBuilder<'a>>,
    inventory: Option<Inventory<'a>>,
    market: Option<Market<'a>>
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
            pc_builder: None,
            inventory: None,
            market: None
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
                    GameState::PcBuilder => {
                        self.game_state = GameState::PcBuilder;
                        self.pc_builder = Some(PCBuilder::new(self.ui));
                        delete_ingame = true;
                    }
                    GameState::Inventory => {
                        self.game_state = GameState::Inventory;
                        self.inventory = Some(Inventory::new(&self.active_save, self.ui));
                        delete_ingame = true;
                    }
                    GameState::Market => {
                        self.game_state = GameState::Market;
                        self.market= Some(Market::new(self.ui));
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

    fn pc_builder(&mut self, window: &Window) {
        if self.game_state != GameState::PcBuilder {
            return;
        }

        let mut delete_pcbuilder = false;

        match &mut self.pc_builder {
            Some(pc_builder) => {
                if pc_builder.run(window, &mut self.active_save, self.ui) {
                    self.game_state = GameState::InGame;
                    self.ingame = Some(InGame::new(&self.active_save, self.ui));
                    delete_pcbuilder = true;
                }
            }
            None => {}
        }

        if delete_pcbuilder {
            self.pc_builder = None;
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

    fn market(&mut self, window: &Window) {
        if self.game_state != GameState::Market {
            return;
        }

        let mut delete_inventory = false;

        match &mut self.market {
            Some(market) => {
                if market.run(window, &mut self.active_save, self.ui) {
                    self.game_state = GameState::InGame;
                    self.ingame = Some(InGame::new(&self.active_save, self.ui));
                    delete_inventory = true;
                }
            }
            None => {}
        }

        if delete_inventory {
            self.market = None;
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
        self.pc_builder(window);
        self.inventory(window);
        self.market(window);
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

        match &self.pc_builder {
            Some(pc_builder) => {
                pc_builder.draw();
            }
            None => {}
        }

        match &self.inventory {
            Some(inventory) => {
                inventory.draw();
            }
            None => {}
        }

        match &self.market {
            Some(market) => {
                market.draw();
            }
            None => {}
        }
    }

    pub fn save(&self) {
        save_game(&self.active_save);
    }
}
