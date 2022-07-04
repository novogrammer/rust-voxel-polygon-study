mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, rust-voxel-polygon-study-wasm!");
// }

const CELL_SIZE_WIDTH:f32=1.0;
const CELL_SIZE_HEIGHT:f32=1.0;
const CELL_SIZE_DEPTH:f32=1.0;

const CHUNK_RESOLUTION_WIDTH:usize=32;
const CHUNK_RESOLUTION_HEIGHT:usize=32;
const CHUNK_RESOLUTION_DEPTH:usize=32;
const CELL_LIST_LENGTH:usize=CHUNK_RESOLUTION_WIDTH*CHUNK_RESOLUTION_HEIGHT*CHUNK_RESOLUTION_DEPTH;

const CHUNK_SIZE_WIDTH:f32=CELL_SIZE_WIDTH * CHUNK_RESOLUTION_WIDTH as f32;
const CHUNK_SIZE_HEIGHT:f32=CELL_SIZE_HEIGHT * CHUNK_RESOLUTION_HEIGHT as f32;
const CHUNK_SIZE_DEPTH:f32=CELL_SIZE_DEPTH * CHUNK_RESOLUTION_DEPTH as f32;

const UNIVERSE_RESOLUTION_WIDTH:usize=2;
const UNIVERSE_RESOLUTION_HEIGHT:usize=2;
const UNIVERSE_RESOLUTION_DEPTH:usize=2;
const CHUNK_LIST_LENGTH:usize=UNIVERSE_RESOLUTION_WIDTH*UNIVERSE_RESOLUTION_HEIGHT*UNIVERSE_RESOLUTION_DEPTH;

