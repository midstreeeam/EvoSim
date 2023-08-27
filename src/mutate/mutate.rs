//! Implementation of `MutatePlugin`

use bevy::prelude::*;
use bevy_rapier2d::prelude::ImpulseJoint;

use crate::{
    blob::{
        blob::{Blob, BlobInfo},
        geno_blob_builder::{BlobGeno, GenoBlobBuilder},
    },
    brain::{
        neuron::{BlockNN, GenericNN},
        resource::BevyBlockNeurons,
    },
    componet::ColliderFlag,
    consts::MUTATE_AND_REFRESH_KEYCODE,
    contorl::{resource::TrainMutPipe, update::block_action},
    physics::world::Wall,
};

use super::{geno_mutate::mutate_geno, nn_mutate::mutate_nn};

/// all implementations relate to mutation
/// 
/// include
/// - geno (morphyology tree structrue) mutate
/// - nn mutate
/// 
/// Notice: this plugin provide mutation function, 
/// but do not provide update functions called each frame
/// 
/// Mutation process was called in `contorl.rs`
pub struct MutatePlugin;

impl Plugin for MutatePlugin {
    fn build(&self, app: &mut App) {
        // this function is not mutation in training process
        app.add_systems(Update, mutate_and_refresh.after(block_action));
    }
}

/// similar implementation as `clean()` in `import.rs`
/// respawn blobs, update bbn
/// 
/// Notice: this function only preform manually mutation after button was pressed,
/// automatical mutation in training process is fn `mutate_and_refresh_after_train`
pub fn mutate_and_refresh(
    mut commands: Commands,
    mut bbn: ResMut<BevyBlockNeurons>,
    geno_info_q: Query<(&BlobGeno, &BlobInfo)>,
    blob_q: Query<Entity, With<Blob>>,
    collider_q: Query<Entity, (With<ColliderFlag>, Without<Wall>)>,
    joint_q: Query<Entity, With<ImpulseJoint>>,
    input: Res<Input<KeyCode>>,
) {
    let mut geno_vec = Vec::<BlobGeno>::new();
    let mut info_vec = Vec::<&BlobInfo>::new();
    for (geno, info) in geno_info_q.iter() {
        geno_vec.push(geno.clone());
        info_vec.push(info);
    }

    if input.just_pressed(MUTATE_AND_REFRESH_KEYCODE) {
        mutate_geno(&mut geno_vec);
        mutate_nn(&mut bbn.nnvec);

        let (mut genovec, nnvec) = sync_mutate(&mut geno_vec, &mut bbn);

        // despawn
        for entity in blob_q.iter().chain(collider_q.iter()).chain(joint_q.iter()) {
            commands.entity(entity).despawn()
        }

        // temp empty vector for builder
        let mut temp_nnvec = Vec::<GenericNN>::new();
        let mut builder = GenoBlobBuilder::from_commands(commands, &mut temp_nnvec);

        for (geno, &info) in genovec.iter_mut().zip(info_vec.iter()) {
            builder.build(geno, info.center_block_pos.to_array())
        }

        // update nnvec
        bbn.nnvec = nnvec;
    }
}

/// mutate all blobs in both geno and nn,
/// refresh the field, update all blobs componets,
/// and update `BevyBlockNeurons` resource
pub fn mutate_and_refresh_after_train(
    mut commands: Commands,
    mut bbn: ResMut<BevyBlockNeurons>,
    mut pipe: ResMut<TrainMutPipe>,
    // geno_info_q: Query<(&BlobGeno, &BlobInfo)>,
    blob_q: Query<Entity, With<Blob>>,
    collider_q: Query<Entity, (With<ColliderFlag>, Without<Wall>)>,
    joint_q: Query<Entity, With<ImpulseJoint>>,
    // input: Res<Input<KeyCode>>,
) {
    // emtpy pipe means no tournament selection preformed in this frame
    if pipe.is_empty() {
        return;
    }

    let (mut pipe_genovec, infovec, mut pipe_nnvec) = pipe.pop();

    mutate_geno(&mut pipe_genovec);
    mutate_nn(&mut pipe_nnvec);

    bbn.nnvec = pipe_nnvec;

    let (mut genovec, nnvec) = sync_mutate(&mut pipe_genovec, &mut bbn);

    // despawn
    for entity in blob_q.iter().chain(collider_q.iter()).chain(joint_q.iter()) {
        commands.entity(entity).despawn()
    }

    // temp empty vector for builder
    let mut temp_nnvec = Vec::<GenericNN>::new();
    let mut builder = GenoBlobBuilder::from_commands(commands, &mut temp_nnvec);

    for (geno, info) in genovec.iter_mut().zip(infovec.iter()) {
        builder.build(geno, info.center_block_pos.to_array())
    }

    // update nnvec
    bbn.nnvec = nnvec;
}

/// mutated blob may gain or lose NN, sync it with resource.
/// 
/// If blob gain limbs, new NN will be append to the end of the NN vector in resource.
/// 
/// If blob lose limbs, unused NN will be delete, and all NN after that deleted NN will have its id changed
/// to make sure the nn_id always match. 
/// (NN resource do not have id since their id is index, changed id are in `BlobGeno`)
fn sync_mutate(
    geno_q: &mut Vec<BlobGeno>,
    bbn: &mut ResMut<BevyBlockNeurons>,
) -> (Vec<BlobGeno>, Vec<GenericNN>) {
    let mut existed_nn_ids = Vec::<usize>::new();

    for geno in geno_q.iter_mut() {
        // generate NN for new limbs
        for id in geno.all_nn_ids_mut() {
            if id.is_none() {
                bbn.nnvec.push(GenericNN::BLOCKNN(BlockNN::default()));
                *id = Some(bbn.nnvec.len() - 1);
            }
            existed_nn_ids.push(id.clone().unwrap());
        }
    }

    existed_nn_ids.sort_unstable();

    // // the zen of rust (bushi
    // // since the id 0 must exist, so we can use `window`
    // let missing_ids: Vec<usize> = existed_nn_ids
    //     .windows(2)
    //     .flat_map(|window| (window[0] + 1)..window[1])
    //     .collect();

    // Start with an empty list of missing IDs
    let mut missing_ids = Vec::new();

    // Check gaps between every consecutive pair of IDs
    for window in existed_nn_ids.windows(2) {
        missing_ids.extend((window[0] + 1)..window[1]);
    }

    // Check gap after the last ID up to the maximum possible ID (assuming it's bbn.nnvec.len())
    if let Some(&last_id) = existed_nn_ids.last() {
        missing_ids.extend((last_id + 1)..bbn.nnvec.len());
    }

    // delete dropped NN
    let mut index = 0;
    bbn.nnvec.retain(|_| {
        let keep = !missing_ids.contains(&index);
        index += 1;
        keep
    });

    // adjust nn_id values
    for geno in geno_q.iter_mut() {
        for option_id in geno.all_nn_ids_mut() {
            let copied_id = option_id.unwrap();
            // Count how many missing_ids are smaller than copied_id
            let count_smaller_gaps = missing_ids.iter().filter(|&&id| id < copied_id).count();
            *option_id = Some(copied_id - count_smaller_gaps);
        }
    }

    // copy geno
    (Vec::from_iter(geno_q.iter().cloned()), bbn.nnvec.clone())
}
