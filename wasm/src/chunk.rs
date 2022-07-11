use wasm_bindgen::prelude::*;

use cgmath::Vector3;

use std::collections::HashSet;
use std::iter::FromIterator;

use crate::block::*;
use crate::geometry::Geometry;
use crate::geometry_buffer::GeometryBuffer;
use crate::universe::*;
use crate::v3f::V3F;
use crate::v3i::V3I;
use crate::vertex::Vertex;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// pub const CHUNK_RESOLUTION_WIDTH: usize = 32;
// pub const CHUNK_RESOLUTION_HEIGHT: usize = 32;
// pub const CHUNK_RESOLUTION_DEPTH: usize = 32;
// pub const CHUNK_RESOLUTION_WIDTH: usize = 16;
// pub const CHUNK_RESOLUTION_HEIGHT: usize = 16;
// pub const CHUNK_RESOLUTION_DEPTH: usize = 16;
pub const CHUNK_RESOLUTION_WIDTH: usize = 4;
pub const CHUNK_RESOLUTION_HEIGHT: usize = 4;
pub const CHUNK_RESOLUTION_DEPTH: usize = 4;
// pub const CHUNK_RESOLUTION_WIDTH: usize = 4;
// pub const CHUNK_RESOLUTION_HEIGHT: usize = 4;
// pub const CHUNK_RESOLUTION_DEPTH: usize = 4;
pub const BLOCK_LIST_LENGTH: usize =
    CHUNK_RESOLUTION_WIDTH * CHUNK_RESOLUTION_HEIGHT * CHUNK_RESOLUTION_DEPTH;

pub const CHUNK_SIZE_WIDTH: f32 = BLOCK_SIZE_WIDTH * CHUNK_RESOLUTION_WIDTH as f32;
pub const CHUNK_SIZE_HEIGHT: f32 = BLOCK_SIZE_HEIGHT * CHUNK_RESOLUTION_HEIGHT as f32;
pub const CHUNK_SIZE_DEPTH: f32 = BLOCK_SIZE_DEPTH * CHUNK_RESOLUTION_DEPTH as f32;

pub struct Chunk {
    // pub parent: *mut Universe,
    pub origin: V3F,
    pub chunk_index: V3I,
    pub block_list: Vec<Block>,
    pub geometry: Geometry,
    pub geometry_buffer: GeometryBuffer,
    pub needs_draw: bool,
    pub version: u32,
}

// error #[wasm_bindgen] generic impls aren't supported
// #[wasm_bindgen]
impl Chunk {}

impl Chunk {
    pub fn new(origin: V3F, chunk_index: V3I) -> Chunk {
        // let size=V3F{
        //     x:1.0,
        //     y:1.0,
        //     z:1.0,
        // };
        // let block_resolution=V3I{
        //     x:1,
        //     y:1,
        //     z:1,
        // };
        let mut block_list = vec![];

        block_list.resize(BLOCK_LIST_LENGTH, Block::Rock);

        let geometry = Geometry {
            vertex_list: vec![],
        };
        let geometry_buffer = GeometryBuffer {
            position_list: vec![],
            normal_list: vec![],
            color_list: vec![],
        };
        Chunk {
            // parent,
            origin: origin,
            chunk_index,
            // size,
            // block_resolution,
            block_list,
            geometry,
            geometry_buffer,
            needs_draw: true,
            version: 0,
        }
    }

