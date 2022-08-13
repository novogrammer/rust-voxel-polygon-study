use noise::{NoiseFn, OpenSimplex};

use crate::{block::Block, universe::UNIVERSE_SIZE_HEIGHT};

pub type UpdaterType = dyn Fn(&glam::Vec3) -> Option<Block>;
pub type ConditionType = dyn Fn(&glam::Vec3) -> bool;

pub struct TerrainUpdater {
    previous_masked_level: f32,
}

impl TerrainUpdater {
    pub fn new() -> TerrainUpdater {
        TerrainUpdater {
            previous_masked_level: UNIVERSE_SIZE_HEIGHT * -0.5,
        }
    }
    pub fn get_updater(&mut self, time: f64) -> Box<UpdaterType> {
        let previous_masked_level = self.previous_masked_level;
        let masked_level = (time as f32 * 30.0).to_radians().sin() * UNIVERSE_SIZE_HEIGHT as f32;

        let f = if previous_masked_level < masked_level {
            update_if(
                Box::new(move |global_position: &glam::Vec3| {
                    return previous_masked_level <= global_position.y()
                        && global_position.y() < masked_level;
                }),
                terrain_updater_b_maker(time),
            )
        } else {
            update_if(
                Box::new(move |global_position: &glam::Vec3| {
                    return masked_level <= global_position.y()
                        && global_position.y() < previous_masked_level;
                }),
                Box::new(|_global_position: &glam::Vec3| Some(Block::Air)),
            )
        };
        self.previous_masked_level = masked_level;
        Box::new(f)
    }
}

pub fn _terrain_updater_first(global_position: &glam::Vec3, time: f64) -> Option<Block> {
    let mut next_cell = Block::Air;
    if global_position.length() < (32.0 * (time.sin() * 0.5 + 0.5)) as f32 {
        next_cell = Block::Sand;
    }
    if (*global_position + glam::vec3(10.0, 0.0, 0.0)).length()
        < (32.0 * (time.sin() * 0.5 + 0.5)) as f32
    {
        next_cell = Block::Metal;
    }
    Some(next_cell)
}

pub fn _terrain_updater_a_maker(time: f64) -> Box<UpdaterType> {
    let f = move |global_position: &glam::Vec3| -> Option<Block> {
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
        Some(next_cell)
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
    let f = move |global_position: &glam::Vec3| -> Option<Block> {
        let mut next_cell = Block::Air;
        // Airであることが確定している座標
        if 10.0 < global_position.y() {
            return Some(Block::Air);
        }
        // Rockであることが確定している座標
        if global_position.y() < -13.0 {
            return Some(Block::Rock);
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
        Some(next_cell)
    };

    Box::new(f)
}

pub fn update_if(condition: Box<ConditionType>, updater: Box<UpdaterType>) -> Box<UpdaterType> {
    let f = move |global_position: &glam::Vec3| -> Option<Block> {
        if condition(global_position) {
            updater(global_position)
        } else {
            None
        }
    };
    Box::new(f)
}
