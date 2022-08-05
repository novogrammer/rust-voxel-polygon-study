use crate::block::Block;

pub fn terrain_updater_a(global_position: &glam::Vec3, time: f64) -> Block {
    let mut next_cell = Block::Air;
    if global_position.length() < (32.0 * (time.sin() * 0.5 + 0.5)) as f32 {
        next_cell = Block::Rock;
    }
    if (*global_position + glam::vec3(10.0, 0.0, 0.0)).length()
        < (32.0 * (time.sin() * 0.5 + 0.5)) as f32
    {
        next_cell = Block::Rock;
    }
    next_cell
}
