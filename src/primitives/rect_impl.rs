use super::rect::Rect;

impl Rect {
    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn set_left(&mut self, left: f32) {
        self.left = left;
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn set_top(&mut self, top: f32) {
        self.top = top;
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn right(&self) -> f32 {
        self.left + self.width
    }

    pub fn set_right(&mut self, right: f32) {
        self.left = right - self.width;
    }

    pub fn bottom(&self) -> f32 {
        self.top + self.height
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.top = bottom - self.height;
    }

    pub fn center(&self) -> glm::Vec2 {
        glm::vec2(self.left + (self.width / 2.0), self.top + (self.height / 2.0))
    }

    pub fn set_center(&mut self, center: glm::Vec2) {
        self.left = center.x - (self.width / 2.0);
        self.top = center.y - (self.height / 2.0);
    }

    pub fn color(&self) -> glm::Vec4 {
        self.color
    }

    pub fn set_color(&mut self, color: glm::Vec4) {
        self.color = color;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Dim color by `amount`
    /// 
    /// # Arguments
    /// 
    /// * `amount` - Amount to dim by (0.0 - 1.0)
    pub fn dim(&mut self, amount: f32) {
        self.color = self.color - glm::vec4(amount, amount, amount, 0.0);
    }

    /// Undim color by `amount`
    /// 
    /// # Arguments
    /// 
    /// * `amount` - Amount to undim by (0.0 - 1.0)
    pub fn undim(&mut self, amount: f32) {
        self.color = self.color + glm::vec4(amount, amount, amount, 0.0);
    }
}