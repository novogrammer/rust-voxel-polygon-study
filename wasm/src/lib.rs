mod utils;

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

mod block;
mod chunk;
mod geometry;
mod geometry_buffer;
mod universe;
mod v3f;
mod v3i;
mod vertex;

// #[wasm_bindgen]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// #[repr(packed)]
// pub struct Color{
//     r:u8,
//     g:u8,
//     b:u8,
// }
// #[wasm_bindgen]
// impl Color {
//     pub fn new(r:u8,g:u8,b:u8)->Color{
//         Color {
//             r,
//             g,
//             b,
//         }
//     }
//     pub fn get_r(&self)->u8{
//         self.r
//     }
//     pub fn get_g(&self)->u8{
//         self.g
//     }
//     pub fn get_b(&self)->u8{
//         self.b
//     }
// }
