use crate::cell::*;
use crate::geometry::Geometry;
use crate::geometry_buffer::GeometryBuffer;
use crate::universe::*;
use crate::v3f::V3F;
use crate::v3i::V3I;
use crate::vertex::Vertex;

pub const CHUNK_RESOLUTION_WIDTH: usize = 32;
pub const CHUNK_RESOLUTION_HEIGHT: usize = 32;
pub const CHUNK_RESOLUTION_DEPTH: usize = 32;
pub const CELL_LIST_LENGTH: usize =
    CHUNK_RESOLUTION_WIDTH * CHUNK_RESOLUTION_HEIGHT * CHUNK_RESOLUTION_DEPTH;

pub const CHUNK_SIZE_WIDTH: f32 = CELL_SIZE_WIDTH * CHUNK_RESOLUTION_WIDTH as f32;
pub const CHUNK_SIZE_HEIGHT: f32 = CELL_SIZE_HEIGHT * CHUNK_RESOLUTION_HEIGHT as f32;
pub const CHUNK_SIZE_DEPTH: f32 = CELL_SIZE_DEPTH * CHUNK_RESOLUTION_DEPTH as f32;

pub struct Chunk {
    pub parent: *mut Universe,
    pub origin: V3F,
    pub chunk_index: V3I,
    pub cell_list: Vec<Cell>,
    pub geometry: Geometry,
    pub geometry_buffer: GeometryBuffer,
    pub needs_draw: bool,
    pub version: u32,
}

// error #[wasm_bindgen] generic impls aren't supported
// #[wasm_bindgen]
impl Chunk {}

impl Chunk {
    pub fn new(parent: *mut Universe, origin: V3F, chunk_index: V3I) -> Chunk {
        // let size=V3F{
        //     x:1.0,
        //     y:1.0,
        //     z:1.0,
        // };
        // let cell_resolution=V3I{
        //     x:1,
        //     y:1,
        //     z:1,
        // };
        let mut cell_list = vec![];

        cell_list.resize(CELL_LIST_LENGTH, Cell::Rock);

        let geometry = Geometry {
            vertex_list: vec![],
        };
        let geometry_buffer = GeometryBuffer {
            position_list: vec![],
            normal_list: vec![],
            color_list: vec![],
        };
        Chunk {
            parent,
            origin: origin,
            chunk_index,
            // size,
            // cell_resolution,
            cell_list,
            geometry,
            geometry_buffer,
            needs_draw: true,
            version: 0,
        }
    }

