use bevy::prelude::*;

use crate::{
    blob::geno_blob_builder::{BlobGeno, GenoBlobBuilder},
    brain::resource::BevyBlockNeurons,
};

use super::update::{block_action, update_blob_info, update_joint_info};

pub struct BlobContorlPlugin;

impl Plugin for BlobContorlPlugin {
    #[cfg(feature = "demo")]
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, demo_setup)
        .add_systems(Update, (block_action, update_blob_info, update_joint_info));
    }

    #[cfg(feature = "move")]
    fn build(&self, app: &mut App) {
        use crate::{contorl::train_move::train_move, mutate::mutate::mutate_and_refresh_after_train};
        use super::resource::TrainMutPipe;

        app
        .add_systems(Startup, move_setup)
        .add_systems(Update, (
            block_action, 
            update_blob_info, 
            update_joint_info,
            train_move.after(block_action),
            mutate_and_refresh_after_train.after(train_move)
        ))
        .init_resource::<TrainMutPipe>();

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

    for i in -3..3 {
        for j in -3..3 {
            builder.build(
                &mut BlobGeno::new_rand(),
                [1500.0 * i as f32, 1000.0 * j as f32],
            );
        }
    }
}