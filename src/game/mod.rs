pub mod pc_components;
pub mod pc;
pub mod inventory;
mod pcbuilder;
mod save;
mod market;

use imgui_glfw_rs::{glfw::{Window, self}, imgui::{Ui, self, ImStr, ImString}};
use crate::{rect::Rect, str_to_imstr, WINDOW_WIDTH, WINDOW_HEIGHT, f64_tuple_to_f32_array, components_list::*};

use self::{pcbuilder::PCBuilder, pc_components::{ram::Ram, storage_device::StorageDevice, fan::Fan}, pc::Pc, inventory::show_inventory, market::market};
pub use self::save::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    MainMenu,
    InGame,
    PcBuilder
}

pub struct Game {
    pub active_save: Save,
    game_state: GameState,
    show_inventory: bool,
    show_market: bool,
    show_contracts: bool,
    background: Rect,
    rects: Vec<Rect>,
    pc_builder: Option<PCBuilder>
}

impl Game {
    pub fn new() -> Self {
        Game {
            active_save: load_save("save1"),
            game_state: GameState::MainMenu,
            show_inventory: false,
            show_market: false,
            show_contracts: false,
            background: Rect::new(0, 0, 1920, 1080, "textures/background.png"),
            rects: Vec::new(),
            pc_builder: None
        }
    }

