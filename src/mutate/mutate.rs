use bevy::prelude::*;

use crate::{
    blob::geno_blob_builder::BlobGeno,
    brain::{
        neuron::{BlockNN, GenericNN},
        resource::BevyBlockNeurons,
    },
};

pub struct MutatePlugin;

impl Plugin for MutatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_mutate);
    }
}

// TODO: test & debug this function, haven't been tested after coded
/// mutated blob may gain or lose NN, sync it with resource
fn sync_mutate(mut geno_q: Query<&mut BlobGeno>, mut bbn: ResMut<BevyBlockNeurons>) {
    let mut existed_nn_ids = Vec::<usize>::new();

    for mut geno in geno_q.iter_mut() {
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

    // the zen of rust (bushi
    // since the id 0 must exist, so we can use `window`
    let missing_ids: Vec<usize> = existed_nn_ids
        .windows(2)
        .flat_map(|window| (window[0] + 1)..window[1])
        .collect();
    
    // delete dropped NN
    let mut index = 0;
    bbn.nnvec.retain(|_| {
        let keep = !missing_ids.contains(&index);
        index += 1;
        keep
    });

    // adjust nn_id values
    for mut geno in geno_q.iter_mut() {
        for option_id in geno.all_nn_ids_mut() {
            let copied_id = option_id.unwrap();
            // Count how many missing_ids are smaller than copied_id
            let count_smaller_gaps = missing_ids.iter().filter(|&&id| id < copied_id).count();
            *option_id = Some(copied_id - count_smaller_gaps);
        }
    }
}