    pub fn make_neighbor_chunk_index_list(&mut self, ix: i32, iy: i32, iz: i32) -> Vec<V3I> {
        let mut neighbor_chunk_index_list = vec![];

        for inz in -1..(1 + 1) {
            let z = iz + inz;
            for iny in -1..(1 + 1) {
                let y = iy + iny;
                for inx in -1..(1 + 1) {
                    let x = ix + inx;
                    let mut neighbor_chunk_index = self.chunk_index.clone();
                    if x < 0 {
                        neighbor_chunk_index.set_x(neighbor_chunk_index.get_x() - 1);
                    }
                    if CHUNK_RESOLUTION_WIDTH as i32 <= x {
                        neighbor_chunk_index.set_x(neighbor_chunk_index.get_x() + 1);
                    }
                    if y < 0 {
                        neighbor_chunk_index.set_y(neighbor_chunk_index.get_y() - 1);
                    }
                    if CHUNK_RESOLUTION_HEIGHT as i32 <= y {
                        neighbor_chunk_index.set_y(neighbor_chunk_index.get_y() + 1);
                    }
                    if z < 0 {
                        neighbor_chunk_index.set_z(neighbor_chunk_index.get_z() - 1);
                    }
                    if CHUNK_RESOLUTION_DEPTH as i32 <= z {
                        neighbor_chunk_index.set_z(neighbor_chunk_index.get_z() + 1);
                    }
                    neighbor_chunk_index_list.push(neighbor_chunk_index);
                }
            }
        }
        neighbor_chunk_index_list
    }
    pub fn update(&mut self) -> Vec<V3I> {
        let mut neighbor_chunk_index_hash = HashSet::new();
        let mut i = 0;
        for iz in 0..(CHUNK_RESOLUTION_DEPTH as i32) {
            for iy in 0..(CHUNK_RESOLUTION_HEIGHT as i32) {
                for ix in 0..(CHUNK_RESOLUTION_WIDTH as i32) {
                    let cell = self.block_list.get_mut(i).unwrap();
                    let mut next_cell = Block::Air;
                    if iz + iy + ix < CHUNK_RESOLUTION_WIDTH as i32 {
                        next_cell = Block::Rock;
                    }
                    if *cell != next_cell {
                        // needs_draw
                        *cell = next_cell;

                        let v = self.make_neighbor_chunk_index_list(ix, iy, iz);
                        for chunk_index in &v {
                            neighbor_chunk_index_hash.insert(*chunk_index);
                        }
                    }
                    i += 1;
                }
            }
        }
        let neighbor_chunk_index_list = Vec::from_iter(neighbor_chunk_index_hash.into_iter());
        neighbor_chunk_index_list
    }
    pub fn get_block_option_by_block_index(&self, block_index: &V3I) -> Option<&Block> {
        let x = block_index.get_x();
        let y = block_index.get_y();
        let z = block_index.get_z();
        if x < 0 {
            return Option::None;
        } else if CHUNK_RESOLUTION_WIDTH as i32 <= x {
            return Option::None;
        }
        if y < 0 {
            return Option::None;
        } else if CHUNK_RESOLUTION_HEIGHT as i32 <= y {
            return Option::None;
        }
        if z < 0 {
            return Option::None;
        } else if CHUNK_RESOLUTION_DEPTH as i32 <= z {
            return Option::None;
        }

        let i = (CHUNK_RESOLUTION_HEIGHT as i32) * (CHUNK_RESOLUTION_WIDTH as i32) * z
            + y * (CHUNK_RESOLUTION_WIDTH as i32)
            + x;
        return self.block_list.get(i as usize);
    }
    fn draw_geometry(&mut self, block_buffer: &Vec<Block>) {
        let mut vertex_list = vec![];
        let mut count = 0;

        for iz in 0..(CHUNK_RESOLUTION_DEPTH as i32) {
            for iy in 0..(CHUNK_RESOLUTION_HEIGHT as i32) {
                for ix in 0..(CHUNK_RESOLUTION_WIDTH as i32) {
                    // let i = (CHUNK_RESOLUTION_HEIGHT as i32) * (CHUNK_RESOLUTION_WIDTH as i32) * iz
                    //     + iy * (CHUNK_RESOLUTION_WIDTH as i32)
                    //     + ix;
                    // let cell = self.block_list.get(i as usize).unwrap();
                    let i = (CHUNK_RESOLUTION_HEIGHT as i32 + 2)
                        * (CHUNK_RESOLUTION_WIDTH as i32 + 2)
                        * (iz + 1)
                        + (iy + 1) * (CHUNK_RESOLUTION_WIDTH as i32 + 2)
                        + (ix + 1);
                    let cell = block_buffer.get(i as usize).unwrap();
                    let position = Vector3::<f32>::new(ix as f32, iy as f32, iz as f32);

                    // for now
                    if *cell == Block::Rock {
                        count += 1;
                        vertex_list.push(Vertex {
                            position: V3F::new(
                                -0.5 + position.x,
                                -0.5 + position.y,
                                0.0 + position.z,
                            ),
                            normal: V3F::new(0.0, 0.0, 1.0),
                            color: V3F::new(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0),
                        });
                        vertex_list.push(Vertex {
                            position: V3F::new(
                                0.5 + position.x,
                                -0.5 + position.y,
                                0.0 + position.z,
                            ),
                            normal: V3F::new(0.0, 0.0, 1.0),
                            color: V3F::new(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0),
                        });
                        vertex_list.push(Vertex {
                            position: V3F::new(
                                0.0 + position.x,
                                0.5 + position.y,
                                0.0 + position.z,
                            ),
                            normal: V3F::new(0.0, 0.0, 1.0),
                            color: V3F::new(255.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0),
                        });
                    }
                }
            }
        }
        // console_log!("{}", count);

        self.geometry.vertex_list = vertex_list;
    }
    fn copy_to_geometry_buffer(&mut self) {
        let mut position_list = vec![];
        let mut normal_list = vec![];
        let mut color_list = vec![];
        for vertex in &self.geometry.vertex_list {
            position_list.push(vertex.position.clone());
            normal_list.push(vertex.normal.clone());
            color_list.push(vertex.color.clone());
        }
        self.geometry_buffer.position_list = position_list;
        self.geometry_buffer.normal_list = normal_list;
        self.geometry_buffer.color_list = color_list;
        self.version += 1;
    }
    pub fn draw(&mut self, block_buffer: &Vec<Block>) {
        if !self.needs_draw {
            return;
        }
        self.draw_geometry(block_buffer);
        self.copy_to_geometry_buffer();
        self.needs_draw = false;
    }
}
