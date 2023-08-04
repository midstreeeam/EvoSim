use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    blob::geno_blob_builder::{BlobGeno, GenoBlobBuilder},
    brain::resource::BevyBlockNeurons,
    consts::{
        BLOB_SPAWN_POINT_RADIUS, PANIC_TRY_TIMES, POPULATION, SCATTER_RATIO, WORLD_HEIGHT,
        WORLD_WIDTH,
    }, logger_info,
};

use super::update::{block_action, update_blob_info, update_joint_info};

pub struct BlobContorlPlugin;

impl Plugin for BlobContorlPlugin {
    #[cfg(feature = "demo")]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, demo_setup)
            .add_systems(Update, (block_action, update_blob_info, update_joint_info));
    }

    #[cfg(feature = "move")]
    fn build(&self, app: &mut App) {
        use super::resource::TrainMutPipe;
        use crate::{
            contorl::{resource::Frames, train_move::{train_move, log_train_move}, update::update_iteration_frames},
            mutate::mutate::mutate_and_refresh_after_train,
        };

        app.add_systems(Startup, move_setup)
            .add_systems(
                Update,
                (
                    update_iteration_frames,
                    block_action,
                    update_blob_info,
                    update_joint_info,
                    log_train_move.before(train_move),
                    train_move.after(block_action),
                    mutate_and_refresh_after_train.after(train_move),
                ),
            )
            .init_resource::<TrainMutPipe>()
            .init_resource::<Frames>();
    }

    fn finish(&self, _app: &mut App) {
        logger_info!("BlobContorlPlugin started");
    }
}

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

pub fn move_setup(commands: Commands, mut bbns: ResMut<BevyBlockNeurons>) {
    let mut builder = GenoBlobBuilder::from_commands(commands, &mut bbns.nnvec);

    let centers = get_center();
    for center in centers.iter() {
        builder.build(&mut BlobGeno::new_rand(), [center.0, center.1]);
    }
}

/// generate blob center pos base on target population
pub fn get_center() -> Vec<(f32, f32)> {
    let mut rng: ThreadRng = thread_rng();

    let x_lim: (f32, f32) = (
        -WORLD_WIDTH as f32 * SCATTER_RATIO * 0.5,
        WORLD_WIDTH as f32 * SCATTER_RATIO * 0.5,
    );
    let y_lim: (f32, f32) = (
        -WORLD_HEIGHT as f32 * SCATTER_RATIO * 0.5,
        WORLD_HEIGHT as f32 * SCATTER_RATIO * 0.5,
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

fn euclidean_distance((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
