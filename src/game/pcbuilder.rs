use imgui_glfw_rs::imgui::{ImString};

use crate::rect::Rect;

use super::pc::Pc;

#[derive(Debug, Clone)]
pub struct PCBuilder {
    pub back_button: Rect,
    pub current_case: i32,
    pub current_motherboard: i32,
    pub current_cpu: i32,
    pub current_cpu_cooler: i32,
    pub current_rams: Vec<i32>,
    pub current_gpu: i32,
    pub current_storages: [i32; 6],
    pub current_fans: Vec<i32>,
    pub current_power_supply: i32,
    pub compatability_window: bool,
    pub pc_works: bool,
    pub sell_window: bool,
    pub comp_change: i32,
    pub pc_to_sell: Option<Pc>,
    pub pc_input_price: ImString,
    pub offer_window: bool,
    pub pc_final_price: i32,
    pub offers: Vec<i32>,
    pub offers_index: i32
}

impl PCBuilder {
    pub fn new() -> Self {
        PCBuilder {
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
        }
    }
}
