use wasm_bindgen::prelude::*;

pub const BLOCK_SIZE_WIDTH: f32 = 1.0;
pub const BLOCK_SIZE_HEIGHT: f32 = 1.0;
pub const BLOCK_SIZE_DEPTH: f32 = 1.0;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    Air = 0xff,
    Weed = 0,
    Metal = 1,
    Brick = 2,
    Tile = 3,
    Dirt = 4,
    Rock = 5,
    Stone = 6,
    Sand = 7,
    Wood = 8,
    Snow = 9,
    Concrete = 10,
}