const UNIVERSE_SIZE_WIDTH:f32=CHUNK_SIZE_WIDTH * UNIVERSE_RESOLUTION_WIDTH as f32;
const UNIVERSE_SIZE_HEIGHT:f32=CHUNK_SIZE_HEIGHT * UNIVERSE_RESOLUTION_HEIGHT as f32;
const UNIVERSE_SIZE_DEPTH:f32=CHUNK_SIZE_DEPTH * UNIVERSE_RESOLUTION_DEPTH as f32;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Air = 0,
    Rock = 1,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct V3F{
    x:f32,
    y:f32,
    z:f32,
}
#[wasm_bindgen]
impl V3F {
    pub fn new(x:f32,y:f32,z:f32)->V3F{
        V3F {
            x,
            y,
            z,
        }
    }
    pub fn get_x(&self)->f32{
        self.x
    }
    pub fn get_y(&self)->f32{
        self.y
    }
    pub fn get_z(&self)->f32{
        self.z
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V3I{
    x:i32,
    y:i32,
    z:i32,
}
#[wasm_bindgen]
impl V3I {
    pub fn new(x:i32,y:i32,z:i32)->V3I{
        V3I {
            x,
            y,
            z,
        }
    }
    pub fn get_x(&self)->i32{
        self.x
    }
    pub fn get_y(&self)->i32{
        self.y
    }
    pub fn get_z(&self)->i32{
        self.z
    }
}


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color{
    r:u8,
    g:u8,
    b:u8,
}
#[wasm_bindgen]
impl Color {
    pub fn new(r:u8,g:u8,b:u8)->Color{
        Color {
            r,
            g,
            b,
        }
    }
    pub fn get_r(&self)->u8{
        self.r
    }
    pub fn get_g(&self)->u8{
        self.g
    }
    pub fn get_b(&self)->u8{
        self.b
    }
}



#[wasm_bindgen]
pub struct Vertex {
    position:V3F,
    normal:V3F,
    color:Color,
}

#[wasm_bindgen]
pub struct Geometry {
    vertex_list:Vec<Vertex>,
}
#[wasm_bindgen]
pub struct GeometryBuffer {
    position_list:Vec<V3F>,
    normal_list:Vec<V3F>,
    color_list:Vec<Color>,
}


pub struct Chunk {
    parent:*mut Universe,
    origin:V3F,
    // size:V3F,
    // cell_resolution:V3I,
    cell_list:Vec<Cell>,
    geometry:Geometry,
    geometry_buffer:GeometryBuffer,
    needs_update:bool,
}

// error #[wasm_bindgen] generic impls aren't supported
// #[wasm_bindgen]
impl Chunk {
}

impl Chunk {
    pub fn new(parent:*mut Universe,origin:V3F) -> Chunk{
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
        let mut cell_list=vec!{};

        cell_list.resize(CELL_LIST_LENGTH,Cell::Rock);

        let geometry=Geometry {
            vertex_list:vec!{},
        };
        let geometry_buffer=GeometryBuffer {
            position_list:vec!{},
            normal_list:vec!{},
            color_list:vec!{},
        };
        Chunk { 
            parent,
            origin: origin,
            // size,
            // cell_resolution,
            cell_list,
            geometry,
            geometry_buffer,
            needs_update:false,
        }
    }
    pub fn update(&mut self){
        // DO NOTHING
        // unsafe{
        //     let parent=self.parent.as_mut().unwrap();
        // }
    }
    pub fn draw(&mut self,_position:&V3F){
        let mut vertex_list=vec!{};
        {
            vertex_list.push(Vertex{
                position:V3F {
                    x: -0.5,
                    y: -1.0,
                    z: 0.0,
                },
                normal:V3F {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                color:Color {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            });
            vertex_list.push(Vertex{
                position:V3F {
                    x: 0.5,
                    y: -1.0,
                    z: 0.0,
                },
                normal:V3F {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                color:Color {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            });
            vertex_list.push(Vertex{
                position:V3F {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                normal:V3F {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                color:Color {
                    r: 255,
                    g: 0,
                    b: 255,
                },
            });
        }
        self.geometry.vertex_list=vertex_list;
        let mut position_list=vec!{};
        let mut normal_list=vec!{};
        let mut color_list=vec!{};
        for vertex in &self.geometry.vertex_list{
            position_list.push(vertex.position.clone());
            normal_list.push(vertex.normal.clone());
            color_list.push(vertex.color.clone());
        }
        self.geometry_buffer.position_list=position_list;
        self.geometry_buffer.normal_list=normal_list;
        self.geometry_buffer.color_list=color_list;
    }
}



#[wasm_bindgen]
pub struct Universe {
    position:V3F,
    size:V3F,
    chunk_resolution:V3I,
    chunk_list:Vec<Chunk>,
}


#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe{
        utils::set_panic_hook();

        let position=V3F{
            x:UNIVERSE_SIZE_WIDTH * -0.5 ,
            y:UNIVERSE_SIZE_HEIGHT * -0.5,
            z:UNIVERSE_SIZE_DEPTH * -0.5,
        };
        let size=V3F{
            x:UNIVERSE_SIZE_WIDTH,
            y:UNIVERSE_SIZE_HEIGHT,
            z:UNIVERSE_SIZE_DEPTH,
        };
        let chunk_resolution=V3I{
            x:UNIVERSE_RESOLUTION_WIDTH as i32,
            y:UNIVERSE_RESOLUTION_HEIGHT as i32,
            z:UNIVERSE_RESOLUTION_DEPTH as i32,
        };
        let mut universe=Universe {
            position,
            size,
            chunk_resolution,
            chunk_list:vec!{},
        };

        let mut chunk_list=vec!{};
        for iz in 0..UNIVERSE_RESOLUTION_DEPTH{
            let z=CHUNK_SIZE_DEPTH * iz as f32 + UNIVERSE_SIZE_DEPTH * -0.5;
            for iy in 0..UNIVERSE_RESOLUTION_HEIGHT{
                let y=CHUNK_SIZE_HEIGHT * iy as f32 + UNIVERSE_SIZE_HEIGHT * -0.5;
                for ix in 0..UNIVERSE_RESOLUTION_WIDTH{
                    let x=CHUNK_SIZE_WIDTH * ix as f32 + UNIVERSE_SIZE_WIDTH * -0.5;
                    let chunk = Chunk::new(&mut universe,V3F{
                        x,
                        y,
                        z,
                    });
                    chunk_list.push(chunk);
                }
            }
        }
        universe.chunk_list=chunk_list;
        universe
    }
    pub fn update(&mut self){
        for chunk in self.chunk_list.iter_mut(){
            chunk.update();
        }
    }
    pub fn draw(&mut self,position:&V3F){
        for chunk in self.chunk_list.iter_mut(){
            chunk.draw(&position);
        }
    }

    pub fn get_geometry_buffer_position_list_ptr(&self,i:usize)-> *const V3F{
        self.get_chunk(i).geometry_buffer.position_list.as_ptr()
    }
    pub fn get_geometry_buffer_normal_list_ptr(&self,i:usize)-> *const V3F{
        self.get_chunk(i).geometry_buffer.normal_list.as_ptr()
    }
    pub fn get_geometry_buffer_color_list_ptr(&self,i:usize)-> *const Color{
        self.get_chunk(i).geometry_buffer.color_list.as_ptr()
    }
    pub fn get_chunk_list_length(&self)->usize{
        CHUNK_LIST_LENGTH
    }
    pub fn get_geometry_buffer_vertex_length(&self,i:usize)->usize{
        self.get_chunk(i).geometry_buffer.position_list.len()
    }
    pub fn get_chunk_origin(&self,i:usize)->V3F{
        self.get_chunk(i).origin.clone()
    }
}

impl Universe {
    pub fn get_chunk(&self,i:usize) -> &Chunk{
        self.chunk_list.get(i).unwrap()
    }
}