    fn main_menu(&mut self, window: &mut Window, ui: &Ui) {
        if self.game_state != GameState::MainMenu {
            return;
        }

        //MAIN MENU imgui window
        ui.window(str_to_imstr("main menu\0"))
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
            if ui.button(str_to_imstr("Play\0"), [470.0, 80.0]) {
                //set gamestate to ingame and change background
                self.game_state = GameState::InGame;
                self.background = Rect::new(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, "textures/ingame-background.png");
            }
            
            ui.new_line();ui.new_line();ui.new_line();

            //EXIT BUTTON
            if ui.button(str_to_imstr("Exit\0"), [470.0, 80.0]) {
                window.set_should_close(true);
            }
        });
    }

    fn ingame(&mut self, ui: &Ui) {
        if self.game_state != GameState::InGame {
            return;
        }

        //shows money, level and points
        ui.window(str_to_imstr("stats\0"))
        .movable(false)
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .build(|| {
            //add spacing in the imgui window
            for _ in 0..7 {
                ui.spacing();
            }
            ui.set_window_font_scale(2.4);
            ui.text(format!("$: {}  lvl: {}  points: {}/1000", self.active_save.money, self.active_save.level, self.active_save.points));
        });

        //buttons
        ui.window(str_to_imstr("buttons\0"))
        .movable(false)
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .build(|| {
            ui.set_window_font_scale(1.8);

            for _ in 0..2 {
                ui.new_line();
            }

            //get current cursor y pos of imgui so i can then set the cursor the cursor x pos at 116.5 without changing the y pos
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            //BUILD PC BUTTON
            if ui.button(str_to_imstr("Build PC\0"), [400.0, 120.0]) {
                //set gamestate and background
                self.game_state = GameState::PcBuilder;
                self.background = Rect::new(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, "textures/pc-builder.png");
                
                //create pc builder instance to hold all the necesasary 'global' variables
                self.pc_builder = Some(PCBuilder::new());
            }
            
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            //INVENTORY BUTTON
            if ui.button(str_to_imstr("Inventory\0"), [400.0, 120.0]) {
                self.show_inventory = true;
            }

            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);
            
            //MARKET BUTTON
            if ui.button(str_to_imstr("Market\0"), [400.0, 120.0]) {
                self.show_market = true;
            }
            
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            //CONTRACTS BUTTON
            if ui.button(str_to_imstr("Contracts\0"), [400.0, 120.0]) {
                self.show_contracts = true;
            }
        });
    }

    fn pc_builder(&mut self, window: &mut Window, ui: &Ui) {
        if self.game_state != GameState::PcBuilder {
            return;
        }

        let mut prev_comp_change = 0;
        let mut delete_pc_builder_instance = false;

        //match pc builder to borrow pc builder
        match self.pc_builder.as_mut() {
            Some(pc_builder) => {
                //
                // PART SELECTION UI
                //
                ui.window(str_to_imstr("pc builder inventory\0"))
                .title_bar(false)
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .build(|| {
                    //component change: add the values of all 'current_" (e.g pc_builder.current_case ) variables each frame and compare
                    // to the last frame to see if there was a change
                    prev_comp_change = pc_builder.comp_change;
                    pc_builder.comp_change = 0;

                    let mut cases: Vec<&ImStr> = Vec::new();
                    cases.push(str_to_imstr("None\0"));
                    for case in &self.active_save.inventory.cases {
                        match get_case_list().iter().find(|f| f.alias == *case) {
                            Some(item) => {
                                cases.push(str_to_imstr(item.name.as_str()));
                            }
                            None => {}
                        };
                    }
                    pc_builder.current_case = create_combo(str_to_imstr("Case\0"), pc_builder.current_case, cases.as_slice(), ui);
                    pc_builder.comp_change += pc_builder.current_case;

                    if pc_builder.current_case > 0 {
                        let max_fans = get_case_list().iter().find(|f| f.alias == self.active_save.inventory.cases[pc_builder.current_case as usize - 1]).unwrap().max_fans;
                        if max_fans != pc_builder.current_fans.len() as u32 {
                            loop {
                                if max_fans > pc_builder.current_fans.len() as u32 {
                                    pc_builder.current_fans.push(0);
                                    continue;
                                } else if max_fans < pc_builder.current_fans.len() as u32 {
                                    pc_builder.current_fans.pop();
                                    continue;
                                }
                                break;
                            }
                        }
                    } else if pc_builder.current_fans.len() as u32 > 0 {
                        pc_builder.current_fans.pop();
                    }

                    let mut motherboards: Vec<&ImStr> = Vec::new();
                    motherboards.push(str_to_imstr("None\0"));
                    for motherboard in &self.active_save.inventory.motherboards {
                        match get_motherboard_list().iter().find(|f| f.alias == *motherboard) {
                            Some(item) => {
                                motherboards.push(str_to_imstr(item.name.as_str()));
                            }
                            None => {}
                        }
                    }
                    pc_builder.current_motherboard = create_combo(str_to_imstr("Motherboard\0"), pc_builder.current_motherboard, motherboards.as_slice(), ui);
                    pc_builder.comp_change += pc_builder.current_motherboard;

                    if pc_builder.current_motherboard > 0 {
                        let max_rams = get_motherboard_list().iter().find(|f| f.alias == self.active_save.inventory.motherboards[pc_builder.current_motherboard as usize - 1]).unwrap().ram_slots;
                        if max_rams != pc_builder.current_rams.len() as u32 {
                            loop {
                                if max_rams > pc_builder.current_rams.len() as u32 {
                                    pc_builder.current_rams.push(0);
                                    continue;
                                } else if max_rams < pc_builder.current_rams.len() as u32 {
                                    pc_builder.current_rams.pop();
                                    continue;
                                }
                                break;
                            }
                        }
                    } else if pc_builder.current_rams.len() as u32 > 0 {
                        pc_builder.current_rams.pop();
                    }
                        
                    let mut cpus: Vec<&ImStr> = Vec::new();
                    cpus.push(str_to_imstr("None\0"));
                    for cpu in &self.active_save.inventory.cpus {
                        match get_cpu_list().iter().find(|f| f.alias == *cpu) {
                            Some(item) => {
                                cpus.push(str_to_imstr(item.name.as_str()));
                            }
                            None => {}
                        }
                    }
                    pc_builder.current_cpu = create_combo(str_to_imstr("CPU\0"), pc_builder.current_cpu, cpus.as_slice(), ui);
                    pc_builder.comp_change += pc_builder.current_cpu;

                    let mut cpu_coolers: Vec<&ImStr> = Vec::new();
                    cpu_coolers.push(str_to_imstr("None\0"));
                    for cpu_cooler in &self.active_save.inventory.cpu_coolers {
                        match get_cpu_cooler_list().iter().find(|f| f.alias == *cpu_cooler) {
                            Some(item) => {
                                cpu_coolers.push(str_to_imstr(item.name.as_str()));
                            }
                            None => {}
                        }
                    }
                    pc_builder.current_cpu_cooler = create_combo(str_to_imstr("CPU Cooler\0"), pc_builder.current_cpu_cooler, cpu_coolers.as_slice(), ui);
                    pc_builder.comp_change += pc_builder.current_cpu_cooler;

                    let mut i = 0;
                    for current_ram in &mut pc_builder.current_rams {
                        let mut rams: Vec<&ImStr> = Vec::new();
                        rams.push(str_to_imstr("None\0"));
                        for ram in &self.active_save.inventory.rams {
                            match get_ram_list().iter().find(|f| f.alias == *ram) {
                                Some(item) => {
                                    rams.push(str_to_imstr(item.name.as_str()));
                                }
                                None => {}
                            }
                        }
                        *current_ram = create_combo(str_to_imstr(format!("RAM {}\0", i).as_str()), *current_ram, rams.as_slice(), ui);
                        pc_builder.comp_change += *current_ram;
                        i += 1;
                    }

                    let mut gpus: Vec<&ImStr> = Vec::new();
                    gpus.push(str_to_imstr("None\0"));
                    for gpu in &self.active_save.inventory.gpus {
                        match get_gpu_list().iter().find(|f| f.alias == *gpu) {
                            Some(item) => {
                                gpus.push(str_to_imstr(item.name.as_str()));
                            }
                            None => {}
                        }
                    }
                    pc_builder.current_gpu = create_combo(str_to_imstr("GPU\0"), pc_builder.current_gpu, gpus.as_slice(), ui);
                    pc_builder.comp_change += pc_builder.current_gpu;

                    let mut i = 0;
                    for current_storage in &mut pc_builder.current_storages {
                        let mut storages: Vec<&ImStr> = Vec::new();
                        storages.push(str_to_imstr("None\0"));
                        for storage in &self.active_save.inventory.storages {
                            match get_storage_list().iter().find(|f| f.alias == *storage) {
                                Some(item) => {
                                    storages.push(str_to_imstr(item.name.as_str()));
                                }
                                None => {}
                            }
                        }
                        *current_storage = create_combo(str_to_imstr(format!("Storage {}\0", i).as_str()), *current_storage, storages.as_slice(), ui);
                        pc_builder.comp_change += *current_storage;
                        i += 1;
                    }

                    let mut i = 0;
                    for current_fan in &mut pc_builder.current_fans {
                        let mut fans: Vec<&ImStr> = Vec::new();
                        fans.push(str_to_imstr("None\0"));
                        for fan in &self.active_save.inventory.fans {
                            match get_fan_list().iter().find(|f| f.alias == *fan) {
                                Some(item) => {
                                    fans.push(str_to_imstr(item.name.as_str()));
                                }
                                None => {}
                            }
                        }
                        *current_fan = create_combo(str_to_imstr(format!("Fan {}\0", i).as_str()), *current_fan, fans.as_slice(), ui);
                        pc_builder.comp_change += *current_fan;
                        i += 1;
                    }

                    let mut power_supplys: Vec<&ImStr> = Vec::new();
                    power_supplys.push(str_to_imstr("None\0"));
                    for power_supply in &self.active_save.inventory.power_supplys {
                        match get_power_supply_list().iter().find(|f| f.alias == *power_supply) {
                            Some(item) => {
                                power_supplys.push(str_to_imstr(item.name.as_str()));
                            }
                            None => {}
                        }
                    }
                    pc_builder.current_power_supply = create_combo(str_to_imstr("Power Supply\0"), pc_builder.current_power_supply, power_supplys.as_slice(), ui);
                    pc_builder.comp_change = pc_builder.current_power_supply;

                    let mut missing_parts = String::new();

                    let case = if pc_builder.current_case > 0 {
                        Some(get_case_list().iter().find(|f| f.alias == self.active_save.inventory.cases[pc_builder.current_case as usize - 1]).unwrap().clone())
                    } else {
                        missing_parts.push_str("no case\n");
                        None
                    };

                    let motherboard = if pc_builder.current_motherboard > 0 {
                        Some(get_motherboard_list().iter().find(|f| f.alias == self.active_save.inventory.motherboards[pc_builder.current_motherboard as usize - 1]).unwrap().clone())
                    } else {
                        missing_parts.push_str("no motherboard\n");
                        None
                    };

                    let cpu = if pc_builder.current_cpu > 0 {
                        Some(get_cpu_list().iter().find(|f| f.alias == self.active_save.inventory.cpus[pc_builder.current_cpu as usize - 1]).unwrap().clone())
                    } else {
                        missing_parts.push_str("no cpu\n");
                        None
                    };

                    let cpu_cooler = if pc_builder.current_cpu_cooler > 0 {
                        Some(get_cpu_cooler_list().iter().find(|f| f.alias == self.active_save.inventory.cpu_coolers[pc_builder.current_cpu_cooler as usize - 1]).unwrap().clone())
                    } else {
                        missing_parts.push_str("no cpu cooler\n");
                        None
                    };

                    let mut slots_filled = 0;
                    let mut ram = Vec::new();
                    for current_ram in &pc_builder.current_rams {
                        if *current_ram > 0 {
                            ram.push(get_ram_list().iter().find(|f| f.alias == self.active_save.inventory.rams[*current_ram as usize - 1]).unwrap().clone());
                            slots_filled += 1;
                        }
                    }
                    if self.active_save.inventory.rams.len() < slots_filled {
                        missing_parts.push_str("selected more ram than there is in inventory\n");
                    }

                    let gpu = if pc_builder.current_gpu > 0 {
                        Some(get_gpu_list().iter().find(|f| f.alias == self.active_save.inventory.gpus[pc_builder.current_gpu as usize - 1]).unwrap().clone())
                    } else {
                        missing_parts.push_str("no gpu\n");
                        None
                    };

                    let mut storage_devices = 0;
                    let mut storage = Vec::new();
                    for current_storage in &pc_builder.current_storages {
                        if *current_storage > 0 {
                            storage.push(get_storage_list().iter().find(|f| f.alias == self.active_save.inventory.storages[*current_storage as usize - 1]).unwrap().clone());
                            storage_devices += 1;
                        }
                    }
                    if self.active_save.inventory.storages.len() < storage_devices {
                        missing_parts.push_str("selected more storage devices than there is in inventory\n");
                    }

                    let mut fans_used = 0;
                    let mut fan = Vec::new();
                    for current_fan in &pc_builder.current_fans {
                        if *current_fan > 0 {
                            fan.push(get_fan_list().iter().find(|f| f.alias == self.active_save.inventory.fans[*current_fan as usize - 1]).unwrap().clone());
                            fans_used += 1;
                        }
                    }
                    if self.active_save.inventory.fans.len() < fans_used {
                        missing_parts.push_str("selected more fans than there is in inventory\n");
                    }

                    let power_supply = if pc_builder.current_power_supply > 0 {
                        Some(get_power_supply_list().iter().find(|f| f.alias == self.active_save.inventory.power_supplys[pc_builder.current_power_supply as usize - 1]).unwrap().clone())
                    } else {
                        missing_parts.push_str("no power supply\n");
                        None
                    };

                    if missing_parts.len() == 0 {
                        let pc = Pc {
                            case: case.unwrap(),
                            motherboard: motherboard.unwrap(),
                            cpu: cpu.unwrap(),
                            cpu_cooler: cpu_cooler.unwrap(),
                            ram,
                            gpu: gpu.unwrap(),
                            storage,
                            fans: fan,
                            power_supply: power_supply.unwrap(),
                            computing_score: 0,
                            graphics_score: 0,
                            total_score: 0
                        };
                        pc_builder.pc_to_sell = Some(pc);
                        
                        //CHECK COMPATABILITY BUTTON
                        if ui.button(str_to_imstr("Check Compatability\0"), [300.0, 80.0]) {
                            if !pc_builder.compatability_window {
                                pc_builder.compatability_window = true;
                            }
                        }
                    } else {
                        for issue in missing_parts.split("\n") {
                            ui.text(issue);
                        }
                    }

                    //SELL BUTTON
                    //only appears when the pc actually works
                    if pc_builder.pc_works {
                        if ui.button(str_to_imstr("Sell\0"), [300.0, 80.0]) {
                            if !pc_builder.sell_window {
                                pc_builder.sell_window = true;
                            }
                        }
                    }
                });

                //COMPATABILITY WINDOW LOGIC
                if pc_builder.compatability_window {
                    ui.window(str_to_imstr("Compatability\0"))
                    .build(|| {
                        let pc = pc_builder.pc_to_sell.as_mut().unwrap();
                        
                        match pc.check_compatability() {
                            Ok(good) => {
                                ui.text(good);
                                pc.calculate_score();
                                ui.text(format!("computing score - {}", pc.computing_score));
                                ui.text(format!("graphics score - {}", pc.graphics_score));
                                ui.text(format!("total score - {}", pc.total_score));
                                pc_builder.pc_works = true;
                                pc_builder.compatability_window = false;
                            }
                            Err(bad) => {
                                ui.text(bad);
                            }
                        }

                        ui.same_line(60.0);

                        //COMPATABILITY WINDOW CLOSE BUTTON
                        if ui.button(str_to_imstr("Close\0"), [40.0, 20.0]) {
                            pc_builder.compatability_window = false;
                        }
                    });
                }

                //if the previous component 'hash' is not the same as the current one
                //then set a few attributes to false or none
                if prev_comp_change != pc_builder.comp_change {
                    pc_builder.sell_window = false;
                    pc_builder.pc_works = false;
                    pc_builder.pc_to_sell = None;
                }

                //SELL WINDOW LOGIC
                if pc_builder.sell_window {
                    //SELL WINDOW
                    ui.window(str_to_imstr("Sell\0"))
                    .build(|| {
                        let pc_worth = pc_builder.pc_to_sell.as_ref().unwrap().get_price();
                        ui.text(format!("pc worth - {}", pc_worth));
                        ui.input_text(str_to_imstr("Price\0"), &mut pc_builder.pc_input_price).build();

                        //SELL WINDOW SELL BUTTON
                        if ui.button(str_to_imstr("Sell\0"), [40.0, 20.0]) {
                            pc_builder.offer_window = true;
                            pc_builder.sell_window = false;
                            pc_builder.pc_final_price = pc_builder.pc_input_price.to_str().trim().parse().unwrap();
                        }

                        ui.same_line(60.0);
                        //SELL WINDOW EXIT BUTTON
                        if ui.button(str_to_imstr("Exit\0"), [40.0, 20.0]) {
                            pc_builder.sell_window = false;
                        }
                    });
                }

                //OFFER WINDOW LOGIC
                if pc_builder.offer_window {
                    ui.window(str_to_imstr("Offers\0")).build(|| {

                        let pc = pc_builder.pc_to_sell.as_ref().unwrap();
                        
                        let overcharge = pc_builder.pc_final_price - pc.get_price() as i32;
                        
                        //CALCULATE OFFERS
                        let offers_amount = if overcharge.is_negative() {
                            8
                        } else if overcharge > 700 {
                            0
                        } else if overcharge > 500 {
                            1
                        } else if overcharge > 400 {
                            2
                        } else if overcharge > 300 {
                            3
                        } else if overcharge > 200 {
                            4
                        } else if overcharge > 1 {
                            5
                        } else {
                            println!("hit else block in offers");
                            0
                        };
                        
                        
                        if pc_builder.offers.len() == 0 {
                            for _ in 0..offers_amount {
                                let small_offset = rand::random::<i32>() % 240;
                                let big_offset: i32 = offers_amount * 50;
                                let offset = big_offset + small_offset;
                                let offer = pc_builder.pc_final_price - offset;
                                pc_builder.offers.push(offer);
                            }
                        }
                        
                        ui.text(format!("offer - {}", pc_builder.offers[pc_builder.offers_index as usize]));
                        
                        if ui.button(str_to_imstr("<\0"), [15.0, 15.0]) {
                            if pc_builder.offers_index > 0 {
                                pc_builder.offers_index -= 1;
                            }
                        }
                    
                        ui.same_line(40.0);
                    
                        if ui.button(str_to_imstr(">\0"), [15.0, 15.0]) {
                            if pc_builder.offers_index < pc_builder.offers.len() as i32 - 1 {
                                pc_builder.offers_index += 1;
                            }
                        }
                        
                        //OFFER WINDOW SELL BUTTON
                        if ui.button(str_to_imstr("Sell\0"), [40.0, 20.0]) {
                            self.active_save.inventory.cases.remove(pc_builder.current_case as usize - 1);
                            self.active_save.inventory.motherboards.remove(pc_builder.current_motherboard as usize - 1);
                            self.active_save.inventory.cpus.remove(pc_builder.current_cpu as usize - 1);
                            self.active_save.inventory.cpu_coolers.remove(pc_builder.current_case as usize - 1);
                            
                            let mut prev_ram = 0;
                            let mut offset = 0;
                            for ram in &pc_builder.current_rams {
                                if *ram > 0 {
                                    
                                    if prev_ram == 0 {
                                        prev_ram = *ram;
                                    } else {
                                        if *ram >= prev_ram {
                                            offset += 1;
                                        } else if *ram < prev_ram {
                                            offset = 0;
                                        }
                                    }
                                    self.active_save.inventory.rams.remove(*ram as usize - 1 - offset);
                                }
                            }

                            self.active_save.inventory.gpus.remove(pc_builder.current_gpu as usize - 1);
                            
                           let mut prev_storage = 0; 
                           let mut offset = 0;
                            for storage in &pc_builder.current_storages {
                                if *storage > 0 {
                                    
                                    if prev_storage == 0 {
                                        prev_storage = *storage;
                                    } else {
                                        if *storage >= prev_ram {
                                            offset += 1;
                                        } else if *storage < prev_ram {
                                            offset = 0;
                                        }
                                    }

                                    self.active_save.inventory.storages.remove(*storage as usize - 1 - offset);
                                }
                            }

                            let mut prev_fan = 0;
                            let mut offset = 0;
                            for fan in &pc_builder.current_fans {
                                if *fan > 0 {
                                    
                                    if prev_fan == 0 {
                                        prev_fan = *fan;
                                    } else {
                                        if *fan >= prev_ram {
                                            offset += 1;
                                        } else if *fan < prev_ram {
                                            offset = 0;
                                        }
                                    }
                                    self.active_save.inventory.fans.remove(*fan as usize - 1 - offset);
                                }
                            }
                            self.active_save.inventory.power_supplys.remove(pc_builder.current_power_supply as usize - 1);

                            self.active_save.money += pc_builder.offers[pc_builder.offers_index as usize] as i32;
                            self.game_state = GameState::InGame;
                            self.background = Rect::new(0, 0, 1920, 1080, "textures/ingame-background.png");
                            delete_pc_builder_instance = true;
                        }
                    });
                }

                if pc_builder.back_button.contains(f64_tuple_to_f32_array(window.get_cursor_pos())) && window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button1) == glfw::Action::Press{
                    self.game_state = GameState::InGame;
                    self.pc_builder = None;
                    self.background = Rect::new(0, 0, 1920, 1080, "textures/ingame-background.png");
                }
            }
            None => {}
        }

        if delete_pc_builder_instance {
            self.pc_builder = None;
        }
    }

    fn inventory(&mut self, ui: &Ui, window: &Window) {
        if !self.show_inventory {
            return;
        }

        self.show_inventory = show_inventory(&self.active_save.inventory, window, ui);
    }

    fn market(&mut self, ui: &Ui, window: &Window) {
        if !self.show_market {
            return;
        }

        self.show_market = market(&mut self.active_save, ui, window);
    }

    fn contract(&mut self, ui: &Ui) {
        if !self.show_contracts {
            return;
        }

        ui.window(str_to_imstr("Coming soon\0"))
        .build(|| {
            ui.text("Coming Soon!");
            if ui.button(str_to_imstr("Back\0"), [40.0, 20.0]) {
                self.show_contracts = false;
            }
        });
    }

    pub fn run(&mut self, window: &mut Window, ui: &Ui) {
        self.background.draw();
        
        for rect in &self.rects {
            rect.draw();
        }

        self.main_menu(window, ui);
        self.ingame(ui);
        self.pc_builder(window, ui);
        self.inventory(ui, window);
        self.market(ui, window);
        self.contract(ui);
    }
}

fn create_combo(name: &ImStr, mut current_item: i32, items: &[&ImStr], ui: &Ui) -> i32 {
    ui.combo(name, &mut current_item, items, 4);
    current_item
}
