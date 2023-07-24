use bevy::prelude::*;

use crate::blob::geno_blob_builder::BlobGeno;

pub fn mutate_geno (
    geno_q: Query<&mut BlobGeno>
) {

}

/// gain or lose limbs
pub fn mutate_tree_structure (geno: &mut BlobGeno) {
    let candidates = geno.vec_tree.leaf_nodes();
    if candidates.is_empty() {
        // at least the root can be leaf
        panic!()
    }

    // if root is leaf node, then it can not lose limb anymore
    for idx in candidates.iter() {

    }
}

/// gain a new limb as the child of the index node
fn gain_limb(geno: &mut BlobGeno, idx: usize) {

}

/// drop the indexed node
fn lose_limb(geno: &mut BlobGeno, idx:usize) {
    
}

pub fn mutate_block_size (geno: &BlobGeno) {

}