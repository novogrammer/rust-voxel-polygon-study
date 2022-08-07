use crate::block::Block;

pub fn terrain_updater_first(global_position: &glam::Vec3, time: f64) -> Block {
    let mut next_cell = Block::Air;
    if global_position.length() < (32.0 * (time.sin() * 0.5 + 0.5)) as f32 {
        next_cell = Block::Sand;
    }
    if (*global_position + glam::vec3(10.0, 0.0, 0.0)).length()
        < (32.0 * (time.sin() * 0.5 + 0.5)) as f32
    {
        next_cell = Block::Metal;
    }
    next_cell
}

pub fn terrain_updater_a(global_position: &glam::Vec3, time: f64) -> Block {
    let mut next_cell = Block::Air;
    let ground_level = (global_position.x() * 5.0 + time as f32 * 30.0)
        .to_radians()
        .sin()
        * (global_position.z() * 5.0).to_radians().sin()
        * 9.0
        - 10.0;
    if global_position.y() < ground_level - 3.0 {
        next_cell = Block::Rock;
    } else if global_position.y() < ground_level {
        next_cell = Block::Sand;
    }
    next_cell
}
