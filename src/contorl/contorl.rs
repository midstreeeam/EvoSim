//! This module is the entrance of all updates and neuron netwrok's function
//! 
//! Implementation of `BlobContorlPlugin`

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    blob::geno_blob_builder::{BlobGeno, GenoBlobBuilder},
    brain::resource::BevyBlockNeurons,
    consts::*,
    contorl::{
        resource::{Frames, TED},
        train_move::{log_train_move_swim, train_move_swim},
        update::{update_crowding_distance, update_iteration_frames},
    },
    logger_info,
    mutate::mutate::mutate_and_refresh_after_train,
};

use super::{
    resource::TrainMutPipe,
    train_move::{log_train_move_walk, train_move_walk},
    update::{block_action, update_blob_info, update_joint_info},
};

/// Main entrance of the whole EvoSim system
///
/// all implementation relate to updates and blob contorl.
///
/// include
/// - world initialize
/// - blob information update (each frame)
/// - training information update (each frame)
/// - logger function call
/// - neurons' input signal collection (each frame)
/// - neurons' forward function call (each frame)
/// - neurons' output collection (each frame)
/// - blobs' behavior update base on neuron output (each frame)
/// - training function call
/// - mutation function call
///
/// some resource implementation
///
/// include
/// - `TrainMutPipe`
/// - `Frames`
/// - `TED`
///
///
/// implement all training style.
/// choose between different training mode in const
pub struct BlobContorlPlugin;

impl Plugin for BlobContorlPlugin {
    #[cfg(feature = "demo")]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, demo_setup)
            .add_systems(Update, (block_action, update_blob_info, update_joint_info));
    }

    #[cfg(feature = "move")]
    fn build(&self, app: &mut App) {
        if TRAINING_MODE == "swim" {
            // train swim
            app.add_systems(Startup, move_setup)
                .add_systems(
                    Update,
                    (
                        update_iteration_frames.before(update_blob_info),
                        block_action,
                        update_blob_info,
                        update_joint_info,
                        update_crowding_distance,
                        log_train_move_swim.after(block_action),
                        train_move_swim.after(log_train_move_swim),
                        mutate_and_refresh_after_train.after(train_move_swim),
                    ),
                )
                .init_resource::<TrainMutPipe>()
                .init_resource::<Frames>()
                .init_resource::<TED>();
        } else if TRAINING_MODE == "walk" {
            // train walk
            app.add_systems(Startup, move_setup)
                .add_systems(
                    Update,
                    (
                        update_iteration_frames.before(update_blob_info),
                        block_action,
                        update_blob_info,
                        update_joint_info,
                        update_crowding_distance,
                        log_train_move_walk.after(block_action),
                        train_move_walk.after(log_train_move_walk),
                        mutate_and_refresh_after_train.after(train_move_walk),
                    ),
                )
                .init_resource::<TrainMutPipe>()
                .init_resource::<Frames>()
                .init_resource::<TED>();
        } else {
            panic!()
        }
    }

    fn finish(&self, _app: &mut App) {
        logger_info!("BlobContorlPlugin started");
    }
}

/// inital setup for demo (mainly for mutation demo)
pub fn demo_setup(commands: Commands, mut bbns: ResMut<BevyBlockNeurons>) {
    let mut builder = GenoBlobBuilder::from_commands(commands, &mut bbns.nnvec);
    // let mut geno = BlobGeno::new_rand();
    // builder.build(&mut geno, [-500.0, 0.0]);
    // println!("{:#?}",geno);

    for i in -2..2 {
        for j in -2..2 {
            builder.build(
                &mut BlobGeno::new_rand(),
                [1000.0 * i as f32, 1000.0 * j as f32],
            );
        }
    }
}

/// inital setup for movement training
pub fn move_setup(commands: Commands, mut bbns: ResMut<BevyBlockNeurons>) {
    let mut builder = GenoBlobBuilder::from_commands(commands, &mut bbns.nnvec);

    let centers = get_center();
    for center in centers.iter() {
        builder.build(&mut BlobGeno::new_rand(), [center.0, center.1]);
    }
}

/// generate a random blob center pos base on target population
///
/// centers generation is contorled by several consts.
///
/// function will panic if it is not very likely to
/// fit all blobs into the given field
pub fn get_center() -> Vec<(f32, f32)> {
    let mut rng: ThreadRng = thread_rng();

    let mut world_width = WORLD_WIDTH_SWIM as f32;
    let mut world_height = WORLD_HEIGHT_SWIM as f32;
    if TRAINING_MODE == "walk" {
        world_width = WORLD_WIDTH_WALK as f32;
        world_height = WORLD_HEIGHT_WALK as f32;
    }

    let x_lim: (f32, f32) = (
        -world_width * SCATTER_RATIO_X * 0.5,
        world_width as f32 * SCATTER_RATIO_X * 0.5,
    );
    let y_lim: (f32, f32) = (
        -world_height as f32 * SCATTER_RATIO_Y * 0.5,
        world_height as f32 * SCATTER_RATIO_Y * 0.5,
    );
    let number: usize = POPULATION;
    let min_distance: f32 = BLOB_SPAWN_POINT_RADIUS;

    let mut points: Vec<(f32, f32)> = Vec::new();

    // Pre-calculate feasibility
    let feasi_ratio: f32 = 0.5;
    let area_of_space = (x_lim.1 - x_lim.0) * (y_lim.1 - y_lim.0);
    let area_of_circle_with_radius_min_distance = min_distance.powi(2);
    let max_points =
        (area_of_space / area_of_circle_with_radius_min_distance * feasi_ratio) as usize;

    if max_points < number {
        panic!("It is not possible to generate the required number of points with the given constraints.");
    }

    let mut outer_try = 0;
    loop {
        let mut inner_try = 0;
        let max_inner_try = 10;
        points.clear();
        while points.len() < number {
            let x: f32 = rng.gen_range(x_lim.0..x_lim.1);
            let y: f32 = rng.gen_range(y_lim.0..y_lim.1);
            if points
                .iter()
                .all(|&point| euclidean_distance(point, (x, y)) > min_distance)
            {
                points.push((x, y));
                inner_try = 0;
            } else {
                inner_try += 1;
                if inner_try >= max_inner_try {
                    break;
                }
            }
        }

        if points.len() == number {
            break;
        }

        outer_try += 1;
        if outer_try >= PANIC_TRY_TIMES {
            panic!()
        }
    }

    points
}

/// educlidean distance between two points
fn euclidean_distance((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
