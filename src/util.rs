pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Bounds {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Bounds { x, y, w, h }
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x > self.x && x < self.x + (self.w as i32) && y > self.y && y < self.y + (self.h as i32)
    }
}
