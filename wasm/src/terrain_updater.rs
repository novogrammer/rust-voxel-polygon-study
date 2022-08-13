use noise::{NoiseFn, OpenSimplex};

use crate::{block::Block, universe::UNIVERSE_SIZE_HEIGHT};

pub type UpdaterType = dyn Fn(&glam::Vec3) -> Option<Block>;
pub type ConditionType = dyn Fn(&glam::Vec3) -> bool;

const SCENE_QTY: i32 = 5;
const SCENE_DURATION: f32 = 6.0;
const INTRO_DURATION: f32 = 2.0;
const OUTRO_DURATION: f32 = 2.0;

const INTRO_BEGIN_TIME: f32 = 0.0;
const INTRO_END_TIME: f32 = INTRO_BEGIN_TIME + INTRO_DURATION;
const OUTRO_BEGIN_TIME: f32 = SCENE_DURATION - OUTRO_DURATION;
const OUTRO_END_TIME: f32 = SCENE_DURATION;

const DELTA_TIME_MAX: f32 = 1.0 / 60.0;

pub struct TerrainUpdater {
    previous_time: f64,
    previous_animation_time: f32,
    previous_scene_index: i32,
}

impl TerrainUpdater {
    pub fn new() -> TerrainUpdater {
        TerrainUpdater {
            previous_time: 0.0,
            previous_animation_time: 0.0,
            previous_scene_index: 0,
        }
    }
    pub fn get_updater(&mut self, time: f64) -> Box<UpdaterType> {
        let delta_time = ((time - self.previous_time) as f32).min(DELTA_TIME_MAX);

        let mut animation_time = self.previous_animation_time + delta_time;

        let to_base_maker = |scene_index, time_for_generate| -> Box<UpdaterType> {
            match scene_index {
                0 => terrain_updater_a_maker(time_for_generate),
                1 => terrain_updater_b_maker(time_for_generate),
                2 => terrain_updater_a_maker(time_for_generate),
                3 => terrain_updater_b_maker(time_for_generate),
                4 => terrain_updater_a_maker(time_for_generate),
                _ => terrain_updater_b_maker(time_for_generate),
            }
        };
        let to_level =
            |t: f32| -> f32 { (t.min(1.0).max(0.0) - 0.5) * UNIVERSE_SIZE_HEIGHT as f32 };

        let mut f_list = vec![];

        let mut scene_index = self.previous_scene_index;
        if self.previous_animation_time < INTRO_END_TIME {
            let previous_masked_level =
                to_level((self.previous_animation_time - INTRO_BEGIN_TIME) / INTRO_DURATION);
            let masked_level = to_level((animation_time - INTRO_BEGIN_TIME) / INTRO_DURATION);
            f_list.push(update_if_maker(
                Box::new(move |global_position: &glam::Vec3| {
                    return previous_masked_level <= global_position.y()
                        && global_position.y() < masked_level;
                }),
                to_base_maker(scene_index, time),
            ));
        }
        if self.previous_animation_time < OUTRO_END_TIME || OUTRO_BEGIN_TIME <= animation_time {
            let previous_masked_level =
                to_level(1.0 - (self.previous_animation_time - OUTRO_BEGIN_TIME) / OUTRO_DURATION);
            let masked_level = to_level(1.0 - (animation_time - OUTRO_BEGIN_TIME) / OUTRO_DURATION);
            f_list.push(update_if_maker(
                Box::new(move |global_position: &glam::Vec3| {
                    return masked_level <= global_position.y()
                        && global_position.y() < previous_masked_level;
                }),
                Box::new(terrain_updater_air),
            ));
        }
        // 次のシーンにはみ出している
        if OUTRO_END_TIME <= animation_time {
            scene_index = (scene_index + 1) % SCENE_QTY;
            animation_time = animation_time - SCENE_DURATION;

            let previous_masked_level = to_level((0.0 - INTRO_BEGIN_TIME) / INTRO_DURATION);
            let masked_level = to_level((animation_time - INTRO_BEGIN_TIME) / INTRO_DURATION);
            f_list.push(update_if_maker(
                Box::new(move |global_position: &glam::Vec3| {
                    return previous_masked_level <= global_position.y()
                        && global_position.y() < masked_level;
                }),
                to_base_maker(scene_index, time),
            ));
        }
        self.previous_animation_time = animation_time;
        self.previous_scene_index = scene_index;

        sequencial_update_maker(f_list)
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

pub fn terrain_updater_a_maker(time: f64) -> Box<UpdaterType> {
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

pub fn update_if_maker(
    condition: Box<ConditionType>,
    updater: Box<UpdaterType>,
) -> Box<UpdaterType> {
    let f = move |global_position: &glam::Vec3| -> Option<Block> {
        if condition(global_position) {
            updater(global_position)
        } else {
            None
        }
    };
    Box::new(f)
}
pub fn sequencial_update_maker(updater_list: Vec<Box<UpdaterType>>) -> Box<UpdaterType> {
    let f = move |global_position: &glam::Vec3| -> Option<Block> {
        let mut result = None;
        for updater in &updater_list {
            match updater(global_position) {
                Some(block) => {
                    result = Some(block);
                }
                _ => {
                    // DO NOTHING
                }
            }
        }
        result
    };
    Box::new(f)
}

pub fn terrain_updater_air(_global_position: &glam::Vec3) -> Option<Block> {
    Some(Block::Air)
}

pub fn terrain_updater_none(_global_position: &glam::Vec3) -> Option<Block> {
    None
}
