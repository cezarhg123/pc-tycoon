pub mod pc_components;
pub mod pc;
mod pcbuilder;
mod save;

use imgui_glfw_rs::{glfw::{Window, self}, imgui::{Ui, self, ImStr, ImString}};
use crate::{rect::Rect, str_to_imstr, WINDOW_WIDTH, WINDOW_HEIGHT, f64_tuple_to_f32_array, components_list::*};

use self::{pcbuilder::PCBuilder, pc_components::{ram::Ram, storage_device::StorageDevice, fan::Fan}, pc::Pc};
pub use self::save::*;

#[derive(Debug, Clone, PartialEq)]
enum GameState {
    MainMenu,
    InGame,
    PcBuilder,
    Inventory,
    Market,
    Contract
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

                //create pc builder instance to hold all the necesasary variables
                self.pc_builder = Some(PCBuilder {
                    back_button: Rect::new(1751, 0, 229, 96, "textures/transparent.png"),
                    current_case: 0,
                    current_motherboard: 0,
                    current_cpu: 0,
                    current_cpu_cooler: 0,
                    current_rams: Vec::new(),
                    current_gpu: 0,
                    current_storages: [0; 6],
                    current_fans: Vec::new(),
                    current_power_supply: 0,
                    compatability_window: false,
                    pc_works: false,
                    sell_window: false,
                    comp_change: 0,
                    pc_to_sell: None,
                    pc_input_price: ImString::new(""),
                    offer_window: false,
                    pc_final_price: 0,
                    offers: Vec::new(),
                    offers_index: 0
                });
            }
            
            //read line 112
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            //INVENTORY BUTTON
            if ui.button(str_to_imstr("Inventory\0"), [400.0, 120.0]) {
                self.game_state = GameState::Inventory;
            }

            //read line 112
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);
            
            //MARKET BUTTON
            if ui.button(str_to_imstr("Market\0"), [400.0, 120.0]) {
                self.game_state = GameState::Market;
            }
            
            //read line 112
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            //CONTRACTS BUTTON
            if ui.button(str_to_imstr("Contracts\0"), [400.0, 120.0]) {
                self.game_state = GameState::Contract;
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

                    //create vector of imgui strings
                    let mut cases: Vec<&ImStr> = Vec::new();
                    //push none so its the first one that appears in the dropdown
                    cases.push(str_to_imstr("none\0"));
                    //push all case names to the vector
                    for case in get_case_list() {
                        cases.push(str_to_imstr(case.name.as_str()));
                    }
                    //actually create dropdown (imgui calls it a combo)
                    pc_builder.current_case = create_combo(
                        str_to_imstr("Case\0"),
                        pc_builder.current_case,
                        cases.as_slice(),
                        ui
                    );
                    
                    pc_builder.comp_change += pc_builder.current_case;

                    if cases.len() > 1 && pc_builder.current_case > 0 {
                        if get_case_list()[pc_builder.current_case as usize - 1].max_fans != pc_builder.current_fans.len() as u32 {
                            loop {
                                if get_case_list()[pc_builder.current_case as usize - 1].max_fans > pc_builder.current_fans.len() as u32 {
                                    pc_builder.current_fans.push(0);
                                    continue;
                                } else if get_case_list()[pc_builder.current_case as usize - 1].max_fans < pc_builder.current_fans.len() as u32 {
                                    pc_builder.current_fans.pop();
                                    continue;
                                }
                                break;
                            }
                        }
                    } else if pc_builder.current_fans.len() > 0 {
                        pc_builder.current_fans.pop();
                    }

                    let mut motherboards: Vec<&ImStr> = Vec::new();
                    motherboards.push(str_to_imstr("none\0"));
                    for motherboard in get_motherboard_list() {
                        motherboards.push(str_to_imstr(motherboard.name.as_str()));
                    }
                    pc_builder.current_motherboard = create_combo(
                        str_to_imstr("Motherboard\0"),
                        pc_builder.current_motherboard,
                        motherboards.as_slice(),
                        ui
                    );

                    pc_builder.comp_change += pc_builder.current_motherboard;

                    if motherboards.len() > 1 && pc_builder.current_motherboard > 0 {
                        if get_motherboard_list()[pc_builder.current_motherboard as usize - 1].ram_slots != pc_builder.current_rams.len() as u32 {
                            loop {
                                if get_motherboard_list()[pc_builder.current_motherboard as usize - 1].ram_slots > pc_builder.current_rams.len() as u32 {
                                    pc_builder.current_rams.push(0);
                                    continue;
                                } else if get_motherboard_list()[pc_builder.current_motherboard as usize - 1].ram_slots < pc_builder.current_rams.len() as u32 {
                                    pc_builder.current_rams.pop();
                                    continue;
                                }
                                break;
                            }
                        }
                    } else if pc_builder.current_rams.len() > 0 {
                        pc_builder.current_rams.pop();
                    }

                    let mut cpus: Vec<&ImStr> = Vec::new();
                    cpus.push(str_to_imstr("none\0"));
                    for cpu in get_cpu_list() {
                        cpus.push(str_to_imstr(cpu.name.as_str()));
                    }
                    pc_builder.current_cpu = create_combo(
                        str_to_imstr("CPU\0"),
                        pc_builder.current_cpu,
                        cpus.as_slice(),
                        ui
                    );

                    pc_builder.comp_change += pc_builder.current_cpu;

                    let mut cpu_coolers: Vec<&ImStr> = Vec::new();
                    cpu_coolers.push(str_to_imstr("none\0"));
                    for cpu_cooler in get_cpu_cooler_list() {
                        cpu_coolers.push(str_to_imstr(cpu_cooler.name.as_str()));
                    }
                    pc_builder.current_cpu_cooler = create_combo(
                        str_to_imstr("CPU Cooler\0"),
                        pc_builder.current_cpu_cooler,
                        cpu_coolers.as_slice(),
                        ui
                    );

                    pc_builder.comp_change += pc_builder.current_cpu_cooler;

                    let mut i = 1;
                    for current_ram in &mut pc_builder.current_rams {
                        let mut rams: Vec<&ImStr> = Vec::new();
                        rams.push(str_to_imstr("none\0"));
                        for ram in get_ram_list() {
                            rams.push(str_to_imstr(ram.name.as_str()));
                        }
                        *current_ram = create_combo(
                            str_to_imstr(format!("Ram slot {}\0", i).as_str()),
                            *current_ram,
                            rams.as_slice(),
                            ui
                        );
                        i += 1;

                        pc_builder.comp_change += *current_ram;
                    }

                    let mut gpus: Vec<&ImStr> = Vec::new();
                    gpus.push(str_to_imstr("none\0"));
                    for gpu in get_gpu_list() {
                        gpus.push(str_to_imstr(gpu.name.as_str()));
                    }
                    pc_builder.current_gpu = create_combo(
                        str_to_imstr("GPU\0"),
                        pc_builder.current_gpu,
                        gpus.as_slice(),
                        ui
                    );

                    pc_builder.comp_change += pc_builder.current_gpu;

                    let mut i = 1;
                    for current_storage in &mut pc_builder.current_storages {
                        let mut storages: Vec<&ImStr> = Vec::new();
                        storages.push(str_to_imstr("none\0"));
                        for storage in get_storage_list() {
                            storages.push(str_to_imstr(storage.name.as_str()));
                        }
                        *current_storage = create_combo(
                            str_to_imstr(format!("Storage {}\0", i).as_str()),
                            *current_storage,
                            storages.as_slice(),
                            ui
                        );
                        i += 1;
                        pc_builder.comp_change += *current_storage;
                    }

                    let mut i = 1;
                    for current_fan in &mut pc_builder.current_fans {
                        let mut fans: Vec<&ImStr> = Vec::new();
                        fans.push(str_to_imstr("none\0"));
                        for fan in get_fan_list() {
                            fans.push(str_to_imstr(fan.name.as_str()));
                        }
                        *current_fan = create_combo(
                            str_to_imstr(format!("Fan {}\0", i).as_str()),
                            *current_fan,
                            fans.as_slice(),
                            ui
                        );
                        i += 1;
                        pc_builder.comp_change += *current_fan;
                    }

                    let mut power_supplys: Vec<&ImStr> = Vec::new();
                    power_supplys.push(str_to_imstr("none\0"));
                    for power_supply in get_power_supply_list() {
                        power_supplys.push(str_to_imstr(power_supply.name.as_str()));
                    }
                    pc_builder.current_power_supply = create_combo(
                        str_to_imstr("Power Supply\0"),
                        pc_builder.current_power_supply,
                        power_supplys.as_slice(),
                        ui
                    );

                    pc_builder.comp_change += pc_builder.current_power_supply;
                    
                    //CHECK COMPATABILITY BUTTON
                    if ui.button(str_to_imstr("Check Compatability\0"), [300.0, 80.0]) {
                        if !pc_builder.compatability_window {
                            pc_builder.compatability_window = true;
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
                        let mut missing_parts = String::new();

                        let case = if pc_builder.current_case > 0 {
                            Some(get_case_list()[pc_builder.current_case as usize - 1].clone())
                        } else {
                            missing_parts.push_str("no case\n");
                            None
                        };

                        let motherboard = if pc_builder.current_motherboard > 0 {
                            Some(get_motherboard_list()[pc_builder.current_case as usize - 1].clone())
                        } else {
                            missing_parts.push_str("no motherboard\n");
                            None
                        };

                        let cpu = if pc_builder.current_cpu > 0 {
                            Some(get_cpu_list()[pc_builder.current_cpu as usize - 1].clone())
                        } else {
                            missing_parts.push_str("no cpu\n");
                            None
                        };

                        let cpu_cooler = if pc_builder.current_cpu_cooler > 0 {
                            Some(get_cpu_cooler_list()[pc_builder.current_cpu_cooler as usize - 1].clone())
                        } else {
                            missing_parts.push_str("no cpu cooler\n");
                            None
                        };

                        let gpu = if pc_builder.current_gpu > 0 {
                            Some(get_gpu_list()[pc_builder.current_gpu as usize - 1].clone())
                        } else {
                            missing_parts.push_str("no gpu\n");
                            None
                        };

                        let power_supply = if pc_builder.current_power_supply > 0 {
                            Some(get_power_supply_list()[pc_builder.current_power_supply as usize - 1].clone())
                        } else {
                            missing_parts.push_str("no power supply\n");
                            None
                        };

                        let mut ram: Vec<Ram> = Vec::new();
                        for ram_i in &pc_builder.current_rams {
                            if *ram_i > 0 {
                                ram.push(get_ram_list()[*ram_i as usize - 1].clone());
                            }
                        }

                        let mut storage: Vec<StorageDevice> = Vec::new();
                        for storage_i in &pc_builder.current_storages {
                            if *storage_i > 0 {
                                storage.push(get_storage_list()[*storage_i as usize - 1].clone());
                            }
                        }

                        let mut fans: Vec<Fan> = Vec::new();
                        for fan_i in &pc_builder.current_fans {
                            if *fan_i > 0 {
                                fans.push(get_fan_list()[*fan_i as usize - 1].clone());
                            }
                        }

                        if missing_parts.len() == 0 {
                            let mut pc = Pc {
                                case: case.unwrap(),
                                motherboard: motherboard.unwrap(),
                                cpu: cpu.unwrap(),
                                cpu_cooler: cpu_cooler.unwrap(),
                                ram,
                                gpu: gpu.unwrap(),
                                storage,
                                fans,
                                power_supply: power_supply.unwrap(),
                                computing_score: 0,
                                graphics_score: 0,
                                total_score: 0
                            };

                            match pc.check_compatability() {
                                Ok(good) => {
                                    pc_builder.pc_works = true;
                                    pc_builder.pc_to_sell = Some(pc.clone());
                                    ui.text(good);
                                    pc.calculate_score();
                                    ui.text(format!("computing score - {}", pc.computing_score));
                                    ui.text(format!("graphics score - {}", pc.graphics_score));
                                    ui.text(format!("total score - {}", pc.total_score));
                                }
                                Err(bad) => {
                                    for issue in bad.split("\n") {
                                        ui.text(issue);
                                    }
                                }
                            }
                        } else {
                            for issue in missing_parts.split("\n") {
                                ui.text(issue);
                            }
                        }

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
                            self.active_save.money += pc_builder.offers[pc_builder.offers_index as usize] as i32;
                            self.game_state = GameState::InGame;
                            self.background = Rect::new(0, 0, 1920, 1080, "textures/ingame-background.png");
                            delete_pc_builder_instance = true;
                        }
                        
                        ui.same_line(60.0);

                        //OFFER WINDOW EXIT BUTTON
                        if ui.button(str_to_imstr("Exit\0"), [40.0, 20.0]) {
                            pc_builder.offer_window = false;
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

    pub fn run(&mut self, window: &mut Window, ui: &Ui) {
        self.background.draw();
        
        for rect in &self.rects {
            rect.draw();
        }

        self.main_menu(window, ui);
        self.ingame(ui);
        self.pc_builder(window, ui);
    }
}

fn create_combo(name: &ImStr, mut current_item: i32, items: &[&ImStr], ui: &Ui) -> i32 {
    ui.combo(name, &mut current_item, items, 4);
    current_item
}
