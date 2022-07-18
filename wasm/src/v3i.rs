use cgmath::{
    num_traits::{FromPrimitive, ToPrimitive},
    Vector3,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct V3I {
    x: i32,
    y: i32,
    z: i32,
}
#[wasm_bindgen]
impl V3I {
    pub fn new(x: i32, y: i32, z: i32) -> V3I {
        V3I { x, y, z }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn get_z(&self) -> i32 {
        self.z
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
}

impl V3I {
    pub fn from_cgmath<T>(v: &Vector3<T>) -> V3I
    where
        T: ToPrimitive,
    {
        V3I {
            x: v.x.to_i32().unwrap(),
            y: v.y.to_i32().unwrap(),
            z: v.z.to_i32().unwrap(),
        }
    }
    pub fn to_cgmath<T>(&self) -> Vector3<T>
    where
        T: FromPrimitive,
    {
        Vector3::<T>::new(
            T::from_i32(self.x).unwrap(),
            T::from_i32(self.y).unwrap(),
            T::from_i32(self.z).unwrap(),
        )
    }
}
