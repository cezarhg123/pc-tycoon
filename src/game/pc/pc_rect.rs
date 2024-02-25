/// Describes a rect to be used in the procedural generation of a pc part image
#[derive(Debug, Clone)]
pub struct PCRect {
    left: u32,
    top: u32,
    width: u32,
    height: u32
}

impl Default for PCRect {
    fn default() -> Self {
        PCRect {
            left: 0,
            top: 0,
            width: 0,
            height: 0
        }
    }
}

impl PCRect {
    pub fn new(left: u32, top: u32, width: u32, height: u32) -> PCRect {
        PCRect {
            left,
            top,
            width,
            height
        }
    }

    pub fn left(&self) -> u32 {
        self.left
    }

    pub fn set_left(&mut self, left: u32) {
        self.left = left;
    }

    pub fn top(&self) -> u32 {
        self.top
    }

    pub fn set_top(&mut self, top: u32) {
        self.top = top;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn right(&self) -> u32 {
        self.left + self.width
    }

    pub fn set_right(&mut self, right: u32) {
        self.left = right - self.width;
    }

    pub fn bottom(&self) -> u32 {
        self.top + self.height
    }

    pub fn set_bottom(&mut self, bottom: u32) {
        self.top = bottom - self.height;
    }

    pub fn contains(&self, pos: glm::UVec2) -> bool {
        pos.x >= self.left && pos.x < self.right() && pos.y >= self.top && pos.y < self.bottom()
    }

    pub fn center(&self) -> glm::UVec2 {
        glm::vec2(self.left + (self.width / 2), self.top + (self.height / 2))
    }
}
