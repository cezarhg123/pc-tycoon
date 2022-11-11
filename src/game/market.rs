use glfw::Window;

use crate::{gfx::{image_rect::ImageRect, color_rect::ColorRect, texture::Texture, vectors::{vec2::vec2, vec3::vec3}, text::Text}, ui::{button::Button, Ui}, WINDOW_WIDTH, WINDOW_HEIGHT, part_loader::{get_case_names, get_case, get_mb_names, get_mb, get_cpu_names, get_cpu, get_cpu_cooler_names, get_cpu_cooler, get_ram_names, get_ram, get_gpu_names, get_gpu, get_storage_names, get_storage, get_fan_names, get_fan, get_psu_names, get_psu}};

use super::{player_inventory::PlayerInventory, save::Save};

#[derive(Debug, Clone)]
pub struct Market<'a> {
    background: ImageRect,
    money: Text<'a>,
    back_button: Button<'a>,
    side_tabs: [Button<'a>; 9],
    contents: Vec<(ColorRect, Vec<Text<'a>>, Button<'a>, &'a str)>,
    current_item: u32
}

impl<'a> Market<'a> {
    pub fn new(ui: &'a Ui) -> Market<'a> {
        let start_pos_y = 96.0;
        let size_y = 109.0;

        Market {
            background: ImageRect::new(Texture::from_path("textures/market-background.png"), 0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            money: ui.text("2", 30.0, vec3(255, 255, 255), Some(vec2(200.0, 10.0))),
            back_button: ui.button("market back button", vec2(1750.0, 0.0), vec2(170.0, 85.0)),
            side_tabs: [
                ui.button("Cases", vec2(0.0, start_pos_y), vec2(280.0, size_y)),
                ui.button("Motherboards", vec2(0.0, start_pos_y + size_y), vec2(280.0, size_y)),
                ui.button("CPUs", vec2(0.0, start_pos_y + (size_y * 2.0)), vec2(280.0, size_y)),
                ui.button("CPU Coolers", vec2(0.0, start_pos_y + (size_y * 3.0)), vec2(280.0, size_y)),
                ui.button("RAMs", vec2(0.0, start_pos_y + (size_y * 4.0)), vec2(280.0, size_y)),
                ui.button("GPUs", vec2(0.0, start_pos_y + (size_y * 5.0)), vec2(280.0, size_y)),
                ui.button("Storages", vec2(0.0, start_pos_y + (size_y * 6.0)), vec2(280.0, size_y)),
                ui.button("Fans", vec2(0.0, start_pos_y + (size_y * 7.0)), vec2(280.0, size_y)),
                ui.button("PSUs", vec2(0.0, start_pos_y + (size_y * 8.0)), vec2(280.0, size_y)),
            ],
            contents: Vec::new(),
            current_item: 0
        }
    }

    pub fn run(&mut self, window: &Window, save: &mut Save, ui: &'a Ui) -> bool {
        self.money = ui.text(format!("Money: {}", save.money).as_str(), 60.0, vec3(0, 0, 0), Some(vec2(460.0, 10.0)));

        for tab in &mut self.side_tabs {
            if tab.clicked(window) {
                self.contents.clear();
                match tab.get_str().to_lowercase().as_str() {
                    "cases" => {
                        self.current_item = 0;
                        
                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for case in get_case_names() {
                            let case = get_case(case);

                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;
                            
                            let name = ui.text(case.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let case_form_factor = ui.text(format!("Case Form Factor - {:#?}", case.case_form_factor).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let mb_form_factor = ui.text(format!("Motherboard Form Factor - {:#?}", case.mb_form_factor).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), case_form_factor.get_top() + case_form_factor.get_height())));
                            let max_fans = ui.text(format!("Max Fans - {}", case.max_fans).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), mb_form_factor.get_top() + mb_form_factor.get_height())));
                            let max_ssd = ui.text(format!("Max SSD - {}", case.max_ssd).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), max_fans.get_top() + max_fans.get_height())));
                            let max_hdd = ui.text(format!("Max HDD - {}", case.max_hdd).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), max_ssd.get_top() + max_ssd.get_height())));
                            let max_cpu_cooler_height = ui.text(format!("Max CPU Cooler Height - {}", case.max_cpu_cooler_height).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), max_hdd.get_top() + max_hdd.get_height())));
                            let max_gpu_length = ui.text(format!("Max GPU Length - {}", case.max_gpu_length).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), max_cpu_cooler_height.get_top() + max_cpu_cooler_height.get_height())));
                            let max_gpu_width = ui.text(format!("Max GPU Width - {}", case.max_gpu_width).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), max_gpu_length.get_top() + max_gpu_length.get_height())));
                            let max_power_supply_length = ui.text(format!("Max Power Supply Length - {}", case.max_power_supply_length).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), max_gpu_width.get_top() + max_gpu_width.get_height())));
                        
                            let mut price = ui.text(case.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
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
                                ],
                                buy,
                                "case"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    "motherboards" => {
                        self.current_item = 0;
                        
                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for mb in get_mb_names() {
                            let mb = get_mb(mb);
                            
                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;

                            let name = ui.text(mb.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let power_usage = ui.text(format!("Power Usage - {}W", mb.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let mb_form_factor = ui.text(format!("Motherboard Form Factor - {:#?}", mb.mb_form_factor).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), power_usage.get_top() + power_usage.get_height())));
                            let socket_type = ui.text(format!("Socket Type - {:#?}", mb.socket_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), mb_form_factor.get_top() + mb_form_factor.get_height())));
                            let ram_type = ui.text(format!("Ram Type - {:#?}", mb.ram_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), socket_type.get_top() + socket_type.get_height())));
                            let ram_slots = ui.text(format!("Ram Slots - {}", mb.ram_slots).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), ram_type.get_top() + ram_type.get_height())));
                            let m2_slots = ui.text(format!("M.2 Slots - {}", mb.m2_slots).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), ram_slots.get_top() + ram_slots.get_height())));
                            let sata_slots = ui.text(format!("Sata Slots - {}", mb.sata_slots).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), m2_slots.get_top() + m2_slots.get_height())));
                            let max_cpu_speed = ui.text(format!("Max CPU Speed - {}", mb.max_cpu_speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), sata_slots.get_top() + sata_slots.get_height())));
                            let max_ram_speed = ui.text(format!("Max RAM Speed - {}", mb.max_ram_speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), max_cpu_speed.get_top() + max_cpu_speed.get_height())));

                            let mut price = ui.text(mb.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
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
                                ],
                                buy,
                                "mb"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    "cpus" => {
                        self.current_item = 0;
                        
                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for cpu in get_cpu_names() {
                            let cpu = get_cpu(cpu);

                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;

                            let name = ui.text(cpu.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let power_usage = ui.text(format!("Power Usage - {}W", cpu.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let socket_type = ui.text(format!("Socket Type - {:#?}", cpu.socket_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), power_usage.get_top() + power_usage.get_height())));
                            let cores = ui.text(format!("Cores - {}", cpu.cores).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), socket_type.get_top() + socket_type.get_height())));
                            let threads = ui.text(format!("Threads - {}", cpu.threads).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), cores.get_top() + cores.get_height())));
                            let speed = ui.text(format!("Speed - {}", cpu.speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), threads.get_top() + threads.get_height())));

                            let mut price = ui.text(cpu.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
                                    name,
                                    price,
                                    power_usage,
                                    socket_type,
                                    cores,
                                    threads,
                                    speed
                                ],
                                buy,
                                "cpu"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    "cpu coolers" => {
                        self.current_item = 0;
                        
                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for cpu_cooler in get_cpu_cooler_names() {
                            let cpu_cooler = get_cpu_cooler(cpu_cooler);
                            
                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;

                            let name = ui.text(cpu_cooler.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let power_usage = ui.text(format!("Power Usage - {}W", cpu_cooler.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let socket_type = ui.text(format!("Socket Type - {:#?}", cpu_cooler.socket_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), power_usage.get_top() + power_usage.get_height())));
                            let height = ui.text(format!("Height - {}", cpu_cooler.height).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), socket_type.get_top() + socket_type.get_height())));
                            let cooling = ui.text(format!("Cooling - {}", cpu_cooler.cooling).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), height.get_top() + height.get_height())));

                            let mut price = ui.text(cpu_cooler.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
                                    name,
                                    price,
                                    power_usage,
                                    socket_type,
                                    height,
                                    cooling
                                ],
                                buy,
                                "cpu cooler"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    "rams" => {
                        self.current_item = 0;

                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for ram in get_ram_names() {
                            let ram = get_ram(ram);

                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;

                            let name = ui.text(ram.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let power_usage = ui.text(format!("Power Usage - {}W", ram.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let ram_type = ui.text(format!("Ram Type - {:#?}", ram.ram_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), power_usage.get_top() + power_usage.get_height())));
                            let size = ui.text(format!("Size - {}", ram.size).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), ram_type.get_top() + ram_type.get_height())));
                            let speed = ui.text(format!("Speed - {}", ram.speed).as_str(),  24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), size.get_top() + size.get_height())));

                            let mut price = ui.text(ram.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
                                    name,
                                    price,
                                    power_usage,
                                    ram_type,
                                    size,
                                    speed
                                ],
                                buy,
                                "ram"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    "gpus" => {
                        self.current_item = 0;

                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for gpu in get_gpu_names() {
                            let gpu = get_gpu(gpu);

                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;

                            let name = ui.text(gpu.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let power_usage = ui.text(format!("Power Usage - {}W", gpu.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let length = ui.text(format!("Length - {}", gpu.length).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), power_usage.get_top() + power_usage.get_height())));
                            let width = ui.text(format!("Width - {}", gpu.width).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), length.get_top() + length.get_height())));
                            let cores = ui.text(format!("Cores - {}", gpu.cores).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), width.get_top() + width.get_height())));
                            let rt_cores = ui.text(format!("Raytracing Cores - {}", gpu.rt_cores).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), cores.get_top() + cores.get_height())));
                            let speed = ui.text(format!("Speed - {}", gpu.speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), rt_cores.get_top() + rt_cores.get_height())));
                            let vram = ui.text(format!("VRAM - {}GB", gpu.vram).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), speed.get_top() + speed.get_height())));

                            let mut price = ui.text(gpu.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
                                    name,
                                    price,
                                    power_usage,
                                    length,
                                    width,
                                    cores,
                                    rt_cores,
                                    speed,
                                    vram
                                ],
                                buy,
                                "gpu"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    "storages" => {
                        self.current_item = 0;

                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for storage in get_storage_names() {
                            let storage = get_storage(storage);

                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;

                            let name = ui.text(storage.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let power_usage = ui.text(format!("Power Usage - {}W", storage.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let storage_type = ui.text(format!("Storage Type - {:#?}", storage.storage_type).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), power_usage.get_top() + power_usage.get_height())));
                            let size = ui.text(format!("Size - {}GB", storage.size).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), storage_type.get_top() + storage_type.get_height())));
                            let speed = ui.text(format!("Speed - {}MB/s", storage.speed).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), size.get_top() + size.get_height())));

                            let mut price = ui.text(storage.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
                                    name,
                                    price,
                                    power_usage,
                                    storage_type,
                                    size,
                                    speed
                                ],
                                buy,
                                "storage"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    "fans" => {
                        self.current_item = 0;

                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for fan in get_fan_names() {
                            let fan = get_fan(fan);
                            
                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;

                            let name = ui.text(fan.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let power_usage = ui.text(format!("Power Usage - {}W", fan.power_usage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let large = if fan.large {
                                ui.text("Size - 140MM", 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), power_usage.get_top() + power_usage.get_height())))
                            } else {
                                ui.text("Size - 120MM", 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), power_usage.get_top() + power_usage.get_height())))
                            };
                            let cooling = ui.text(format!("Cooling - {}", fan.cooling).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), large.get_top() + large.get_height())));

                            let mut price = ui.text(fan.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
                                    name,
                                    price,
                                    power_usage,
                                    large,
                                    cooling
                                ],
                                buy,
                                "fan"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    "psus" => {
                        self.current_item = 0;

                        let width = 380.0;
                        let height = 400.0;
                        let padding = 15.0;
                        let padding_total = 30.0; // ..###.., the dots are the padding total
                        let starting_x = 280.0;
                        let starting_y = 100.0;
                        let mut x_offset = 0.0;
                        let mut y_offset = 0.0;

                        for psu in get_psu_names() {
                            let psu = get_psu(psu);

                            let black_background = ColorRect::new(vec3(0.0, 0.0, 0.0), starting_x + padding + ((padding_total + width) * x_offset), starting_y + ((padding + height) * y_offset), width, height);
                            let middle_x = black_background.get_center().x;

                            let name = ui.text(psu.name.as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_top())));
                            let wattage = ui.text(format!("Wattage - {}", psu.wattage).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), name.get_top() + name.get_height())));
                            let length = ui.text(format!("Length - {}", psu.length).as_str(), 24.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), wattage.get_top() + wattage.get_height())));
                        
                            let mut price = ui.text(psu.price.to_string().as_str(), 60.0, vec3(255, 255, 255), Some(vec2(black_background.get_left(), black_background.get_bottom() - 60.0)));
                            price.set_center(vec2(black_background.get_left() + (black_background.get_width() / 4.0), black_background.get_top() + black_background.get_height() - 30.0));
                            let buy = ui.button("Buy", vec2(middle_x, black_background.get_bottom() - 60.0), vec2(black_background.get_width() / 2.0, 60.0));

                            self.contents.push((
                                black_background,
                                vec![
                                    name,
                                    price,
                                    wattage,
                                    length
                                ],
                                buy,
                                "psu"
                            ));

                            if x_offset == 3.0 {
                                x_offset = 0.0;
                                y_offset += 1.0;
                            } else {
                                x_offset += 1.0;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        for content in &mut self.contents {
            if content.2.clicked(window) {
                match content.3 {
                    "case" => {
                        let case = get_case(&content.1[0].get_str());

                        if save.money >= case.price {
                            save.money -= case.price;
                            save.inventory.cases.push(case.name);
                        }
                    }
                    "mb" => {}
                    "cpu" => {}
                    "cpu cooler" => {}
                    "ram" => {}
                    "gpu" => {}
                    "storage" => {}
                    "fan" => {}
                    "psu" => {}
                    _ => {}
                }
            }
        }

        self.back_button.clicked(window)
    } 

    pub fn draw(&self) {
        self.background.draw();
        for tab in &self.side_tabs {
            tab.draw();
        }

        for item in self.current_item..(self.current_item + 8) {
            let item = self.contents.get(item as usize);

            match item {
                Some(item) => {
                    item.0.draw();
                    for text in &item.1 {
                        text.draw();
                    }
                    item.2.draw();
                }
                None => {}
            }
        }

        self.money.draw();
    }
}
