use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct V3F {
    x: f32,
    y: f32,
    z: f32,
}
#[wasm_bindgen]
impl V3F {
    pub fn new(x: f32, y: f32, z: f32) -> V3F {
        V3F { x, y, z }
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_z(&self) -> f32 {
        self.z
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }
}

impl V3F {
    pub fn from_glam(v: &glam::Vec3) -> V3F {
        V3F {
            x: v.x(),
            y: v.y(),
            z: v.z(),
        }
    }
    pub fn to_glam(&self) -> glam::Vec3 {
        glam::vec3(self.get_x(), self.get_y(), self.get_z())
    }
}
