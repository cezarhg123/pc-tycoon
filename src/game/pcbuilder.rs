use std::io::Cursor;
use glfw::Window;
use rand::Rng;
use crate::{gfx::{image_rect::ImageRect, texture::Texture, vectors::{vec2::{vec2, Vec2}, vec3::vec3}, color_rect::ColorRect, text::Text}, ui::{button::Button, Ui}, WINDOW_WIDTH, WINDOW_HEIGHT, part_loader::{get_case, get_mb, get_cpu, get_cpu_cooler, get_ram, get_gpu, get_storage, get_fan, get_psu}};
use super::{save::Save, pc::PC};

#[derive(Debug, Clone)]
pub struct PCBuilder<'a> {
    background: ImageRect,
    back_button: Button<'a>,
    open_buttons: [Button<'a>; 9],
    buttons_window: Option<(ColorRect, Vec<Button<'a>>, String)>,
    select_window: Option<(ColorRect, Vec<Text<'a>>, Button<'a>, String)>,
    pc: PC,
    total_price: Text<'a>,
    sell_button: Option<Button<'a>>,
    sell_window: Option<(ColorRect, [Text<'a>; 5]/*value, compute score, graphic score, total score, offer*/, [Button<'a>; 3] /*left, right, sell*/)>,
    offers: (usize, Vec<u32>),
    parts_text: Vec<Text<'a>>
}

const BUTTON_POPUP_POS: Vec2<f32> = vec2((WINDOW_WIDTH as f32 / 2.0) - 200.0, (WINDOW_HEIGHT as f32 / 2.0) - 300.0);
const BUTTON_POPUP_SIZE: Vec2<f32> = vec2(400.0, 600.0);
const SELECT_WINDOW_POS: Vec2<f32> = vec2(BUTTON_POPUP_POS.x + BUTTON_POPUP_SIZE.x, BUTTON_POPUP_POS.y);
const SELECT_WINDOW_SIZE: Vec2<f32> = BUTTON_POPUP_SIZE;

impl<'a> PCBuilder<'a> {
    pub fn new(ui: &'a Ui) -> PCBuilder<'a> {
        PCBuilder {
            background: ImageRect::new(Texture::from_path("textures/pc-builder.png"), 0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            back_button: ui.button("pc builder back button", vec2(1750.0, 0.0), vec2(170.0, 85.0)),
            open_buttons: [
                ui.button("Cases", vec2(10.0, 105.0), vec2(380.0, 95.0)),
                ui.button("Motherboards", vec2(10.0, 215.0), vec2(380.0, 95.0)),
                ui.button("CPUs", vec2(10.0, 325.0), vec2(380.0, 95.0)),
                ui.button("CPU Coolers", vec2(10.0, 435.0), vec2(380.0, 95.0)),
                ui.button("RAMs", vec2(10.0, 545.0), vec2(380.0, 95.0)),
                ui.button("GPUs", vec2(10.0, 655.0), vec2(380.0, 95.0)),
                ui.button("Storages", vec2(10.0, 765.0), vec2(380.0, 95.0)),
                ui.button("Fans", vec2(10.0, 875.0), vec2(380.0, 95.0)),
                ui.button("PSUs", vec2(10.0, 985.0), vec2(380.0, 95.0))
            ],
            buttons_window: None,
            select_window: None,
            pc: PC::new(),
            total_price: ui.text("0", 40.0, vec3(255, 255, 255), Some(vec2(0.0, 0.0))),
            sell_button: None,
            sell_window: None,
            offers: (0, Vec::new()),
            parts_text: Vec::new()
        }
    }

    pub fn run(&mut self, window: &Window, save: &mut Save, ui: &'a Ui) -> bool {
        for button in &mut self.open_buttons {
            if button.clicked(window) {
                match button.get_str().as_str() {
                    "Cases" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for case in &save.inventory.cases {
                            buttons.push(ui.button(case.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "case".to_string()));
                    }
                    "Motherboards" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for mb in &save.inventory.mbs {
                            buttons.push(ui.button(mb.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "mb".to_string()));
                    }
                    "CPUs" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for cpu in &save.inventory.cpus {
                            buttons.push(ui.button(cpu.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "cpu".to_string()));
                    }
                    "CPU Coolers" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for cpu_cooler in &save.inventory.cpu_coolers {
                            buttons.push(ui.button(cpu_cooler.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "cpu cooler".to_string()));
                    }
                    "RAMs" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for ram in &save.inventory.rams {
                            buttons.push(ui.button(ram.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "ram".to_string()));
                    }
                    "GPUs" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for gpu in &save.inventory.gpus {
                            buttons.push(ui.button(gpu.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "gpu".to_string()));
                    }
                    "Storages" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for storage in &save.inventory.storages {
                            buttons.push(ui.button(storage.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "storage".to_string()));
                    }
                    "Fans" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for fan in &save.inventory.fans {
                            buttons.push(ui.button(fan.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "fan".to_string()));
                    }
                    "PSUs" => {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), BUTTON_POPUP_POS.x, BUTTON_POPUP_POS.y, BUTTON_POPUP_SIZE.x, BUTTON_POPUP_SIZE.y);
                        let mut buttons: Vec<Button> = Vec::new();

                        let mut i = 0;
                        for psu in &save.inventory.psus {
                            buttons.push(ui.button(psu.as_str(), vec2(BUTTON_POPUP_POS.x + 10.0, BUTTON_POPUP_POS.y + 10.0 + (60.0 * i as f32)), vec2(BUTTON_POPUP_SIZE.x - 20.0, 60.0)));
                            i += 1;
                        }

                        self.buttons_window = Some((background, buttons, "psu".to_string()));
                    }
                    _ => {}
                }
            }
        }

        match &mut self.buttons_window {
            Some(buttons_window) => {
                for button in &mut buttons_window.1 {
                    if button.clicked(window) {
                        let background = ColorRect::new(vec3(0.0, 0.0, 0.0), SELECT_WINDOW_POS.x, SELECT_WINDOW_POS.y, SELECT_WINDOW_SIZE.x, SELECT_WINDOW_SIZE.y);
                        let mut texts = Vec::new();
                        let select_button = ui.button("Select", vec2(background.get_left() + 10.0, background.get_bottom() - 50.0), vec2(100.0, 40.0));
                        let mut item_type = String::new();

                        match buttons_window.2.as_str() {
                            "case" => {
                                let case = get_case(&button.get_str());

                                let name = ui.text(case.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(case.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let case_form_factor = ui.text(format!("Case Form Factor - {:#?}", case.case_form_factor).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let mb_form_factor = ui.text(format!("Motherboard Form Factor - {:#?}", case.mb_form_factor).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), case_form_factor.get_top() + case_form_factor.get_height())));
                                let max_fans = ui.text(format!("Max Fans - {}", case.max_fans).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), mb_form_factor.get_top() + mb_form_factor.get_height())));
                                let max_ssd = ui.text(format!("Max SSD - {}", case.max_ssd).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), max_fans.get_top() + max_fans.get_height())));
                                let max_hdd = ui.text(format!("Max HDD - {}", case.max_hdd).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), max_ssd.get_top() + max_ssd.get_height())));
                                let max_cpu_cooler_height = ui.text(format!("Max CPU Cooler Height - {}", case.max_cpu_cooler_height).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), max_hdd.get_top() + max_hdd.get_height())));
                                let max_gpu_length = ui.text(format!("Max GPU Length - {}", case.max_gpu_length).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), max_cpu_cooler_height.get_top() + max_cpu_cooler_height.get_height())));
                                let max_gpu_width = ui.text(format!("Max GPU Width - {}", case.max_gpu_width).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), max_gpu_length.get_top() + max_gpu_length.get_height())));
                                let max_power_supply_length = ui.text(format!("Max Power Supply Length - {}", case.max_power_supply_length).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), max_gpu_width.get_top() + max_gpu_width.get_height())));
                            
                                texts.append(&mut vec![
                                    name,
                                    price,
                                    case_form_factor,
                                    mb_form_factor,
                                    max_fans,
                                    max_ssd,
                                    max_hdd,
                                    max_cpu_cooler_height,
                                    max_gpu_length,
                                    max_gpu_width,
                                    max_power_supply_length
                                ]);
                                item_type = "case".to_string();
                            }
                            "mb" => {
                                let mb = get_mb(&button.get_str());
                                
                                let name = ui.text(mb.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(mb.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let power_usage = ui.text(format!("Power Usage - {}W", mb.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let mb_form_factor = ui.text(format!("Motherboard Form Factor - {:#?}", mb.mb_form_factor).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), power_usage.get_top() + power_usage.get_height())));
                                let socket_type = ui.text(format!("Socket Type - {:#?}", mb.socket_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), mb_form_factor.get_top() + mb_form_factor.get_height())));
                                let ram_type = ui.text(format!("Ram Type - {:#?}", mb.ram_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), socket_type.get_top() + socket_type.get_height())));
                                let ram_slots = ui.text(format!("Ram Slots - {}", mb.ram_slots).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), ram_type.get_top() + ram_type.get_height())));
                                let m2_slots = ui.text(format!("M.2 Slots - {}", mb.m2_slots).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), ram_slots.get_top() + ram_slots.get_height())));
                                let sata_slots = ui.text(format!("Sata Slots - {}", mb.sata_slots).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), m2_slots.get_top() + m2_slots.get_height())));
                                let max_cpu_speed = ui.text(format!("Max CPU Speed - {}", mb.max_cpu_speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), sata_slots.get_top() + sata_slots.get_height())));
                                let max_ram_speed = ui.text(format!("Max RAM Speed - {}", mb.max_ram_speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), max_cpu_speed.get_top() + max_cpu_speed.get_height())));
                                
                                texts.append(&mut vec![
                                    name,
                                    price,
                                    power_usage,
                                    mb_form_factor,
                                    socket_type,
                                    ram_type,
                                    ram_slots,
                                    m2_slots,
                                    sata_slots,
                                    max_cpu_speed,
                                    max_ram_speed
                                ]);
                                item_type = "mb".to_string();
                            }
                            "cpu" => {
                                let cpu = get_cpu(&button.get_str());

                                let name = ui.text(cpu.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(cpu.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let power_usage = ui.text(format!("Power Usage - {}W", cpu.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let socket_type = ui.text(format!("Socket Type - {:#?}", cpu.socket_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), power_usage.get_top() + power_usage.get_height())));
                                let cores = ui.text(format!("Cores - {}", cpu.cores).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), socket_type.get_top() + socket_type.get_height())));
                                let threads = ui.text(format!("Threads - {}", cpu.threads).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), cores.get_top() + cores.get_height())));
                                let speed = ui.text(format!("Speed - {}", cpu.speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), threads.get_top() + threads.get_height())));
                            
                                texts.append(&mut vec![
                                    name,
                                    price,
                                    power_usage,
                                    socket_type,
                                    cores,
                                    threads,
                                    speed
                                ]);
                                item_type = "cpu".to_string();
                            }
                            "cpu cooler" => {
                                let cpu_cooler = get_cpu_cooler(&button.get_str());

                                let name = ui.text(cpu_cooler.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(cpu_cooler.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let power_usage = ui.text(format!("Power Usage - {}W", cpu_cooler.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let socket_type = ui.text(format!("Socket Type - {:#?}", cpu_cooler.socket_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), power_usage.get_top() + power_usage.get_height())));
                                let height = ui.text(format!("Height - {}", cpu_cooler.height).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), socket_type.get_top() + socket_type.get_height())));
                                let cooling = ui.text(format!("Cooling - {}", cpu_cooler.cooling).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), height.get_top() + height.get_height())));

                                texts.append(&mut vec![
                                    name,
                                    price,
                                    power_usage,
                                    socket_type,
                                    height,
                                    cooling
                                ]);
                                item_type = "cpu cooler".to_string();
                            }
                            "ram" => {
                                let ram = get_ram(&button.get_str());

                                let name = ui.text(ram.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(ram.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let power_usage = ui.text(format!("Power Usage - {}W", ram.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let ram_type = ui.text(format!("Ram Type - {:#?}", ram.ram_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), power_usage.get_top() + power_usage.get_height())));
                                let size = ui.text(format!("Size - {}", ram.size).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), ram_type.get_top() + ram_type.get_height())));
                                let speed = ui.text(format!("Speed - {}", ram.speed).as_str(),  24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), size.get_top() + size.get_height())));

                                texts.append(&mut vec![
                                    name,
                                    price,
                                    power_usage,
                                    ram_type,
                                    size,
                                    speed
                                ]);
                                item_type = "ram".to_string();
                            }
                            "gpu" => {
                                let gpu = get_gpu(&button.get_str());

                                let name = ui.text(gpu.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(gpu.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let power_usage = ui.text(format!("Power Usage - {}W", gpu.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let length = ui.text(format!("Length - {}", gpu.length).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), power_usage.get_top() + power_usage.get_height())));
                                let width = ui.text(format!("Width - {}", gpu.width).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), length.get_top() + length.get_height())));
                                let cores = ui.text(format!("Cores - {}", gpu.cores).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), width.get_top() + width.get_height())));
                                let rt_cores = ui.text(format!("Raytracing Cores - {}", gpu.rt_cores).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), cores.get_top() + cores.get_height())));
                                let speed = ui.text(format!("Speed - {}", gpu.speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), rt_cores.get_top() + rt_cores.get_height())));
                                let vram = ui.text(format!("VRAM - {}GB", gpu.vram).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), speed.get_top() + speed.get_height())));

                                texts.append(&mut vec![
                                    name,
                                    price,
                                    power_usage,
                                    length,
                                    width,
                                    cores,
                                    rt_cores,
                                    speed,
                                    vram
                                ]);
                                item_type = "gpu".to_string();
                            }
                            "storage" => {
                                let storage = get_storage(&button.get_str());

                                let name = ui.text(storage.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(storage.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let power_usage = ui.text(format!("Power Usage - {}W", storage.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let storage_type = ui.text(format!("Storage Type - {:#?}", storage.storage_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), power_usage.get_top() + power_usage.get_height())));
                                let size = ui.text(format!("Size - {}GB", storage.size).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), storage_type.get_top() + storage_type.get_height())));
                                let speed = ui.text(format!("Speed - {}MB/s", storage.speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), size.get_top() + size.get_height())));

                                texts.append(&mut vec![
                                    name,
                                    price,
                                    power_usage,
                                    storage_type,
                                    size,
                                    speed
                                ]);
                                item_type = "storage".to_string();
                            }
                            "fan" => {
                                let fan = get_fan(&button.get_str());

                                let name = ui.text(fan.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(fan.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let power_usage = ui.text(format!("Power Usage - {}W", fan.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let large = if fan.large {
                                    ui.text("Size - 140MM", 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), power_usage.get_top() + power_usage.get_height())))
                                } else {
                                    ui.text("Size - 120MM", 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), power_usage.get_top() + power_usage.get_height())))
                                };
                                let cooling = ui.text(format!("Cooling - {}", fan.cooling).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), large.get_top() + large.get_height())));

                                texts.append(&mut vec![
                                    name,
                                    price,
                                    power_usage,
                                    large,
                                    cooling
                                ]);
                                item_type = "fan".to_string();
                            }
                            "psu" => {
                                let psu = get_psu(&button.get_str());

                                let name = ui.text(psu.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), background.get_top())));
                                let price = ui.text(psu.price.to_string().as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), name.get_top() + name.get_height())));
                                let wattage = ui.text(format!("Wattage - {}", psu.wattage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), price.get_top() + price.get_height())));
                                let length = ui.text(format!("Length - {}", psu.length).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(background.get_left(), wattage.get_top() + wattage.get_height())));
                                
                                texts.append(&mut vec![
                                    name,
                                    price,
                                    wattage,
                                    length
                                ]);
                                item_type = "psu".to_string();
                            }
                            _ => {}
                        }

                        self.select_window = Some((background, texts, select_button, item_type));
                    }
                }
            }
            None => {}
        }

        match &mut self.select_window {
            Some(select_window) => {
                if select_window.2.clicked(window) {
                    match select_window.3.as_str() {
                        "case" => {
                            let case = get_case(&select_window.1[0].get_str());
                            //1510 110
                            self.parts_text.push(ui.text(format!("{} - ${}", &case.name, case.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 110.0))));

                            self.pc.set_case(case);
                            self.buttons_window = None;
                            self.select_window = None;

                        }
                        "mb" => {
                            let mb = get_mb(&select_window.1[0].get_str());
                            self.parts_text.push(ui.text(format!("{} - ${}", &mb.name, mb.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 160.0))));

                            if self.pc.can_set_mb(&mb) {
                                self.pc.set_mb(mb);
                                self.buttons_window = None;
                                self.select_window = None;
                            }
                        }
                        "cpu" => {
                            let cpu = get_cpu(&select_window.1[0].get_str());
                            self.parts_text.push(ui.text(format!("{} - ${}", &cpu.name, cpu.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 210.0))));

                            if self.pc.can_set_cpu(&cpu) {
                                self.pc.set_cpu(cpu);
                                self.buttons_window = None;
                                self.select_window = None;
                            }
                        }
                        "cpu cooler" => {
                            let cpu_cooler = get_cpu_cooler(&select_window.1[0].get_str());
                            self.parts_text.push(ui.text(format!("{} - ${}", &cpu_cooler.name, cpu_cooler.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 260.0))));

                            if self.pc.can_set_cpu_cooler(&cpu_cooler) {
                                self.pc.set_cpu_cooler(cpu_cooler);
                                self.buttons_window = None;
                                self.select_window = None;
                            }
                        }
                        "ram" => {
                            let ram = get_ram(&select_window.1[0].get_str());
                            self.parts_text.push(ui.text(format!("{} - ${}", &ram.name, ram.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 310.0))));

                            if self.pc.can_set_ram(&ram) {
                                self.pc.set_ram(ram);
                                self.buttons_window = None;
                                self.select_window = None;
                            }
                        }
                        "gpu" => {
                            let gpu = get_gpu(&select_window.1[0].get_str());
                            self.parts_text.push(ui.text(format!("{} - ${}", &gpu.name, gpu.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 360.0))));

                            if self.pc.can_set_gpu(&gpu) {
                                self.pc.set_gpu(gpu);
                                self.buttons_window = None;
                                self.select_window = None;
                            }
                        }
                        "storage" => {
                            let storage = get_storage(&select_window.1[0].get_str());
                            self.parts_text.push(ui.text(format!("{} - ${}", &storage.name, storage.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 410.0))));

                            if self.pc.can_set_storage(&storage) {
                                self.pc.set_storage(storage);
                                self.buttons_window = None;
                                self.select_window = None;
                            }
                        }
                        "fan" => {
                            let fan = get_fan(&select_window.1[0].get_str());
                            self.parts_text.push(ui.text(format!("{} - ${}", &fan.name, fan.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 460.0))));

                            if self.pc.can_set_fan(&fan) {
                                self.pc.set_fan(fan);
                                self.buttons_window = None;
                                self.select_window = None;
                            }
                        }
                        "psu" => {
                            let psu = get_psu(&select_window.1[0].get_str());
                            self.parts_text.push(ui.text(format!("{} - ${}", &psu.name, psu.price).as_str(), 40.0, vec3(0, 0, 0), Some(vec2(1510.0, 510.0))));

                            if self.pc.can_set_psu(&psu) {
                                self.pc.set_psu(psu);
                                self.buttons_window = None;
                                self.select_window = None;
                            }
                        }
                        _ => {}
                    }
                }
            }
            None => {}
        }

        //set total price of PC
        let price = self.pc.get_price();
        self.total_price = ui.text(format!("PC Value: ${}", price).as_str(), 30.0, vec3(0, 0, 0), None);
        self.total_price.set_center(vec2(1620.0, 42.0));
        //1510 920

        if self.pc.works() && self.sell_button.is_none() {
            self.sell_button = Some(ui.button("Sell", vec2(1510.0, 920.0), vec2(400.0, 120.0)));
        } else if !self.pc.works() && self.sell_button.is_some() {
            self.sell_button = None;
        }

        match &mut self.sell_button {
            Some(button) => {
                if button.clicked(window) {
                    let mut rect = ColorRect::new(vec3(0.0, 0.0, 0.0), 0.0, 0.0, 400.0, 400.0);
                    rect.set_center(vec2(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0));

                    let mut value = ui.text(format!("PC Value: ${}", self.pc.get_price()).as_str(), 40.0, vec3(255, 255, 255), None);
                    value.set_center(vec2(rect.get_center().x, rect.get_top() + 30.0));

                    let mut compute_score = ui.text(format!("Compute Score: {}", self.pc.get_compute_score()).as_str(), 40.0, vec3(255, 255, 255), None);
                    compute_score.set_center(vec2(rect.get_center().x, 0.0));
                    compute_score.set_top(value.get_top() + value.get_height() + 10.0);

                    let mut graphic_score = ui.text(format!("Graphic Score: {}", self.pc.get_graphic_score()).as_str(), 40.0, vec3(255, 255, 255), None);
                    graphic_score.set_center(vec2(rect.get_center().x, 0.0));
                    graphic_score.set_top(compute_score.get_top() + compute_score.get_height() + 10.0);
                
                    let mut total_score = ui.text(format!("Total Score: {}", self.pc.get_total_score()).as_str(), 40.0, vec3(255, 255, 255), None);
                    total_score.set_center(vec2(rect.get_center().x, 0.0));
                    total_score.set_top(graphic_score.get_top() + graphic_score.get_height() + 10.0);
                
                    for _ in 0..5 {
                        self.offers.1.push((self.pc.get_price() as i32 + 100 + rand::thread_rng().gen_range(-50..50)) as u32);
                    }

                    let mut offer_score = ui.text(format!("${}", self.offers.1[self.offers.0]).as_str(), 50.0, vec3(255, 255, 255), None);
                    offer_score.set_center(vec2(rect.get_center().x, 0.0));
                    offer_score.set_top(total_score.get_top() + total_score.get_height() + 20.0);

                    let left = ui.button("<", vec2(offer_score.get_left() - 55.0, offer_score.get_top()), vec2(50.0, 50.0));
                    let right = ui.button(">", vec2(offer_score.get_left() + offer_score.get_width() + 5.0, offer_score.get_top()), vec2(50.0, 50.0));
                    let sell = ui.button("Sell", vec2(rect.get_center().x - 60.0, rect.get_bottom() - 45.0), vec2(120.0, 40.0));
                
                    self.sell_window = Some((
                        rect,
                        [value, compute_score, graphic_score, total_score, offer_score],
                        [left, right, sell]
                    ));
                }
            }
            None => {}
        }

        match &mut self.sell_window {
            Some(sell_window) => {
                if sell_window.2[0].clicked(window) {
                    if self.offers.0 != 0 {
                        self.offers.0 -= 1;

                        let mut offer_score = ui.text(format!("${}", self.offers.1[self.offers.0]).as_str(), 50.0, vec3(255, 255, 255), None);
                        offer_score.set_center(vec2(sell_window.0.get_center().x, 0.0));
                        offer_score.set_top(sell_window.1[3].get_top() + sell_window.1[3].get_height() + 20.0);

                        let sell = ui.button("Sell", vec2(sell_window.0.get_center().x - 60.0, sell_window.0.get_bottom() - 45.0), vec2(120.0, 40.0));
                    
                        sell_window.1[4] = offer_score;
                        sell_window.2[2] = sell;
                    }
                }

                if sell_window.2[1].clicked(window) {
                    if self.offers.0 != 4 {
                        self.offers.0 += 1;

                        let mut offer_score = ui.text(format!("${}", self.offers.1[self.offers.0]).as_str(), 50.0, vec3(255, 255, 255), None);
                        offer_score.set_center(vec2(sell_window.0.get_center().x, 0.0));
                        offer_score.set_top(sell_window.1[3].get_top() + sell_window.1[3].get_height() + 20.0);

                        let sell = ui.button("Sell", vec2(sell_window.0.get_center().x - 60.0, sell_window.0.get_bottom() - 45.0), vec2(120.0, 40.0));
                    
                        sell_window.1[4] = offer_score;
                        sell_window.2[2] = sell;
                    }
                }

                if sell_window.2[2].clicked(window) {
                    fn remove(vec: &mut Vec<String>, name: &String) {
                        vec.remove(vec.iter().position(|s| s == name).unwrap());
                    }

                    remove(&mut save.inventory.cases, &self.pc.case().unwrap().name);
                    remove(&mut save.inventory.mbs, &self.pc.mb().unwrap().name);
                    remove(&mut save.inventory.cpus, &self.pc.cpu().unwrap().name);
                    remove(&mut save.inventory.cpu_coolers, &self.pc.cpu_cooler().unwrap().name);
                    for ram in self.pc.ram() {
                        remove(&mut save.inventory.rams, &ram.name);
                    }
                    remove(&mut save.inventory.gpus, &self.pc.gpu().unwrap().name);
                    for storage in self.pc.storage() {
                        remove(&mut save.inventory.storages, &storage.name);
                    }
                    for fan in self.pc.fans() {
                        remove(&mut save.inventory.fans, &fan.name);
                    }
                    remove(&mut save.inventory.psus, &self.pc.psu().unwrap().name);
                    
                    save.money += self.offers.1[self.offers.0];
                    return true;
                }
            }
            None => {}
        }

        self.back_button.clicked(window)
    }

    pub fn draw(&self) {
        self.background.draw();
        self.total_price.draw();

        for text in &self.parts_text {
            text.draw();
        }

        match &self.sell_button {
            Some(button) => {
                button.draw();
            }
            None => {}
        }

        for button in &self.open_buttons {
            button.draw();
        }

        match &self.buttons_window {
            Some(buttons_window) => {
                buttons_window.0.draw();
                for button in &buttons_window.1 {
                    button.draw();
                }
            }
            None => {}
        }

        match &self.select_window {
            Some(select_window) => {
                select_window.0.draw();
                for text in &select_window.1 {
                    text.draw();
                }
                select_window.2.draw();
            }
            None => {}
        }

        match &self.sell_window {
            Some(sell_window) => {
                sell_window.0.draw();
                for text in &sell_window.1 {
                    text.draw();
                }
                for button in &sell_window.2 {
                    button.draw();
                }
            }
            None => {}
        }
    }
}
