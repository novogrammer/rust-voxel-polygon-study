use crate::{
    block::*,
    chunk::*,
    terrain_updater::{terrain_updater_b_maker, update_if, UpdaterType},
    utils,
    v2f::V2F,
    v3f::V3F,
    v3i::V3I,
};
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

// }

// macro_rules! console_log {
//   // Note that this is using the `log` function imported above during
//   // `bare_bones`
//   ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

// pub const UNIVERSE_RESOLUTION_WIDTH: usize = 8;
// pub const UNIVERSE_RESOLUTION_HEIGHT: usize = 8;
// pub const UNIVERSE_RESOLUTION_DEPTH: usize = 8;
pub const UNIVERSE_RESOLUTION_WIDTH: usize = 4;
pub const UNIVERSE_RESOLUTION_HEIGHT: usize = 4;
pub const UNIVERSE_RESOLUTION_DEPTH: usize = 4;
// pub const UNIVERSE_RESOLUTION_WIDTH: usize = 2;
// pub const UNIVERSE_RESOLUTION_HEIGHT: usize = 2;
// pub const UNIVERSE_RESOLUTION_DEPTH: usize = 2;
// pub const UNIVERSE_RESOLUTION_WIDTH: usize = 1;
// pub const UNIVERSE_RESOLUTION_HEIGHT: usize = 1;
// pub const UNIVERSE_RESOLUTION_DEPTH: usize = 1;
pub const CHUNK_LIST_LENGTH: usize =
    UNIVERSE_RESOLUTION_WIDTH * UNIVERSE_RESOLUTION_HEIGHT * UNIVERSE_RESOLUTION_DEPTH;

pub const UNIVERSE_SIZE_WIDTH: f32 = CHUNK_SIZE_WIDTH * UNIVERSE_RESOLUTION_WIDTH as f32;
pub const UNIVERSE_SIZE_HEIGHT: f32 = CHUNK_SIZE_HEIGHT * UNIVERSE_RESOLUTION_HEIGHT as f32;
pub const UNIVERSE_SIZE_DEPTH: f32 = CHUNK_SIZE_DEPTH * UNIVERSE_RESOLUTION_DEPTH as f32;

