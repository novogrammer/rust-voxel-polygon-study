use cgmath::{vec3, InnerSpace, Vector3};

use crate::block::Block;

pub fn terrain_updater_a(global_position: &Vector3<f32>, time: f64) -> Block {
    let mut next_cell = Block::Air;
    if global_position.magnitude() < (32.0 * (time.sin() * 0.5 + 0.5)) as f32 {
        next_cell = Block::Rock;
    }
    if (global_position + vec3::<f32>(10.0, 0.0, 0.0)).magnitude()
        < (32.0 * (time.sin() * 0.5 + 0.5)) as f32
    {
        next_cell = Block::Rock;
    }
    next_cell
}
