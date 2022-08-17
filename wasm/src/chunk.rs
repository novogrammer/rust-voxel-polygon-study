// use wasm_bindgen::prelude::*;

use crate::block::*;
use crate::geometry_buffer::GeometryBuffer;
use crate::terrain_updater::UpdaterType;
use crate::v2f::V2F;
use crate::v3f::V3F;
use crate::v3i::V3I;
use crate::vertex::Vertex;

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

// pub const CHUNK_RESOLUTION_WIDTH: usize = 64;
// pub const CHUNK_RESOLUTION_HEIGHT: usize = 64;
// pub const CHUNK_RESOLUTION_DEPTH: usize = 64;
// pub const CHUNK_RESOLUTION_WIDTH: usize = 32;
// pub const CHUNK_RESOLUTION_HEIGHT: usize = 32;
// pub const CHUNK_RESOLUTION_DEPTH: usize = 32;
pub const CHUNK_RESOLUTION_WIDTH: usize = 16;
pub const CHUNK_RESOLUTION_HEIGHT: usize = 16;
pub const CHUNK_RESOLUTION_DEPTH: usize = 16;
// pub const CHUNK_RESOLUTION_WIDTH: usize = 8;
// pub const CHUNK_RESOLUTION_HEIGHT: usize = 8;
// pub const CHUNK_RESOLUTION_DEPTH: usize = 8;
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
    // pub geometry: Geometry,
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

        block_list.resize(BLOCK_LIST_LENGTH, Block::Air);

        let geometry_buffer = GeometryBuffer {
            position_list: vec![],
            normal_list: vec![],
            color_list: vec![],
            uv_list: vec![],
        };
        Chunk {
            // parent,
            origin: origin,
            chunk_index,
            // size,
            // block_resolution,
            block_list,
            geometry_buffer,
            needs_draw: true,
            version: 0,
        }
    }
    pub fn calc_index_by_position(&self, position: &V3F) -> V3I {
        // let p = position.to_cgmath();
        // let block_index = p - vec3::<f32>(0.5, 0.5, 0.5);
        // V3I::new(
        //     (block_index.x + 0.5).floor() as i32,
        //     (block_index.y + 0.5).floor() as i32,
        //     (block_index.z + 0.5).floor() as i32,
        // )
        V3I::new(
            (position.get_x()).floor() as i32,
            (position.get_y()).floor() as i32,
            (position.get_z()).floor() as i32,
        )
    }

    pub fn calc_index_by_global_position(&self, position: &V3F) -> V3I {
        // let o = self.origin.to_cgmath();
        // let p = position.to_cgmath();
        // let block_index = p - vec3::<f32>(0.5, 0.5, 0.5) - o;
        // V3I::new(
        //     (block_index.x + 0.5).floor() as i32,
        //     (block_index.y + 0.5).floor() as i32,
        //     (block_index.z + 0.5).floor() as i32,
        // )
        V3I::new(
            (position.get_x() - self.origin.get_x()).floor() as i32,
            (position.get_y() - self.origin.get_y()).floor() as i32,
            (position.get_z() - self.origin.get_z()).floor() as i32,
        )
    }
    pub fn calc_position_by_index(&self, block_index: &V3I) -> V3F {
        // let p = vec3(
        //     block_index.get_x() as f32,
        //     block_index.get_y() as f32,
        //     block_index.get_z() as f32,
        // );
        // let position = p + vec3::<f32>(0.5, 0.5, 0.5);
        // V3F::from_cgmath(&position)
        V3F::new(
            block_index.get_x() as f32 + 0.5,
            block_index.get_y() as f32 + 0.5,
            block_index.get_z() as f32 + 0.5,
        )
    }
    pub fn calc_global_position_by_index(&self, block_index: &V3I) -> V3F {
        // let o = self.origin.to_cgmath();
        // let p = vec3(
        //     block_index.get_x() as f32,
        //     block_index.get_y() as f32,
        //     block_index.get_z() as f32,
        // );
        // let position = p + o + vec3::<f32>(0.5, 0.5, 0.5);
        // V3F::from_cgmath(&position)
        V3F::new(
            block_index.get_x() as f32 + self.origin.get_x() + 0.5,
            block_index.get_y() as f32 + self.origin.get_y() + 0.5,
            block_index.get_z() as f32 + self.origin.get_z() + 0.5,
        )
    }

    pub fn make_neighbor_chunk_index_list(&mut self, block_index: &V3I) -> Vec<V3I> {
        let mut neighbor_chunk_index_list = vec![];
        neighbor_chunk_index_list.reserve(3 * 3 * 3);

        for inz in -1..(1 + 1) {
            let z = block_index.get_z() + inz;
            for iny in -1..(1 + 1) {
                let y = block_index.get_y() + iny;
                for inx in -1..(1 + 1) {
                    let x = block_index.get_x() + inx;
                    let mut neighbor_chunk_index = self.chunk_index;
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
    pub fn update(&mut self, terrain_updater: &Box<UpdaterType>) -> Vec<V3I> {
        let mut chunk_index_and_invalidate_list = vec![];
        chunk_index_and_invalidate_list.reserve(3 * 3 * 3);
        for iz in -1..(1 + 1) {
            for iy in -1..(1 + 1) {
                for ix in -1..(1 + 1) {
                    let mut chunk_index = self.chunk_index;
                    chunk_index.set_x(chunk_index.get_x() + ix);
                    chunk_index.set_y(chunk_index.get_y() + iy);
                    chunk_index.set_z(chunk_index.get_z() + iz);
                    chunk_index_and_invalidate_list.push((chunk_index, false))
                }
            }
        }

        let mut i = 0;
        for iz in 0..(CHUNK_RESOLUTION_DEPTH as i32) {
            for iy in 0..(CHUNK_RESOLUTION_HEIGHT as i32) {
                for ix in 0..(CHUNK_RESOLUTION_WIDTH as i32) {
                    let block_index = V3I::new(ix, iy, iz);
                    let position = self.calc_global_position_by_index(&block_index);
                    let position = position.to_glam();
                    let cell = self.block_list.get_mut(i).unwrap();
                    let next_cell_option = terrain_updater(&position);
                    if let Some(next_cell) = next_cell_option {
                        if *cell != next_cell {
                            // needs_draw
                            *cell = next_cell;
                            let block_index = V3I::new(ix, iy, iz);
                            let v = self.make_neighbor_chunk_index_list(&block_index);
                            for (chunk_index_to_invalidate, is_invalidate) in
                                &mut chunk_index_and_invalidate_list
                            {
                                for chunk_index in &v {
                                    if *chunk_index_to_invalidate == *chunk_index {
                                        *is_invalidate = true;
                                    }
                                }
                            }
                        }
                    }
                    i += 1;
                }
            }
        }
        let chunk_index_to_invalidate_list: Vec<V3I> = chunk_index_and_invalidate_list
            .iter()
            .filter(|(_ci, i)| *i == true)
            .map(|(ci, _i)| *ci)
            .collect();
        chunk_index_to_invalidate_list
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
    pub fn draw(&mut self, block_buffer: &Vec<Block>) {
        if !self.needs_draw {
            return;
        }
        // 楽観的な数値として
        let l = BLOCK_LIST_LENGTH * 2;
        let mut position_list = vec![];
        position_list.reserve(l);
        let mut normal_list = vec![];
        normal_list.reserve(l);
        let mut color_list = vec![];
        color_list.reserve(l);
        let mut uv_list = vec![];
        uv_list.reserve(l);

        // let mmm = glam::vec3(-0.5, -0.5, -0.5);
        let mmp = glam::vec3(-0.5, -0.5, 0.5);
        // let mpm = glam::vec3(-0.5, 0.5, -0.5);
        let mpp = glam::vec3(-0.5, 0.5, 0.5);
        // let pmm = glam::vec3(0.5, -0.5, -0.5);
        let pmp = glam::vec3(0.5, -0.5, 0.5);
        // let ppm = glam::vec3(0.5, 0.5, -0.5);
        let ppp = glam::vec3(0.5, 0.5, 0.5);
        let front_face_position_list = vec![mmp, pmp, mpp, ppp];
        let mm = glam::vec2(0.0, 0.0);
        let mp = glam::vec2(0.0, 1.0);
        let pm = glam::vec2(1.0, 0.0);
        let pp = glam::vec2(1.0, 1.0);
        let base_uv_list = vec![mm, pm, mp, pp];
        let remap_uv = |uv: &glam::Vec2| {
            let each_image_size = 256.0;
            let padding = 8.0;
            let one = 1.0 / each_image_size;
            let result =
                *uv * (1.0 - one * padding * 2.0) + glam::vec2(one * padding, one * padding);
            result
        };

        let weed_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.0, 0.75))
            .collect();

        let metal_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.25, 0.75))
            .collect();

        let brick_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.5, 0.75))
            .collect();
        let tile_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.75, 0.75))
            .collect();
        let dirt_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.0, 0.5))
            .collect();
        let rock_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.25, 0.5))
            .collect();
        let stone_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.5, 0.5))
            .collect();
        let sand_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.75, 0.5))
            .collect();
        let wood_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.0, 0.25))
            .collect();
        let snow_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.25, 0.25))
            .collect();
        let concrete_front_face_uv_list: Vec<glam::Vec2> = base_uv_list
            .iter()
            .map(remap_uv)
            .map(|uv| uv * 0.25 + glam::vec2(0.5, 0.25))
            .collect();
        let front_face_index_list: Vec<usize> = vec![1, 3, 0, 2, 0, 3];
        let front_face_index_list_flipped: Vec<usize> = vec![0, 1, 2, 3, 2, 1];
        let front_face_normal = glam::vec3(0.0, 0.0, 1.0);

        let matrix_for_direction_list = vec![
            glam::Mat4::IDENTITY,
            glam::Mat4::from_rotation_y(90.0_f32.to_radians()),
            glam::Mat4::from_rotation_y(-90.0_f32.to_radians()),
            glam::Mat4::from_rotation_y(180.0_f32.to_radians()),
            glam::Mat4::from_rotation_x(90.0_f32.to_radians()),
            glam::Mat4::from_rotation_x(-90.0_f32.to_radians()),
        ];
        struct MyVertex {
            position: glam::Vec3,
            side1: glam::Vec3,
            side2: glam::Vec3,
            corner: glam::Vec3,

            weed_uv: glam::Vec2,
            metal_uv: glam::Vec2,
            brick_uv: glam::Vec2,
            tile_uv: glam::Vec2,
            dirt_uv: glam::Vec2,
            rock_uv: glam::Vec2,
            stone_uv: glam::Vec2,
            sand_uv: glam::Vec2,
            wood_uv: glam::Vec2,
            snow_uv: glam::Vec2,
            concrete_uv: glam::Vec2,
        }
        struct MyVertexListAndNormalAndMatrix {
            my_vertex_list: Vec<MyVertex>,
            normal: glam::Vec3,
            // matrix: glam::Mat4,
        }

        let my_vertex_base_list: Vec<MyVertex> = (0..4)
            .map(|i| {
                let position = *front_face_position_list.get(i).unwrap();
                MyVertex {
                    position: position,
                    side1: glam::vec3(position.x * 2.0, position.y * 0.0, position.z * 2.0),
                    side2: glam::vec3(position.x * 0.0, position.y * 2.0, position.z * 2.0),
                    corner: glam::vec3(position.x * 2.0, position.y * 2.0, position.z * 2.0),
                    weed_uv: *weed_front_face_uv_list.get(i).unwrap(),
                    metal_uv: *metal_front_face_uv_list.get(i).unwrap(),
                    brick_uv: *brick_front_face_uv_list.get(i).unwrap(),
                    tile_uv: *tile_front_face_uv_list.get(i).unwrap(),
                    dirt_uv: *dirt_front_face_uv_list.get(i).unwrap(),
                    rock_uv: *rock_front_face_uv_list.get(i).unwrap(),
                    stone_uv: *stone_front_face_uv_list.get(i).unwrap(),
                    sand_uv: *sand_front_face_uv_list.get(i).unwrap(),
                    wood_uv: *wood_front_face_uv_list.get(i).unwrap(),
                    snow_uv: *snow_front_face_uv_list.get(i).unwrap(),
                    concrete_uv: *concrete_front_face_uv_list.get(i).unwrap(),
                }
            })
            .collect();
        let my_vertex_list_and_normal_and_matrix_list: Vec<MyVertexListAndNormalAndMatrix> =
            matrix_for_direction_list
                .iter()
                .map(|matrix_for_direction| {
                    let my_vertex_list: Vec<MyVertex> = my_vertex_base_list
                        .iter()
                        .map(|my_vertex_base| {
                            let vertex_for_direction = MyVertex {
                                position: matrix_for_direction
                                    .transform_vector3(my_vertex_base.position),
                                side1: matrix_for_direction.transform_vector3(my_vertex_base.side1),
                                side2: matrix_for_direction.transform_vector3(my_vertex_base.side2),
                                corner: matrix_for_direction
                                    .transform_vector3(my_vertex_base.corner),
                                weed_uv: my_vertex_base.weed_uv,
                                metal_uv: my_vertex_base.metal_uv,
                                brick_uv: my_vertex_base.brick_uv,
                                tile_uv: my_vertex_base.tile_uv,
                                dirt_uv: my_vertex_base.dirt_uv,
                                rock_uv: my_vertex_base.rock_uv,
                                stone_uv: my_vertex_base.stone_uv,
                                sand_uv: my_vertex_base.sand_uv,
                                wood_uv: my_vertex_base.wood_uv,
                                snow_uv: my_vertex_base.snow_uv,
                                concrete_uv: my_vertex_base.concrete_uv,
                            };
                            vertex_for_direction
                        })
                        .collect::<Vec<_>>();
                    MyVertexListAndNormalAndMatrix {
                        my_vertex_list,
                        normal: matrix_for_direction.transform_vector3(front_face_normal),
                        // matrix: *matrix_for_direction,
                    }
                })
                .collect::<Vec<_>>();

        let toi = |ix: i32, iy: i32, iz: i32| {
            (CHUNK_RESOLUTION_HEIGHT as i32 + 2) * (CHUNK_RESOLUTION_WIDTH as i32 + 2) * (iz + 1)
                + (iy + 1) * (CHUNK_RESOLUTION_WIDTH as i32 + 2)
                + (ix + 1)
        };
        // see https://0fps.net/2013/07/03/ambient-occlusion-for-minecraft-like-worlds/
        let vertex_a_o = |side1: i32, side2: i32, corner: i32| {
            if side1 != 0 && side2 != 0 {
                return 0;
            }
            return 3 - (side1 + side2 + corner);
        };
        let make_ao_color = |ao| match ao {
            0 => V3F::new(0.25, 0.25, 0.25),
            1 => V3F::new(0.5, 0.5, 0.5),
            2 => V3F::new(0.75, 0.75, 0.75),
            3 => V3F::new(1.0, 1.0, 1.0),
            _ => V3F::new(0.0, 0.0, 0.0),
        };

        let index_to_ao_not_air = |index: V3I| {
            let i = toi(index.get_x(), index.get_y(), index.get_z());
            match block_buffer.get(i as usize) {
                Some(block) => {
                    if *block != Block::Air {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        };

        let is_flipped_quad = |a00: i32, a01: i32, a10: i32, a11: i32| a00 + a11 > a01 + a10;

        for iz in 0..(CHUNK_RESOLUTION_DEPTH as i32) {
            for iy in 0..(CHUNK_RESOLUTION_HEIGHT as i32) {
                for ix in 0..(CHUNK_RESOLUTION_WIDTH as i32) {
                    // let i = (CHUNK_RESOLUTION_HEIGHT as i32) * (CHUNK_RESOLUTION_WIDTH as i32) * iz
                    //     + iy * (CHUNK_RESOLUTION_WIDTH as i32)
                    //     + ix;
                    // let cell = self.block_list.get(i as usize).unwrap();
                    let i = toi(ix, iy, iz);
                    let cell = block_buffer.get(i as usize).unwrap();
                    let base_position =
                        glam::vec3(ix as f32 + 0.5, iy as f32 + 0.5, iz as f32 + 0.5);

                    if *cell != Block::Air {
                        for my_vertex_list_and_normal_and_matrix in
                            &my_vertex_list_and_normal_and_matrix_list
                        {
                            let normal = &my_vertex_list_and_normal_and_matrix.normal;
                            let next_index = toi(
                                ix + (normal.x as i32),
                                iy + (normal.y as i32),
                                iz + (normal.z as i32),
                            );
                            let next_cell = block_buffer.get(next_index as usize).unwrap();
                            if *next_cell == Block::Air {
                                let my_vertex_list =
                                    &my_vertex_list_and_normal_and_matrix.my_vertex_list;
                                // let matrix = &my_vertex_list_and_normal_and_matrix.matrix;

                                let quad_vertex_and_ao_list: Vec<(Vertex, i32)> = my_vertex_list
                                    .iter()
                                    .map(|my_vertex| {
                                        let uv = match *cell {
                                            Block::Metal => &my_vertex.metal_uv,
                                            Block::Brick => &my_vertex.brick_uv,
                                            Block::Tile => &my_vertex.tile_uv,
                                            Block::Dirt => &my_vertex.dirt_uv,
                                            Block::Rock => &my_vertex.rock_uv,
                                            Block::Stone => &my_vertex.stone_uv,
                                            Block::Sand => &my_vertex.sand_uv,
                                            Block::Wood => &my_vertex.wood_uv,
                                            Block::Snow => &my_vertex.snow_uv,
                                            Block::Concrete => &my_vertex.concrete_uv,
                                            _ => &my_vertex.weed_uv,
                                        };
                                        let side1_index = self.calc_index_by_position(
                                            &V3F::from_glam(&(base_position + my_vertex.side1)),
                                        );
                                        let side1 = index_to_ao_not_air(side1_index);
                                        let side2_index = self.calc_index_by_position(
                                            &V3F::from_glam(&(base_position + my_vertex.side2)),
                                        );
                                        let side2 = index_to_ao_not_air(side2_index);
                                        let corner_index = self.calc_index_by_position(
                                            &V3F::from_glam(&(base_position + my_vertex.corner)),
                                        );
                                        let corner = index_to_ao_not_air(corner_index);
                                        let ao = vertex_a_o(side1, side2, corner);

                                        (
                                            Vertex {
                                                position: V3F::from_glam(
                                                    &(base_position + my_vertex.position),
                                                ),
                                                normal: V3F::from_glam(&normal),
                                                uv: V2F::from_glam(&uv),
                                                color: make_ao_color(ao),
                                            },
                                            ao,
                                        )
                                    })
                                    .collect::<Vec<_>>();
                                let quad_vertex_list: Vec<&Vertex> = quad_vertex_and_ao_list
                                    .iter()
                                    .map(|(quad_vertex, _ao)| quad_vertex)
                                    .collect();
                                let ao_list: Vec<i32> = quad_vertex_and_ao_list
                                    .iter()
                                    .map(|(_quad_vertex, ao)| *ao)
                                    .collect();
                                let face_index_list = if is_flipped_quad(
                                    *ao_list.get(0).unwrap(),
                                    *ao_list.get(1).unwrap(),
                                    *ao_list.get(2).unwrap(),
                                    *ao_list.get(3).unwrap(),
                                ) {
                                    &front_face_index_list
                                } else {
                                    &front_face_index_list_flipped
                                };
                                for front_face_index in face_index_list {
                                    let vertex = **quad_vertex_list.get(*front_face_index).unwrap();

                                    position_list.push(vertex.position);
                                    normal_list.push(vertex.normal);
                                    color_list.push(vertex.color);
                                    uv_list.push(vertex.uv);
                                }
                            }
                        }
                    }
                }
            }
        }
        // // self.geometry.vertex_list = vertex_list;
        // std::mem::swap(&mut self.geometry.vertex_list, &mut vertex_list);

        // self.geometry_buffer.position_list = position_list;
        std::mem::swap(&mut self.geometry_buffer.position_list, &mut position_list);
        // self.geometry_buffer.normal_list = normal_list;
        std::mem::swap(&mut self.geometry_buffer.normal_list, &mut normal_list);
        // self.geometry_buffer.color_list = color_list;
        std::mem::swap(&mut self.geometry_buffer.color_list, &mut color_list);
        // self.geometry_buffer.uv_list = uv_list;
        std::mem::swap(&mut self.geometry_buffer.uv_list, &mut uv_list);

        self.needs_draw = false;
        self.version += 1;
    }
}