#[wasm_bindgen]
pub struct Universe {
    origin: V3F,
    size: V3F,
    chunk_resolution: V3I,
    chunk_list: Vec<Chunk>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let origin = V3F::new(
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
            origin,
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
                    let chunk =
                        Chunk::new(V3F::new(x, y, z), V3I::new(ix as i32, iy as i32, iz as i32));
                    chunk_list.push(chunk);
                }
            }
        }
        universe.chunk_list = chunk_list;
        universe
    }
    pub fn update(&mut self, time: f64) {
        let mut chunk_to_invalidate_list = vec![];

        // let my_terrain_updater = |global_position: &glam::Vec3, time: f64| -> Block {
        //     // terrain_updater_a(global_position, time)
        //     terrain_updater_b(global_position, time)
        // };
        // let my_terrain_updater: Box<UpdaterType> = terrain_updater_a_maker(time);
        let time_to_move = time;
        let my_terrain_updater: Box<UpdaterType> = update_if(
            Box::new(move |global_position: &glam::Vec3| {
                return (time_to_move % 64.0) as f32 <= global_position.x() + 32.0
                    && global_position.x() + 32.0 < ((time_to_move + 1.0) % 64.0) as f32;
            }),
            terrain_updater_b_maker(time),
        );
        for chunk in self.chunk_list.iter_mut() {
            let mut v = chunk.update(&my_terrain_updater);
            chunk_to_invalidate_list.append(&mut v);
        }
        for chunk_to_invalidate in chunk_to_invalidate_list {
            let chunk_option = self.get_mut_chunk_option_by_chunk_index(&chunk_to_invalidate);
            if let Some(chunk) = chunk_option {
                chunk.needs_draw = true;
            }
        }
    }
    fn make_block_buffer(&mut self, chunk_index: &V3I) -> Vec<Block> {
        let mut block_buffer = vec![];
        block_buffer.reserve(
            (CHUNK_RESOLUTION_DEPTH + 2)
                * (CHUNK_RESOLUTION_HEIGHT + 2)
                * (CHUNK_RESOLUTION_WIDTH + 2),
        );

        for iz in -1..(CHUNK_RESOLUTION_DEPTH as i32 + 1) {
            for iy in -1..(CHUNK_RESOLUTION_HEIGHT as i32 + 1) {
                for ix in -1..(CHUNK_RESOLUTION_WIDTH as i32 + 1) {
                    let mut block_index = V3I::new(ix, iy, iz);
                    let mut chunk_index = *chunk_index;

                    if ix < 0 {
                        block_index.set_x(ix + (CHUNK_RESOLUTION_WIDTH as i32));
                        chunk_index.set_x(chunk_index.get_x() - 1);
                    } else if CHUNK_RESOLUTION_WIDTH as i32 <= ix {
                        block_index.set_x(ix - (CHUNK_RESOLUTION_WIDTH as i32));
                        chunk_index.set_x(chunk_index.get_x() + 1);
                    }
                    if iy < 0 {
                        block_index.set_y(iy + (CHUNK_RESOLUTION_HEIGHT as i32));
                        chunk_index.set_y(chunk_index.get_y() - 1);
                    } else if CHUNK_RESOLUTION_HEIGHT as i32 <= iy {
                        block_index.set_y(iy - (CHUNK_RESOLUTION_HEIGHT as i32));
                        chunk_index.set_y(chunk_index.get_y() + 1);
                    }
                    if iz < 0 {
                        block_index.set_z(iz + (CHUNK_RESOLUTION_DEPTH as i32));
                        chunk_index.set_z(chunk_index.get_z() - 1);
                    } else if CHUNK_RESOLUTION_DEPTH as i32 <= iz {
                        block_index.set_z(iz - (CHUNK_RESOLUTION_DEPTH as i32));
                        chunk_index.set_z(chunk_index.get_z() + 1);
                    }
                    let mut cell = Block::Air;
                    let chunk_option = self.get_mut_chunk_option_by_chunk_index(&chunk_index);

                    if let Some(result_chunk) = chunk_option {
                        // console_log!("chunk!");

                        let block_option =
                            result_chunk.get_block_option_by_block_index(&block_index);
                        if let Some(result_cell) = block_option {
                            cell = *result_cell;
                            // console_log!("cell!");
                        }
                    }
                    block_buffer.push(cell);
                }
            }
        }
        // console_log!("{}", block_buffer.len());

        block_buffer
    }

    pub fn draw(&mut self) {
        let chunk_index_list: Vec<V3I> = self
            .chunk_list
            .iter()
            .map(|chunk: &Chunk| chunk.chunk_index)
            .collect();
        let block_buffer_list: Vec<Vec<Block>> = chunk_index_list
            .iter()
            .map(|chunk_index: &V3I| self.make_block_buffer(chunk_index))
            .collect();
        for (chunk, block_buffer) in self.chunk_list.iter_mut().zip(block_buffer_list.iter()) {
            chunk.draw(&block_buffer);
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
    pub fn get_geometry_buffer_uv_list_ptr(&self, i: usize) -> *const V2F {
        self.get_chunk(i).geometry_buffer.uv_list.as_ptr()
    }
    pub fn get_chunk_list_length(&self) -> usize {
        self.chunk_list.len()
    }
    pub fn get_geometry_buffer_vertex_length(&self, i: usize) -> usize {
        self.get_chunk(i).geometry_buffer.position_list.len()
    }
    pub fn get_chunk_origin(&self, i: usize) -> V3F {
        self.get_chunk(i).origin
    }
    pub fn get_chunk_size(&self, _i: usize) -> V3F {
        V3F::new(CHUNK_SIZE_WIDTH, CHUNK_SIZE_HEIGHT, CHUNK_SIZE_DEPTH)
    }
    pub fn get_geometry_version(&self, i: usize) -> u32 {
        self.get_chunk(i).version
    }
    pub fn get_max_geometry_vertex_length(&self) -> u32 {
        // 交互に存在する場合が最悪のパターン
        BLOCK_LIST_LENGTH as u32 / 2 * 6 * (3 * 2)
    }
    pub fn get_initial_geometry_vertex_length(&self) -> u32 {
        // 楽観的な数値として
        BLOCK_LIST_LENGTH as u32
        // 1 as u32
    }
    pub fn get_origin(&self) -> V3F {
        self.origin
    }
    pub fn get_size(&self) -> V3F {
        self.size
    }
    pub fn get_chunk_resolution(&self) -> V3I {
        self.chunk_resolution
    }
}

impl Universe {
    pub fn get_chunk(&self, i: usize) -> &Chunk {
        self.chunk_list.get(i).unwrap()
    }
    pub fn get_mut_chunk_option_by_chunk_index(&mut self, chunk_index: &V3I) -> Option<&mut Chunk> {
        let x = chunk_index.get_x();
        let y = chunk_index.get_y();
        let z = chunk_index.get_z();

        if x < 0 {
            return Option::None;
        } else if UNIVERSE_RESOLUTION_WIDTH as i32 <= x {
            return Option::None;
        }
        if y < 0 {
            return Option::None;
        } else if UNIVERSE_RESOLUTION_HEIGHT as i32 <= y {
            return Option::None;
        }
        if z < 0 {
            return Option::None;
        } else if UNIVERSE_RESOLUTION_DEPTH as i32 <= z {
            return Option::None;
        }
        let i = (UNIVERSE_RESOLUTION_HEIGHT as i32) * (UNIVERSE_RESOLUTION_WIDTH as i32) * z
            + y * (UNIVERSE_RESOLUTION_WIDTH as i32)
            + x;
        self.chunk_list.get_mut(i as usize)
    }
}
