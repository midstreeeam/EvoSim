use std::f32::consts::PI;

use rand::prelude::*;

use crate::{
    blob::geno_blob_builder::{BlobGeno, GenericGenoNode, GenoNode},
    consts::{*, mutate_consts::*},
};

const CLAMP: [f32;2] = MUTATE_SINGLE_BLOCK_SIZE_CLAMP_SCALER;

/// loop over all blobs to mutate geno.
/// mutate tree-structure, block-size, joint-limit in the order
/// 
/// After the mutation, the genos and the NN is unmatched, 
/// will be rematched in function `sync_mutate`
pub fn mutate_geno(
    geno_q: &mut Vec<BlobGeno>
) {
    for mut geno in geno_q {
        mutate_tree_structure(&mut geno);
        mutate_block_size(&mut geno);
        mutate_joint_limit(&mut geno)
    }
}

/// gain or lose limbs for a blob
/// 
/// gain limb might cause self confilt.
/// set `MUTATE_GAIN_LIMB_MAX_TRY` to try if gain limb process is unsuccessful.
pub fn mutate_tree_structure(geno: &mut BlobGeno) {
    let mut rng: ThreadRng = thread_rng();

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

        for _ in 0..MUTATE_GAIN_LIMB_MAX_TRY {
            if let Some(idx) = candidates.iter().choose(&mut rand::thread_rng()) {
                // loop till get validate limb
                if gain_limb(geno, *idx) {
                    break;
                }
            }
        }
    } else {
        // TODO: it is better not lose parent indicator, which might cause self-confilt if a node
        // without parent indicator gain four limbs
        //
        // lose limb
        let candidates = geno.leaf_nodes();
        if candidates.len() <= 1 {
            // the only leaf is root, which cannot lose limb
            // or the root only have one limb left
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
    let slots: Vec<(usize, usize)> = geno
        .vec_tree
        .children(idx)
        .iter()
        .enumerate()
        .filter(|&(_, &child_idx)| geno.vec_tree.nodes[child_idx].is_none())
        .map(|(index, &child_idx)| (index, child_idx))
        .collect();

    if slots.is_empty() {
        return false;
    }
    let choosen = *slots.iter().choose(&mut rand::thread_rng()).unwrap();
    if let Some(Some(GenericGenoNode::Child(parent))) = geno.vec_tree.nodes.get(idx) {
        // TODO: new nodes should also have parent indicator
        geno.vec_tree.nodes[choosen.1] = Some(new_rand_node(parent, choosen.0));
        if geno.is_valid() {
            return true;
        } else {
            geno.vec_tree.nodes[choosen.1] = None;
            return false;
        }
    } else {
        false
    }
}

/// generate a new random `GenericGenoNode`，
/// 
/// used to generate new limbs to blob
/// 
/// Need to know the direction of the node to generate to prevent self confilt
/// and to calculate the presice position of the new block.
fn new_rand_node(parent: &GenoNode, direction: usize) -> GenericGenoNode {
    let mut rng = thread_rng();

    let parent_size = parent.size;
    let parent_center = parent.center;

    let joint_limits = [rng.gen_range(-PI * 0.9..0.0), rng.gen_range(0.0..PI * 0.9)];

    // // set limitation
    // // limitation can only avoid block conflict
    // // it can not avoid conflict caused by tree structure
    // let dx_dy_limits_top_bottom = [parent_size[0], DEFAULT_BLOCK_SIZE[0] * RAND_SIZE_SCALER[1]];
    // let dx_dy_limits_left_right = [DEFAULT_BLOCK_SIZE[0] * RAND_SIZE_SCALER[1], parent_size[1]];

    // let mut size = [
    //     rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[0]..dx_dy_limits_top_bottom[0]),
    //     rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[1]..dx_dy_limits_top_bottom[1]),
    // ];
    // if direction == 2 || direction == 3 {
    //     size = [
    //         rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[0]..dx_dy_limits_left_right[0]),
    //         rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[1]..dx_dy_limits_left_right[1]),
    //     ];
    // }

    // no limitation implementation
    let size = [
        rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[0]..RAND_SIZE_SCALER[1] * DEFAULT_BLOCK_SIZE[0]),
        rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[1]..RAND_SIZE_SCALER[1] * DEFAULT_BLOCK_SIZE[1]),
    ];

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
        nn_id: None,
    });
}

/// the blob lose a block at index `idx`, and all its subnodes.
/// (but in the implementation, dropped candidates can only be leaf nodes)
/// 
/// drop the indexed node
/// after a node is dropped, 
/// the bounded nn will also be removed in function `sync_mutate`
fn lose_limb(geno: &mut BlobGeno, idx: usize) {
    geno.vec_tree.clean_subtree(idx);
    // geno.vec_tree.nodes[idx] = None;
}

