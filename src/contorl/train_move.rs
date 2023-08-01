use bevy::prelude::*;

use crate::{
    blob::{blob::BlobInfo, block::NeuronId, geno_blob_builder::BlobGeno},
    brain::{neuron::GenericNN, resource::BevyBlockNeurons},
    consts::TRAIN_MOVE_SURVIVAL_RATE,
};

use super::resource::TrainMutPipe;

pub fn train_move(
    entity_geno_info_q: Query<(Entity, (&BlobGeno, &BlobInfo))>,
    nn_q: Query<(&Parent, &NeuronId)>,
    mut bbn: ResMut<BevyBlockNeurons>,
    mut pipe: ResMut<TrainMutPipe>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::R) {
        let nnvec = &mut bbn.nnvec;
        let mut blob_vec: Vec<(Entity, (BlobGeno, BlobInfo))> = Vec::new();
        for (e, (geno, info)) in entity_geno_info_q.iter() {
            blob_vec.push((e, (geno.clone(), info.clone())));
        }
    
        blob_vec.sort_by(|a, b| {
            let mag_a =
                a.1 .1
                    .move_distance
                    .iter()
                    .fold(0.0, |acc, &x| acc + x * x)
                    .sqrt();
            let mag_b =
                b.1 .1
                    .move_distance
                    .iter()
                    .fold(0.0, |acc, &x| acc + x * x)
                    .sqrt();
            mag_b
                .partial_cmp(&mag_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    
        let split_idx = (blob_vec.len() as f32 * TRAIN_MOVE_SURVIVAL_RATE).ceil() as usize;
        let (survivers, _outcasts) = blob_vec.split_at_mut(split_idx);
    
        let (new_genovec, infovec, new_nnvec) = clean_outcast(survivers, nn_q, nnvec);
        
        // println!("{:#?}",new_genovec);
        // println!("nnveclen: {:#?}",new_nnvec.len());
        
        pipe.push(new_genovec, infovec, new_nnvec);
    }
}

// TODO: test and debug this function
/// delete neuron from nnvec based on outcasts
fn clean_outcast(
    surviers: &mut [(Entity, (BlobGeno, BlobInfo))],
    nn_q: Query<(&Parent, &NeuronId)>,
    nnvec: &mut Vec<GenericNN>,
) -> (Vec<BlobGeno>, Vec<BlobInfo>, Vec<GenericNN>) {
    let mut new_geno_vec = Vec::<BlobGeno>::new();
    let mut infovec = Vec::<BlobInfo>::new();

    let mut existed_nn_ids = Vec::<usize>::new();

    for (blob_id, _) in surviers.iter() {
        for (parent_id, neuron) in nn_q.iter() {
            if parent_id.get() != *blob_id {
                continue;
            }
            // unwrap since neuron must be in nnvec
            existed_nn_ids.push(neuron.id)
        }
    }

    existed_nn_ids.sort_unstable();

    let mut missing_ids = Vec::new();

    // Check gap before the first ID
    if let Some(&first_id) = existed_nn_ids.first() {
        missing_ids.extend(0..first_id); // This line checks for missing IDs before the smallest ID.
    }

    // Check gaps between every consecutive pair of IDs
    for window in existed_nn_ids.windows(2) {
        missing_ids.extend((window[0] + 1)..window[1]);
    }

    // Check gap after the last ID up to the maximum possible ID (assuming it's nnvec.len())
    if let Some(&last_id) = existed_nn_ids.last() {
        missing_ids.extend((last_id + 1)..nnvec.len());
    }

    // delete dropped NN
    let mut index = 0;
    nnvec.retain(|_| {
        let keep = !missing_ids.contains(&index);
        index += 1;
        keep
    });

    // adjust nn_id values
    for (_, (geno, _)) in surviers.iter_mut() {
        for option_id in geno.all_nn_ids_mut() {
            let copied_id = option_id.unwrap();
            // Count how many missing_ids are smaller than copied_id
            let count_smaller_gaps = missing_ids.iter().filter(|&&id| id < copied_id).count();
            *option_id = Some(copied_id - count_smaller_gaps);
        }
    }

    for (_, (geno, info)) in surviers.iter() {
        new_geno_vec.push(geno.clone());
        infovec.push(info.clone());
    }

    (new_geno_vec, infovec, nnvec.clone())
}
