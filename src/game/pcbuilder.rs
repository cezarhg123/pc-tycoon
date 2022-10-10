use imgui_glfw_rs::imgui::ImStr;

use crate::rect::Rect;

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
    pub compatability_window: bool
}
