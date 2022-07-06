use crate::{chunk::*, utils, v3f::V3F, v3i::V3I};
use wasm_bindgen::prelude::*;

pub const UNIVERSE_RESOLUTION_WIDTH: usize = 2;
pub const UNIVERSE_RESOLUTION_HEIGHT: usize = 2;
pub const UNIVERSE_RESOLUTION_DEPTH: usize = 2;
pub const CHUNK_LIST_LENGTH: usize =
    UNIVERSE_RESOLUTION_WIDTH * UNIVERSE_RESOLUTION_HEIGHT * UNIVERSE_RESOLUTION_DEPTH;

pub const UNIVERSE_SIZE_WIDTH: f32 = CHUNK_SIZE_WIDTH * UNIVERSE_RESOLUTION_WIDTH as f32;
pub const UNIVERSE_SIZE_HEIGHT: f32 = CHUNK_SIZE_HEIGHT * UNIVERSE_RESOLUTION_HEIGHT as f32;
pub const UNIVERSE_SIZE_DEPTH: f32 = CHUNK_SIZE_DEPTH * UNIVERSE_RESOLUTION_DEPTH as f32;

#[wasm_bindgen]
pub struct Universe {
    position: V3F,
    size: V3F,
    chunk_resolution: V3I,
    chunk_list: Vec<Chunk>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let position = V3F::new(
            UNIVERSE_SIZE_WIDTH * -0.5,
            UNIVERSE_SIZE_HEIGHT * -0.5,
            UNIVERSE_SIZE_DEPTH * -0.5,
        );
        let size = V3F::new(
            UNIVERSE_SIZE_WIDTH,
            UNIVERSE_SIZE_HEIGHT,
            UNIVERSE_SIZE_DEPTH,
        );
        let chunk_resolution = V3I::new(
            UNIVERSE_RESOLUTION_WIDTH as i32,
            UNIVERSE_RESOLUTION_HEIGHT as i32,
            UNIVERSE_RESOLUTION_DEPTH as i32,
        );
        let mut universe = Universe {
            position,
            size,
            chunk_resolution,
            chunk_list: vec![],
        };

        let mut chunk_list = vec![];
        for iz in 0..UNIVERSE_RESOLUTION_DEPTH {
            let z = CHUNK_SIZE_DEPTH * iz as f32 + UNIVERSE_SIZE_DEPTH * -0.5;
            for iy in 0..UNIVERSE_RESOLUTION_HEIGHT {
                let y = CHUNK_SIZE_HEIGHT * iy as f32 + UNIVERSE_SIZE_HEIGHT * -0.5;
                for ix in 0..UNIVERSE_RESOLUTION_WIDTH {
                    let x = CHUNK_SIZE_WIDTH * ix as f32 + UNIVERSE_SIZE_WIDTH * -0.5;
                    let chunk = Chunk::new(
                        &mut universe,
                        V3F::new(x, y, z),
                        V3I::new(ix as i32, iy as i32, iz as i32),
                    );
                    chunk_list.push(chunk);
                }
            }
        }
        universe.chunk_list = chunk_list;
        universe
    }
    pub fn update(&mut self) {
        for chunk in self.chunk_list.iter_mut() {
            chunk.update();
        }
    }
    pub fn draw(&mut self, position: &V3F) {
        for chunk in self.chunk_list.iter_mut() {
            chunk.draw(&position);
        }
    }

    pub fn get_geometry_buffer_position_list_ptr(&self, i: usize) -> *const V3F {
        self.get_chunk(i).geometry_buffer.position_list.as_ptr()
    }
    pub fn get_geometry_buffer_normal_list_ptr(&self, i: usize) -> *const V3F {
        self.get_chunk(i).geometry_buffer.normal_list.as_ptr()
    }
    pub fn get_geometry_buffer_color_list_ptr(&self, i: usize) -> *const V3F {
        self.get_chunk(i).geometry_buffer.color_list.as_ptr()
    }
    pub fn get_chunk_list_length(&self) -> usize {
        CHUNK_LIST_LENGTH
    }
    pub fn get_geometry_buffer_vertex_length(&self, i: usize) -> usize {
        self.get_chunk(i).geometry_buffer.position_list.len()
    }
    pub fn get_chunk_origin(&self, i: usize) -> V3F {
        self.get_chunk(i).origin.clone()
    }
    pub fn get_geometry_version(&self, i: usize) -> u32 {
        self.get_chunk(i).version
    }
}

impl Universe {
    pub fn get_chunk(&self, i: usize) -> &Chunk {
        self.chunk_list.get(i).unwrap()
    }
    pub fn get_mut_chunk_option_by_chunk_index(&mut self, chunk_index: &V3I) -> Option<&mut Chunk> {
        if chunk_index.get_x() < 0 {
            return Option::None;
        }
        if UNIVERSE_RESOLUTION_WIDTH as i32 <= chunk_index.get_x() {
            return Option::None;
        }
        if chunk_index.get_y() < 0 {
            return Option::None;
        }
        if UNIVERSE_RESOLUTION_HEIGHT as i32 <= chunk_index.get_y() {
            return Option::None;
        }
        if chunk_index.get_z() < 0 {
            return Option::None;
        }
        if UNIVERSE_RESOLUTION_DEPTH as i32 <= chunk_index.get_z() {
            return Option::None;
        }
        let i = (UNIVERSE_RESOLUTION_HEIGHT as i32)
            * (UNIVERSE_RESOLUTION_WIDTH as i32)
            * chunk_index.get_z()
            + chunk_index.get_y() * (UNIVERSE_RESOLUTION_WIDTH as i32)
            + chunk_index.get_x();
        self.chunk_list.get_mut(i as usize)
    }
}
