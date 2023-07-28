use bevy::prelude::*;
use bevy_rapier2d::prelude::ImpulseJoint;

use crate::{
    blob::{blob::{Blob, BlobInfo}, geno_blob_builder::{BlobGeno, GenoBlobBuilder}},
    brain::{
        neuron::{BlockNN, GenericNN},
        resource::BevyBlockNeurons,
    },
    componet::ColliderFlag,
    physics::world::Wall, contorl::block_action,
};

use super::{geno_mutate::mutate_geno, nn_mutate::mutate_nn};

pub struct MutatePlugin;

impl Plugin for MutatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mutate_and_refresh.after(block_action));
    }
}

/// similar implementation as `clean()` in `import.rs`
/// respawn blobs, update bbn
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
    for (geno,info) in geno_info_q.iter() {
        geno_vec.push(geno.clone());
        info_vec.push(info);
    }

    if input.just_pressed(KeyCode::R) {
        mutate_geno(&mut geno_vec);
        mutate_nn(&mut bbn);

        let (mut genovec,nnvec) = sync_mutate(&mut geno_vec, &mut bbn);
    
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

// TODO: test & debug this function, haven't been tested after coded
/// mutated blob may gain or lose NN, sync it with resource
fn sync_mutate(
    geno_q: &mut Vec<BlobGeno>, 
    bbn: &mut ResMut<BevyBlockNeurons>
) -> (Vec<BlobGeno>,Vec<GenericNN>) {
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
    (Vec::from_iter(geno_q.iter().cloned()),bbn.nnvec.clone())
}
