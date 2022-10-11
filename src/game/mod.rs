pub mod pc_components;
pub mod pc;
mod save;
mod pcbuilder;

use imgui_glfw_rs::{glfw::{Window, self}, imgui::{Ui, self, ImStr}};
use crate::{rect::Rect, str_to_imstr, WINDOW_WIDTH, WINDOW_HEIGHT, f64_tuple_to_f32_array, components_list::*};

use self::{pcbuilder::PCBuilder, pc_components::{cpu_cooler, ram::Ram, storage_device::StorageDevice, fan::Fan}, pc::Pc};
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
            //set cursor pos x - 116.5
            //button width - 400

            for _ in 0..2 {
                ui.new_line();
            }

            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            if ui.button(str_to_imstr("Build PC\0"), [400.0, 120.0]) {
                self.game_state = GameState::PcBuilder;
                self.background = Rect::new(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, "textures/pc-builder.png");
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
                    compatability_window: false
                });
            }
            
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            if ui.button(str_to_imstr("Inventory\0"), [400.0, 120.0]) {
                self.game_state = GameState::Inventory;
            }

            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            if ui.button(str_to_imstr("Market\0"), [400.0, 120.0]) {
                self.game_state = GameState::Market;
            }
            
            let crnt_y = ui.get_cursor_pos()[1];
            ui.set_cursor_pos([116.5, crnt_y]);

            if ui.button(str_to_imstr("Contracts\0"), [400.0, 120.0]) {
                self.game_state = GameState::Contract;
            }
        });
    }

    fn pc_builder(&mut self, window: &mut Window, ui: &Ui) {
        if self.game_state != GameState::PcBuilder {
            return;
        }

        match self.pc_builder.as_mut() {
            Some(pc_builder) => {
                ui.window(str_to_imstr("pc builder inventory\0"))
                .title_bar(false)
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .build(|| {
                    let mut cases: Vec<&ImStr> = Vec::new();
                    cases.push(str_to_imstr("none\0"));
                    for case in get_case_list() {
                        cases.push(str_to_imstr(case.name.as_str()));
                    }
                    pc_builder.current_case = create_combo(
                        str_to_imstr("Case\0"),
                        pc_builder.current_case,
                        cases.as_slice(),
                        ui
                    );
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
                    
                    if ui.button(str_to_imstr("Check Compatability\0"), [300.0, 80.0]) {
                        if !pc_builder.compatability_window {
                            pc_builder.compatability_window = true;
                        }
                    }
                });

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
                                    ui.text(good);
                                    pc.calculate_score();
                                    ui.text(pc.computing_score.to_string());
                                    ui.text(pc.graphics_score.to_string());
                                    ui.text(pc.total_score.to_string());
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

                if pc_builder.back_button.contains(f64_tuple_to_f32_array(window.get_cursor_pos())) && window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button1) == glfw::Action::Press{
                    self.game_state = GameState::InGame;
                    self.pc_builder = None;
                    self.background = Rect::new(0, 0, 1920, 1080, "textures/ingame-background.png");
                }
            }
            None => {}
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
