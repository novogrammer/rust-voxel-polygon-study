use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct V2F {
    x: f32,
    y: f32,
}
#[wasm_bindgen]
impl V2F {
    pub fn new(x: f32, y: f32) -> V2F {
        V2F { x, y }
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

impl V2F {
    pub fn from_glam(v: &glam::Vec2) -> V2F {
        V2F { x: v.x, y: v.y }
    }
    pub fn to_glam(&self) -> glam::Vec2 {
        glam::vec2(self.get_x(), self.get_y())
    }
}
