use cgmath::{
    num_traits::{FromPrimitive, ToPrimitive},
    Vector3,
};
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
    pub fn from_cgmath<T>(v: &Vector3<T>) -> V3F
    where
        T: ToPrimitive,
    {
        V3F {
            x: v.x.to_f32().unwrap(),
            y: v.y.to_f32().unwrap(),
            z: v.z.to_f32().unwrap(),
        }
    }
    pub fn to_cgmath<T>(&self) -> Vector3<T>
    where
        T: FromPrimitive,
    {
        Vector3::<T>::new(
            T::from_f32(self.x).unwrap(),
            T::from_f32(self.y).unwrap(),
            T::from_f32(self.z).unwrap(),
        )
    }
}
