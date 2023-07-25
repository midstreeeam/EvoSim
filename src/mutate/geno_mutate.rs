use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{blob::geno_blob_builder::{BlobGeno, GenoNode, GenericGenoNode}, consts::{MUTATE_TREE_STRUCTURE_PROB, MUTATE_GAIN_LIMB_PROB, DEFAULT_BLOCK_SIZE, RAND_SIZE_SCALER}};

pub fn mutate_geno (
    geno_q: Query<&mut BlobGeno>
) {

}

/// gain or lose limbs
pub fn mutate_tree_structure(geno: &mut BlobGeno) {
    let mut rng = thread_rng();
    
    if !rng.gen_bool(MUTATE_TREE_STRUCTURE_PROB as f64) {
        return;
    }

    if rng.gen_bool(MUTATE_GAIN_LIMB_PROB as f64) {
        // gain limb
        let mut candidates = geno.vec_tree.branch_nodes();
        if candidates.is_empty() {
            // the root is leaf, add it to candidates
            candidates.push(0);
        }
        if let Some(idx) = candidates.iter().choose(&mut rand::thread_rng()) {
            gain_limb(geno, *idx);
        }
    } else {
        // TODO: it is better not lose parent indicator, which might cause self-confilt if a node
        // without parent indicator gain four limbs
        //
        // lose limb
        let candidates = geno.vec_tree.leaf_nodes();
        if candidates.is_empty() {
            // the only leaf is root, which cannot lose limb
            return;
        }
        if let Some(idx) = candidates.iter().choose(&mut rand::thread_rng()) {
            lose_limb(geno, *idx);
        }
    }
}


/// gain a new limb as the child of the index node
/// return type means success or fail
fn gain_limb(geno: &mut BlobGeno, idx: usize) -> bool {
    // direction and index of node
    // slots are nodes has `none` as value
    let slots: Vec<(usize,usize)> = geno.vec_tree.children(idx)
        .iter()
        .enumerate()
        .filter(|&(_,&child_idx)| geno.vec_tree.nodes[child_idx].is_none())
        .map(|(index, &child_idx)| (index, child_idx))
        .collect();

    if slots.is_empty() {
        return false
    }
    let choosen = *slots.iter().choose(&mut rand::thread_rng()).unwrap();
    if let Some(Some(GenericGenoNode::Child(parent))) = geno.vec_tree.nodes.get(idx) {
        // TODO: new nodes should also have parent indicator
        geno.vec_tree.nodes[choosen.1] = Some(new_rand_node(parent,choosen.0));
        return geno.is_valid()
    } else {
        false
    }
}

fn new_rand_node(
    parent: &GenoNode,
    direction: usize
) -> GenericGenoNode{
    let mut rng = thread_rng();

    let parent_size = parent.size;
    let parent_center = parent.center;

    // set limitation
    // limitation can only avoid block conflict
    // it can not avoid conflict caused by tree structure
    let dx_dy_limits_top_bottom =
        [parent_size[0], DEFAULT_BLOCK_SIZE[0] * RAND_SIZE_SCALER[1]];
    let dx_dy_limits_left_right =
        [DEFAULT_BLOCK_SIZE[0] * RAND_SIZE_SCALER[1], parent_size[1]];

    let joint_limits = [rng.gen_range(-PI * 0.9..0.0), rng.gen_range(0.0..PI * 0.9)];
    let mut size = [
        rng.gen_range(
            RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[0]..dx_dy_limits_top_bottom[0],
        ),
        rng.gen_range(
            RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[1]..dx_dy_limits_top_bottom[1],
        ),
    ];
    if direction == 2 || direction == 3 {
        size = [
            rng.gen_range(
                RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[0]..dx_dy_limits_left_right[0],
            ),
            rng.gen_range(
                RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[1]..dx_dy_limits_left_right[1],
            ),
        ];
    }

    // center
    let mut center = [
        parent_center[0],
        parent_center[1] + parent_size[1] + size[1],
    ];
    if direction == 1 {
        center = [
            parent_center[0],
            parent_center[1] - parent_size[1] - size[1],
        ];
    } else if direction == 2 {
        center = [
            parent_center[0] - parent_size[0] - size[0],
            parent_center[1],
        ];
    } else if direction == 3 {
        center = [
            parent_center[0] + parent_size[0] + size[0],
            parent_center[1],
        ]
    }

    return GenericGenoNode::Child(GenoNode {
        joint_limits,
        size,
        center,
    });
}

/// drop the indexed node
fn lose_limb(geno: &mut BlobGeno, idx:usize) {
    geno.vec_tree.nodes[idx] = None;
}

pub fn mutate_block_size (geno: &BlobGeno) {

}