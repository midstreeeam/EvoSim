use std::collections::HashSet;

use bevy::prelude::*;
use rand::prelude::*;
use rand_distr::WeightedIndex;

use crate::{
    blob::{blob::BlobInfo, block::NeuronId, geno_blob_builder::BlobGeno},
    brain::{neuron::GenericNN, resource::BevyBlockNeurons},
    consts::{ITERATION_LENGTH, NEW_ITERATION_KEYCODE, POPULATION, TRAIN_MOVE_SURVIVAL_RATE, HYBRID_RATE},
    contorl::contorl::get_center,
    logger_info,
};

use super::resource::{Frames, TrainMutPipe, TED};

/// `POPULATION == 1` in will make thread panic since it never trains
pub fn train_move(
    entity_geno_info_q: Query<(Entity, (&BlobGeno, &BlobInfo))>,
    nn_q: Query<(&Parent, &NeuronId)>,
    mut bbn: ResMut<BevyBlockNeurons>,
    mut pipe: ResMut<TrainMutPipe>,
    input: Res<Input<KeyCode>>,
    frames: Res<Frames>,
) {
    if input.just_pressed(NEW_ITERATION_KEYCODE) || iteration_end(frames) {
        let nnvec = &mut bbn.nnvec;
        let mut blob_vec_move: Vec<(Entity, (BlobGeno, BlobInfo))> = Vec::new();
        let mut blob_vec_ted: Vec<(Entity, (BlobGeno, BlobInfo))> = Vec::new();
        for (e, (geno, info)) in entity_geno_info_q.iter() {
            blob_vec_move.push((e, (geno.clone(), info.clone())));
            blob_vec_ted.push((e, (geno.clone(), info.clone())));
        }

        blob_vec_move.sort_by(|a, b| {
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

        blob_vec_ted.sort_by(|a, b| {
            let mag_a = a.1.1.crowding_distance;
            let mag_b = b.1.1.crowding_distance;
            mag_b
                .partial_cmp(&mag_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let split_idx = (blob_vec_move.len() as f32 * TRAIN_MOVE_SURVIVAL_RATE).ceil() as usize;

        // tournament selection
        let (survivers_move, _outcasts) = blob_vec_move.split_at_mut(split_idx);
        hybrid_selection(survivers_move, &blob_vec_ted);

        let (mut new_genovec, mut infovec, mut new_nnvec) = clean_outcast(survivers_move, nn_q, nnvec);

        // reproduce
        reproduce(&mut new_genovec, &mut infovec, &mut new_nnvec);

        // println!("{:#?}",new_genovec);
        // println!("nnveclen: {:#?}",new_nnvec.len());

        pipe.push(new_genovec, infovec, new_nnvec);
    }
}

/// determine the final surviers by random select blobs from 
/// survivers won move tournament and survivers won ted tournament
/// 
/// aiming to keep diversity
/// 
/// BUG: might select a signle blob and then insert it back to survivers_move, causing duplicate
fn hybrid_selection(
    survivers_move: &mut [(Entity, (BlobGeno, BlobInfo))], 
    blob_vec_ted: &Vec<(Entity, (BlobGeno, BlobInfo))>
) {
    let mut rng: ThreadRng = thread_rng();
    let x = (HYBRID_RATE * survivers_move.len() as f32) as usize;
    let bias_factor = 4.0;

    // Generate the weighted distribution
    let weights: Vec<f64> = (0..blob_vec_ted.len())
        .map(|i| ((blob_vec_ted.len() - i) as f64).powf(bias_factor))
        .collect();
    
    let mut chosen_indices = HashSet::new();
    let survivers_entities: HashSet<_> = survivers_move.iter().map(|(entity, _)| *entity).collect();

    for _ in 0..x {
        let rand_surviver_idx = rng.gen_range(0..survivers_move.len());
        
        let mut blobvec_idx = WeightedIndex::new(&weights).unwrap().sample(&mut rng);

        // Ensure the selected index is unique and its Entity is not already in survivers_move
        while chosen_indices.contains(&blobvec_idx) || survivers_entities.contains(&blob_vec_ted[blobvec_idx].0) {
            blobvec_idx = WeightedIndex::new(&weights).unwrap().sample(&mut rng);
        }

        chosen_indices.insert(blobvec_idx);
        survivers_move[rand_surviver_idx] = blob_vec_ted[blobvec_idx].clone();
    }
}

/// delete neuron from nnvec based on outcasts
fn clean_outcast(
    survivers: &mut [(Entity, (BlobGeno, BlobInfo))],
    nn_q: Query<(&Parent, &NeuronId)>,
    nnvec: &mut Vec<GenericNN>,
) -> (Vec<BlobGeno>, Vec<BlobInfo>, Vec<GenericNN>) {
    let mut new_geno_vec = Vec::<BlobGeno>::new();
    let mut infovec = Vec::<BlobInfo>::new();

    let mut existed_nn_ids = Vec::<usize>::new();

    for (blob_id, _) in survivers.iter() {
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
        missing_ids.extend(0..first_id); // checks for missing IDs before the smallest ID.
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
    for (_, (geno, _)) in survivers.iter_mut() {
        for option_id in geno.all_nn_ids_mut() {
            let copied_id = option_id.unwrap();
            // Count how many missing_ids are smaller than copied_id
            let count_smaller_gaps = missing_ids.iter().filter(|&&id| id < copied_id).count();
            *option_id = Some(copied_id - count_smaller_gaps);
        }
    }

    for (_, (geno, info)) in survivers.iter() {
        new_geno_vec.push(geno.clone());
        infovec.push(info.clone());
    }

    (new_geno_vec, infovec, nnvec.clone())
}

/// reproduce the blob to the target population
///
/// this function will reset spawn position of all blobs,
/// the position won't inherit
///
/// new NN will be append to nnvec
fn reproduce(genovec: &mut Vec<BlobGeno>, infovec: &mut Vec<BlobInfo>, nnvec: &mut Vec<GenericNN>) {
    assert_eq!(genovec.len(), infovec.len());
    assert!(genovec.len() < POPULATION);

    let mut rng: ThreadRng = thread_rng();

    let mut new_genovec: Vec<BlobGeno> = Vec::new();
    let mut new_infovec: Vec<BlobInfo> = Vec::new();
    let mut new_nnvec: Vec<GenericNN> = Vec::new();

    loop {
        let chosen_idx: usize = rng.gen_range(0..genovec.len());
        let mut new_geno = genovec.get(chosen_idx).unwrap().clone();
        let new_info = infovec.get(chosen_idx).unwrap().clone();
        for nn_id in new_geno.all_nn_ids_mut() {
            let copied_id = nn_id.unwrap();
            let new_nn = nnvec.get(copied_id).unwrap().clone();
            new_nnvec.push(new_nn);
            // modify nn_id
            *nn_id = Some(new_nnvec.len() + nnvec.len() - 1)
        }
        new_genovec.push(new_geno);
        new_infovec.push(new_info);

        if new_genovec.len() + genovec.len() == POPULATION {
            break;
        }
    }

    genovec.append(&mut new_genovec);
    infovec.append(&mut new_infovec);
    nnvec.append(&mut new_nnvec);

    let rand_centers = get_center();
    assert_eq!(infovec.len(), rand_centers.len());
    for (center, info) in rand_centers.iter().zip(infovec.iter_mut()) {
        info.center_block_pos = Vec2::from_array([center.0, center.1])
    }
}

fn iteration_end(frames: Res<Frames>) -> bool {
    let cur_gen_frame_cnt = frames.0 % ITERATION_LENGTH as u128;
    if cur_gen_frame_cnt == 0 && frames.0 != 0 {
        true
    } else {
        false
    }
}

pub fn log_train_move(frames: Res<Frames>, info_q: Query<&BlobInfo>, ted: Res<TED>) {
    let cur_gen_frame_cnt = frames.0 % ITERATION_LENGTH as u128;
    if cur_gen_frame_cnt != 0 || frames.0 == 0 {
        return;
    }

    let mut infovec = Vec::from_iter(info_q.iter());

    infovec.sort_by(|a, b| {
        let a_distance_mag = a
            .move_distance
            .iter()
            .fold(0.0, |acc, &x| acc + x * x)
            .sqrt();
        let b_distance_mag = b
            .move_distance
            .iter()
            .fold(0.0, |acc, &x| acc + x * x)
            .sqrt();
        b_distance_mag.partial_cmp(&a_distance_mag).unwrap()
    });

    let top_distance = infovec[0]
        .move_distance
        .iter()
        .fold(0.0, |acc, &x| acc + x * x)
        .sqrt();

    let total_distances: f32 = infovec
        .iter()
        .map(|info| {
            info.move_distance
                .iter()
                .fold(0.0, |acc, &x| acc + x * x)
                .sqrt()
        })
        .sum();

    let mean_distance = total_distances / infovec.len() as f32;

    logger_info!(
        "iteration {}, top_distance {:.5}, mean_distance {:.5}, ted {:.5}",
        frames.0 / ITERATION_LENGTH as u128,
        top_distance,
        mean_distance,
        ted.0
    );
}