/// mutate size of blocks for a blob
/// 
/// all blocks of the blob can be mutate (but not must be mutate)
/// 
/// the mutation must valid, which means this function won't cause self confilt
pub fn mutate_block_size(geno: &mut BlobGeno) {
    let mut rng = thread_rng();

    if !rng.gen_bool(MUTATE_BLOCK_SIZE_PROB as f64) {
        return;
    }

    let mut potential_mutations: Vec<(usize, [f32; 2])> = Vec::new();

    for (index, i) in geno.vec_tree.nodes.iter().enumerate() {
        if let Some(GenericGenoNode::Child(node)) = i {
            if !rng.gen_bool(MUTATE_SINGLE_BLOCK_SIZE_PROB as f64) {
                continue;
            }
            let mutation_factor_0 = rng.gen_range(0.9..=1.1);
            let mutation_factor_1 = rng.gen_range(0.9..=1.1);
            let new_size_0 = (node.size[0] * mutation_factor_0).clamp(DEFAULT_BLOCK_SIZE[0]*CLAMP[0], DEFAULT_BLOCK_SIZE[0]*CLAMP[1]);
            let new_size_1 = (node.size[1] * mutation_factor_1).clamp(DEFAULT_BLOCK_SIZE[1]*CLAMP[0], DEFAULT_BLOCK_SIZE[1]*CLAMP[1]);
    
            // Store the mutation
            potential_mutations.push((index, [new_size_0, new_size_1]));
        }
    }
    
    for (index, new_size) in &potential_mutations {
        mutate_single_block_size(geno, *index, *new_size);      
    }
}


/// mutate the size of a single block to the new size provided
/// 
/// Notice: if a block has its size changed, all its subnode will have their position changed,
/// which can casue self-confilt to happen.
/// 
/// if the mutation (new size) is not valid (cause self confilt), 
/// then will not apply
fn mutate_single_block_size(
    geno: &mut BlobGeno,
    index: usize,
    new_size: [f32;2]
) {
    let temp_geno = geno.clone();

    if let Some(Some(GenericGenoNode::Child(node))) = temp_geno.vec_tree.nodes.get(index) {
        // TODO: modify the center of self and all children nodes if size changed
        geno.change_node_size(index, new_size);

        if let Some(move_vec) = get_movevec(index, node.size, new_size) {
            // if muated block is not root
            // println!("{:?}", move_vec);
            geno.move_subtree_nodes(index, move_vec);
        } else {
            // if mutate block is root
            geno.move_subtree_nodes_root(node.size, new_size)
        }
    } else {
        // mutate block index must be valid
        panic!()
    }
    // validation check
    if !geno.is_valid() {
        *geno = temp_geno;
    }
}

/// if a block has its size changed, all its subnode will have their position changed
/// 
/// Base on the changes of a node, return how much and which direction all its subnodes will move.
/// 
/// outputs front, left, right movement vector for input node (facing outward to the root node)
fn get_movevec(
    index: usize,
    old_size: [f32;2],
    new_size: [f32;2]
) -> Option<([f32;2],[f32;2],[f32;2])> {
    if index == 0 {
        return None
    } else {
        let top = [0.0, new_size[1]-old_size[1]];
        let bottom = [0.0, old_size[1]-new_size[1]];
        let left = [old_size[0]-new_size[0], 0.0];
        let right = [new_size[0]-old_size[0], 0.0];

        match (index - 1) % 4 {
            0 => Some((
                top,
                left,
                right
            )),
            1 => Some((
                bottom,
                right,
                left
            )),
            2 => Some((
                left,bottom,top
            )),
            3 => Some((
                right,top,bottom
            )),
            _ => {panic!()}
        }
    }
    
}

/// Mutate joint limit of limbs
pub fn mutate_joint_limit(geno: &mut BlobGeno){
    let mut rng: ThreadRng = thread_rng();

    for i in geno.vec_tree.nodes.iter_mut(){
        if !rng.gen_bool(MUTATE_JOINT_LIMIT_PROB as f64) {
            continue;
        }
        if let Some(GenericGenoNode::Child(node)) = i {
            let mutation_factor_0 = rng.gen_range(0.9..=1.1);
            let mutation_factor_1 = rng.gen_range(0.9..=1.1);
            let new_limit_0 = (node.joint_limits[0] * mutation_factor_0).clamp(MUTATE_JOINT_LIMIT_MIN, 0.0);
            let new_limit_1 = (node.joint_limits[1] * mutation_factor_1).clamp(0.0, MUTATE_JOINT_LIMIT_MAX);
            node.joint_limits = [new_limit_0,new_limit_1];
        }
    }
}