use glfw::Window;

use crate::{gfx::{image_rect::ImageRect, color_rect::ColorRect, texture::Texture, vectors::{vec2::vec2, vec3::vec3}, text::Text}, ui::{Ui, button::Button, listbox::ListBox, info_popup::InfoPopup}, WINDOW_WIDTH, WINDOW_HEIGHT};

use super::{save::Save, Scroll};

#[derive(Debug, Clone)]
pub struct Inventory<'a> {
    background: ImageRect,
    back_button: Button<'a>,
    popups: Vec<InfoPopup<'a>>,
    cases_list: ListBox<'a>,
    label_texts: Vec<Text<'a>>
}

impl<'a> Inventory<'a> {
    pub fn new(save: &Save, ui: &'a Ui) -> Inventory<'a> {
        let cases_list = if save.inventory.cases.as_slice().len() > 0 {
            ListBox::new(vec2(20.0, 140.0), vec2(300.0, 100.0), save.inventory.cases.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(20.0, 140.0), vec2(300.0, 100.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut cases_label = ui.text("Cases", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        cases_label.set_center(cases_list.get_center());
        cases_label.set_top(cases_list.get_top() - 30.0);

        Inventory {
            background: ImageRect::new(Texture::from_path("textures/inventory-background.png"), 0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            back_button: ui.button("a", vec2(1750.0, 0.0), vec2(170.0, 85.0)),
            popups: Vec::new(),
            cases_list,
            label_texts: vec![
                cases_label
            ]
        }
    }

    pub fn run(&mut self, window: &Window, scrolls: &mut Vec<Scroll>, ui: &'a Ui) -> bool {
        self.cases_list.run(window, scrolls);
        for text in &self.cases_list.texts {
            let cursor_pos = window.get_cursor_pos().try_into().unwrap();
            // CASE INFO POPUP
            if text.contains(cursor_pos) {
                let mut info = Vec::new();
                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    
                }

                let popup = ui.info_popup("case info", info.as_slice(), cursor_pos, 24.0);

                if self.popups.len() > 0 {
                    for i in 0..self.popups.len() {
                        if self.popups[i].id == "case info" {
                            self.popups[i] = popup;
                        break;
                        } else {
                            self.popups.push(popup);
                            break;
                        }
                    }
                } else {
                    self.popups.push(popup);
                }
            } else {
                for i in 0..self.popups.len() {
                    if self.popups[i].id == "case info" {
                        self.popups.remove(i);
                    }
                }
            }
        }

        self.back_button.clicked(window)
    }

    pub fn draw(&self) {
        self.background.draw();

        for text in &self.label_texts {
            text.draw();
        }

        self.cases_list.draw();

        for popup in &self.popups {
            popup.draw();
        }
    }
}
