use cgmath::{
    num_traits::{FromPrimitive, ToPrimitive},
    Vector2,
};
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
    pub fn from_cgmath<T>(v: &Vector2<T>) -> V2F
    where
        T: ToPrimitive,
    {
        V2F {
            x: v.x.to_f32().unwrap(),
            y: v.y.to_f32().unwrap(),
        }
    }
    pub fn to_cgmath<T>(&self) -> Vector2<T>
    where
        T: FromPrimitive,
    {
        Vector2::<T>::new(T::from_f32(self.x).unwrap(), T::from_f32(self.y).unwrap())
    }
}
