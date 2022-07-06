use wasm_bindgen::prelude::*;

pub const CELL_SIZE_WIDTH: f32 = 1.0;
pub const CELL_SIZE_HEIGHT: f32 = 1.0;
pub const CELL_SIZE_DEPTH: f32 = 1.0;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Air = 0,
    Rock = 1,
}
