use noise::{NoiseFn, OpenSimplex};

use crate::block::Block;

pub type UpdaterType = dyn Fn(&glam::Vec3) -> Block;

pub fn _terrain_updater_first(global_position: &glam::Vec3, time: f64) -> Block {
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

pub fn _terrain_updater_a_maker(time: f64) -> Box<UpdaterType> {
    let f = move |global_position: &glam::Vec3| -> Block {
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
    };
    Box::new(f)
}

// pub fn terrain_updater_b(global_position: &glam::Vec3, time: f64) -> Block {
//     let mut next_cell = Block::Air;
//     let mut open_simplex = OpenSimplex::new();
//     let value = open_simplex.get([
//         global_position.x() as f64 * 0.1,
//         global_position.z() as f64 * 0.1,
//         time,
//     ]) as f32;

//     let ground_level = value * 10.0;
//     if global_position.y() < ground_level - 3.0 {
//         next_cell = Block::Rock;
//     } else if global_position.y() < ground_level {
//         next_cell = Block::Sand;
//     }
//     next_cell
// }

pub fn terrain_updater_b_maker(time: f64) -> Box<UpdaterType> {
    let noise = OpenSimplex::default();
    let f = move |global_position: &glam::Vec3| -> Block {
        let mut next_cell = Block::Air;
        // Airであることが確定している座標
        if 10.0 < global_position.y() {
            return Block::Air;
        }
        // Rockであることが確定している座標
        if global_position.y() < -13.0 {
            return Block::Rock;
        }
        let value = noise.get([
            global_position.x() as f64 * 0.1,
            global_position.z() as f64 * 0.1,
            time,
        ]) as f32;

        let ground_level = value * 10.0;
        if global_position.y() < ground_level - 3.0 {
            next_cell = Block::Rock;
        } else if global_position.y() < ground_level {
            next_cell = Block::Sand;
        }
        next_cell
    };

    Box::new(f)
}