    pub fn invalidate_neighbors(&mut self, ix: i32, iy: i32, iz: i32) {
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
                    unsafe {
                        let parent = self.parent.as_mut().unwrap();
                        let neighbor_chunk_option =
                            parent.get_mut_chunk_option_by_chunk_index(&neighbor_chunk_index);
                        if let Some(neighbor_chunk) = neighbor_chunk_option {
                            neighbor_chunk.needs_draw = true
                        }
                    }
                }
            }
        }
    }
    pub fn update(&mut self) {
        let mut i = 0;
        for iz in 0..(CHUNK_RESOLUTION_DEPTH as i32) {
            for iy in 0..(CHUNK_RESOLUTION_HEIGHT as i32) {
                for ix in 0..(CHUNK_RESOLUTION_WIDTH as i32) {
                    let cell = self.cell_list.get_mut(i).unwrap();
                    let mut next_cell = Cell::Air;
                    if iz + iy + ix <= 32 {
                        next_cell = Cell::Rock;
                    }
                    if *cell != next_cell {
                        // needs_draw
                        *cell = next_cell;
                        self.invalidate_neighbors(ix, iy, iz);
                    }
                    i += 1;
                }
            }
        }
    }
    pub fn get_cell_option_by_cell_index(&self, cell_index: &V3I) -> Option<&Cell> {
        let x = cell_index.get_x();
        let y = cell_index.get_y();
        let z = cell_index.get_z();
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
        return self.cell_list.get(i as usize);
    }
    fn get_cell_buffer(&mut self) -> Vec<Cell> {
        let mut cell_buffer = vec![];
        for iz in -1..(CHUNK_RESOLUTION_DEPTH as i32 + 1) {
            for iy in -1..(CHUNK_RESOLUTION_HEIGHT as i32 + 1) {
                for ix in -1..(CHUNK_RESOLUTION_WIDTH as i32 + 1) {
                    let mut cell_index = V3I::new(ix, iy, iz);
                    let mut chunk_index = self.chunk_index.clone();

                    if ix < 0 {
                        chunk_index.set_x(ix + (CHUNK_RESOLUTION_WIDTH as i32));
                        cell_index.set_x(cell_index.get_x() - 1);
                    } else if CHUNK_RESOLUTION_WIDTH as i32 <= ix {
                        chunk_index.set_x(ix - (CHUNK_RESOLUTION_WIDTH as i32));
                        cell_index.set_x(cell_index.get_x() + 1);
                    }
                    if iy < 0 {
                        chunk_index.set_y(iy + (CHUNK_RESOLUTION_HEIGHT as i32));
                        cell_index.set_y(cell_index.get_y() - 1);
                    } else if CHUNK_RESOLUTION_HEIGHT as i32 <= iy {
                        chunk_index.set_y(iy - (CHUNK_RESOLUTION_HEIGHT as i32));
                        cell_index.set_y(cell_index.get_y() + 1);
                    }
                    if iz < 0 {
                        chunk_index.set_z(iz + (CHUNK_RESOLUTION_DEPTH as i32));
                        cell_index.set_z(cell_index.get_z() - 1);
                    } else if CHUNK_RESOLUTION_DEPTH as i32 <= iz {
                        chunk_index.set_z(iz - (CHUNK_RESOLUTION_DEPTH as i32));
                        cell_index.set_z(cell_index.get_z() + 1);
                    }
                    let mut cell = Cell::Air;
                    unsafe {
                        let chunk_option = self
                            .parent
                            .as_mut()
                            .unwrap()
                            .get_mut_chunk_option_by_chunk_index(&chunk_index);
                        if let Some(result_chunk) = chunk_option {
                            let cell_option =
                                result_chunk.get_cell_option_by_cell_index(&cell_index);
                            if let Some(result_cell) = cell_option {
                                cell = *result_cell;
                            }
                        }
                    }
                    cell_buffer.push(cell);
                }
            }
        }
        cell_buffer
    }
    fn draw_geometry(&mut self, _position: &V3F) {
        let cell_buffer = self.get_cell_buffer();
        let mut vertex_list = vec![];

        for iz in 0..(CHUNK_RESOLUTION_DEPTH as i32) {
            for iy in 0..(CHUNK_RESOLUTION_HEIGHT as i32) {
                for ix in 0..(CHUNK_RESOLUTION_WIDTH as i32) {
                    let i = (CHUNK_RESOLUTION_HEIGHT as i32) * (CHUNK_RESOLUTION_WIDTH as i32) * iz
                        + iy * (CHUNK_RESOLUTION_WIDTH as i32)
                        + ix;
                    let position = V3F::new(ix as f32, iy as f32, iz as f32);
                    let cell = self.cell_list.get(i as usize).unwrap();
                    // for now
                    if *cell == Cell::Rock {
                        vertex_list.push(Vertex {
                            position: V3F::new(
                                -0.5 + position.get_x(),
                                -1.0 + position.get_y(),
                                0.0 + position.get_z(),
                            ),
                            normal: V3F::new(0.0, 0.0, 1.0),
                            color: V3F::new(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0),
                        });
                        vertex_list.push(Vertex {
                            position: V3F::new(
                                0.5 + position.get_x(),
                                -1.0 + position.get_y(),
                                0.0 + position.get_z(),
                            ),
                            normal: V3F::new(0.0, 0.0, 1.0),
                            color: V3F::new(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0),
                        });
                        vertex_list.push(Vertex {
                            position: V3F::new(
                                0.0 + position.get_x(),
                                1.0 + position.get_y(),
                                0.0 + position.get_z(),
                            ),
                            normal: V3F::new(0.0, 0.0, 1.0),
                            color: V3F::new(255.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0),
                        });
                    }
                }
            }
        }

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
    pub fn draw(&mut self, position: &V3F) {
        if !self.needs_draw {
            return;
        }
        self.draw_geometry(position);
        self.copy_to_geometry_buffer();
        self.needs_draw = false;
    }
}
