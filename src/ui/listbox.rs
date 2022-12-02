use glfw::Window;
use crate::{gfx::{vectors::{vec2::{Vec2, vec2}, vec3::vec3}, color_rect::ColorRect, text::Text}, game::Scroll};
use super::Ui;

#[derive(Debug, Clone)]
pub struct ListBox<'a> {
    pos: Vec2<f32>,
    size: Vec2<f32>,
    text_size: f32,
    scroll_bar: ColorRect,
    scroll_cursor: ColorRect,
    box_rect: ColorRect,
    pub texts: Vec<Text<'a>>,
    current_item: i32,
    max_items: i32
}

impl<'a> ListBox<'a> {
    pub fn new(pos: Vec2<f32>, size: Vec2<f32>, texts: &[String], text_size: f32, ui: &'a Ui) -> ListBox<'a> {
        let mut max_items = 0;
        let mut size_y = size.y;
        'math: loop {
            size_y -= text_size;

            if size_y < 0.0 {
                break 'math;
            } else {
                max_items += 1;
            }
        }
        
        let cursor_pos_y = pos.y;
        let cursor_size_y = if max_items <= texts.len() as i32 {
            size.y * (max_items as f32 / texts.len() as f32)
        } else {
            size.y
        };
        let mut texts2 = Vec::new();
        let mut offset = 0;
        for text in texts {
            texts2.push(ui.text(text.as_str(), text_size, vec3(255, 255, 255), Some(vec2(pos.x + (text_size / 2.0), pos.y + (offset as f32 * text_size)))));
            
            if offset == max_items - 1 {
                offset = 0;
            } else {
                offset += 1;
            }
        }

        ListBox {
            pos,
            size,
            text_size,
            scroll_bar: ColorRect::new(vec3(0.15, 0.15, 0.15), pos.x + size.x - 10.0, pos.y, 10.0, size.y),
            scroll_cursor: ColorRect::new(vec3(0.7, 0.7, 0.7), pos.x + size.x - 10.0, cursor_pos_y, 10.0, cursor_size_y),
            box_rect: ColorRect::new(vec3(0.0, 0.0, 0.0), pos.x, pos.y, size.x, size.y),
            texts: texts2,
            current_item: 0,
            max_items: max_items
        }
    }

    pub fn run(&mut self, window: &Window, scrolls: &mut Vec<Scroll>) {
        if !self.box_rect.contains(window.get_cursor_pos().try_into().unwrap()) {
            return;
        }

        let mut pop_scroll = false;
        match scrolls.get(0) {
            Some(scroll) => {
                pop_scroll = true;

                match scroll {
                    Scroll::Up => {
                        if self.current_item - self.max_items >= 0 {
                            self.current_item -= self.max_items;
                            self.scroll_cursor.set_top(self.scroll_cursor.get_top() - self.scroll_cursor.get_height());
                        }
                    }
                    Scroll::Down => {
                        if self.current_item + self.max_items < self.texts.len() as i32 {
                            self.current_item += self.max_items;
                            self.scroll_cursor.set_top(self.scroll_cursor.get_top() + self.scroll_cursor.get_height());
                        }
                    }
                }
            }
            None => {}
        }
        
        if pop_scroll {
            scrolls.pop();
        }
    }

    pub fn draw(&self) {
        self.box_rect.draw();
        self.scroll_bar.draw();
        self.scroll_cursor.draw();
        for i in self.current_item..(self.current_item + self.max_items) {
            match self.texts.get(i as usize) {
                Some(text) => {
                    text.draw();
                }
                None => {}
            }
        }
    }

    pub fn get_left(&self) -> f32 {
        self.box_rect.get_left()
    }

    pub fn get_top(&self) -> f32 {
        self.box_rect.get_top()
    }

    pub fn get_center(&self) -> Vec2<f32> {
        self.box_rect.get_center()
    }
}
