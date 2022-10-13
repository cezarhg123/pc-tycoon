use imgui_glfw_rs::imgui::{ImStr, ImString};

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
