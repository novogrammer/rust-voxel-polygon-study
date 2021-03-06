use wasm_bindgen::prelude::*;

//use cgmath::{vec3, Matrix4, SquareMatrix, Transform, Transform3, Vector3};
use cgmath::*;

use crate::block::*;
use crate::geometry::Geometry;
use crate::geometry_buffer::GeometryBuffer;
use crate::v2f::V2F;
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

// pub const CHUNK_RESOLUTION_WIDTH: usize = 64;
// pub const CHUNK_RESOLUTION_HEIGHT: usize = 64;
// pub const CHUNK_RESOLUTION_DEPTH: usize = 64;
pub const CHUNK_RESOLUTION_WIDTH: usize = 32;
pub const CHUNK_RESOLUTION_HEIGHT: usize = 32;
pub const CHUNK_RESOLUTION_DEPTH: usize = 32;
// pub const CHUNK_RESOLUTION_WIDTH: usize = 16;
// pub const CHUNK_RESOLUTION_HEIGHT: usize = 16;
// pub const CHUNK_RESOLUTION_DEPTH: usize = 16;
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

        block_list.resize(BLOCK_LIST_LENGTH, Block::Air);

        let geometry = Geometry {
            vertex_list: vec![],
        };
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
            geometry,
            geometry_buffer,
            needs_draw: true,
            version: 0,
        }
    }
    pub fn calc_index_by_position(&self, position: &V3F) -> V3I {
        let p = position.to_cgmath();
        let block_index = p - vec3::<f32>(0.5, 0.5, 0.5);
        V3I::new(
            (block_index.x + 0.5).floor() as i32,
            (block_index.y + 0.5).floor() as i32,
            (block_index.z + 0.5).floor() as i32,
        )
    }

    pub fn calc_index_by_global_position(&self, position: &V3F) -> V3I {
        let o = self.origin.to_cgmath();
        let p = position.to_cgmath();
        let block_index = p - vec3::<f32>(0.5, 0.5, 0.5) - o;
        V3I::new(
            (block_index.x + 0.5).floor() as i32,
            (block_index.y + 0.5).floor() as i32,
            (block_index.z + 0.5).floor() as i32,
        )
    }
    pub fn calc_position_by_index(&self, block_index: &V3I) -> V3F {
        let p = vec3(
            block_index.get_x() as f32,
            block_index.get_y() as f32,
            block_index.get_z() as f32,
        );
        let position = p + vec3::<f32>(0.5, 0.5, 0.5);
        V3F::from_cgmath(&position)
    }
    pub fn calc_global_position_by_index(&self, block_index: &V3I) -> V3F {
        let o = self.origin.to_cgmath();
        let p = vec3(
            block_index.get_x() as f32,
            block_index.get_y() as f32,
            block_index.get_z() as f32,
        );
        let position = p + o + vec3::<f32>(0.5, 0.5, 0.5);
        V3F::from_cgmath(&position)
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
    pub fn update(
        &mut self,
        terrain_updater: fn(global_position: &Vector3<f32>, time: f64) -> Block,
        time: f64,
    ) -> Vec<V3I> {
        let mut chunk_index_and_invalidate_list = vec![];
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
                    let position = position.to_cgmath::<f32>();
                    let cell = self.block_list.get_mut(i).unwrap();
                    let next_cell = terrain_updater(&position, time);
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
    fn calc_index_for_ao(
        &mut self,
        front_face_position: &Vector3<f32>,
        multiplier: Vector3<f32>,
        matrix_for_direction: &Matrix4<f32>,
        position: &Vector3<f32>,
    ) -> V3I {
        self.calc_index_by_position(&V3F::from_cgmath(
            &(position
                + matrix_for_direction
                    .transform_vector(front_face_position.mul_element_wise(multiplier))),
        ))
    }
    fn draw_geometry(&mut self, block_buffer: &Vec<Block>) {
        let mut vertex_list: Vec<Vertex> = vec![];

        // let mmm = vec3::<f32>(-0.5, -0.5, -0.5);
        let mmp = vec3::<f32>(-0.5, -0.5, 0.5);
        // let mpm = vec3::<f32>(-0.5, 0.5, -0.5);
        let mpp = vec3::<f32>(-0.5, 0.5, 0.5);
        // let pmm = vec3::<f32>(0.5, -0.5, -0.5);
        let pmp = vec3::<f32>(0.5, -0.5, 0.5);
        // let ppm = vec3::<f32>(0.5, 0.5, -0.5);
        let ppp = vec3::<f32>(0.5, 0.5, 0.5);
        let front_face_position_list = vec![mmp, pmp, mpp, ppp];
        let mm = vec2::<f32>(0.0, 0.0);
        let mp = vec2::<f32>(0.0, 1.0);
        let pm = vec2::<f32>(1.0, 0.0);
        let pp = vec2::<f32>(1.0, 1.0);
        let front_face_uv_list = vec![mm, pm, mp, pp];
        let front_face_index_list: Vec<usize> = vec![1, 3, 0, 2, 0, 3];
        let front_face_index_list_flipped: Vec<usize> = vec![0, 1, 2, 3, 2, 1];
        let front_face_normal = vec3::<f32>(0.0, 0.0, 1.0);
        // let color_pink = vec3::<f32>(1.0, 0.5, 0.5);
        // let color_lime = vec3::<f32>(0.0, 1.0, 0.5);
        // let color_white = vec3::<f32>(1.0, 1.0, 1.0);
        // let front_face_color_list = vec![
        //     color_white,
        //     color_white,
        //     color_white,
        //     color_white,
        //     color_white,
        //     color_white,
        // ];

        let matrix_for_direction_list = vec![
            Matrix4::<f32>::identity(),
            Matrix4::<f32>::from_angle_y(Deg(90.0)),
            Matrix4::<f32>::from_angle_y(Deg(-90.0)),
            Matrix4::<f32>::from_angle_y(Deg(180.0)),
            Matrix4::<f32>::from_angle_x(Deg(90.0)),
            Matrix4::<f32>::from_angle_x(Deg(-90.0)),
        ];

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
                    let position = vec3::<f32>(ix as f32 + 0.5, iy as f32 + 0.5, iz as f32 + 0.5);

                    // for now
                    if *cell != Block::Air {
                        for matrix_for_direction in &matrix_for_direction_list {
                            let normal = matrix_for_direction.transform_vector(front_face_normal);
                            let next_index = toi(
                                ix + ((normal.x + 0.5).floor() as i32),
                                iy + ((normal.y + 0.5).floor() as i32),
                                iz + ((normal.z + 0.5).floor() as i32),
                            );
                            let next_cell = block_buffer.get(next_index as usize).unwrap();
                            if *next_cell == Block::Air {
                                let quad_vertex_and_ao_list: Vec<(Vertex, i32)> =
                                    front_face_position_list
                                        .iter()
                                        .zip(front_face_uv_list.iter())
                                        .map(|(front_face_position, front_face_uv)| {
                                            let vertex_position = V3F::from_cgmath(
                                                &(position
                                                    + matrix_for_direction
                                                        .transform_vector(*front_face_position)),
                                            );
                                            let uv = front_face_uv;
                                            let side1_index = self.calc_index_for_ao(
                                                front_face_position,
                                                vec3::<f32>(2.0, 0.0, 2.0),
                                                matrix_for_direction,
                                                &position,
                                            );
                                            let side1 = index_to_ao_not_air(side1_index);
                                            let side2_index = self.calc_index_for_ao(
                                                front_face_position,
                                                vec3::<f32>(0.0, 2.0, 2.0),
                                                matrix_for_direction,
                                                &position,
                                            );
                                            let side2 = index_to_ao_not_air(side2_index);
                                            let corner_index = self.calc_index_for_ao(
                                                front_face_position,
                                                vec3::<f32>(2.0, 2.0, 2.0),
                                                matrix_for_direction,
                                                &position,
                                            );
                                            let corner = index_to_ao_not_air(corner_index);
                                            let ao = vertex_a_o(side1, side2, corner);
                                            (
                                                Vertex {
                                                    position: vertex_position,
                                                    normal: V3F::from_cgmath(&normal),
                                                    color: make_ao_color(ao),
                                                    uv: V2F::from_cgmath(uv),
                                                },
                                                ao,
                                            )
                                        })
                                        .collect();
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
                                    vertex_list.push(vertex);
                                }
                            }
                        }
                    }
                }
            }
        }

        self.geometry.vertex_list = vertex_list;
    }
    fn copy_to_geometry_buffer(&mut self) {
        let l = self.geometry.vertex_list.len();
        let mut position_list = vec![];
        position_list.reserve(l);
        let mut normal_list = vec![];
        normal_list.reserve(l);
        let mut color_list = vec![];
        color_list.reserve(l);
        let mut uv_list = vec![];
        uv_list.reserve(l);
        for vertex in &self.geometry.vertex_list {
            position_list.push(vertex.position);
            normal_list.push(vertex.normal);
            color_list.push(vertex.color);
            uv_list.push(vertex.uv);
        }
        self.geometry_buffer.position_list = position_list;
        self.geometry_buffer.normal_list = normal_list;
        self.geometry_buffer.color_list = color_list;
        self.geometry_buffer.uv_list = uv_list;
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
