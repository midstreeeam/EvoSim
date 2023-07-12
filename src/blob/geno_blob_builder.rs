use std::f32::consts::PI;
use std::fmt::{self, Debug};

use bevy::prelude::*;
use rand::prelude::*;

use crate::consts::*;

use super::blob_builder::BlobBuilder;


/// The Geno for morphyology of the blob.
/// The Geno is a QuadTree (it can be represented as TernaryTree as well).
/// index 0,1,2,3 means up,down,left,right (one of them can be ParentIndicator)
#[derive(Debug)]
pub struct BlobGeno{
    pub vec_tree: QuadTree<GenericGenoNode>,
}

impl Default for BlobGeno{
    fn default() -> Self {
        Self {
            vec_tree: QuadTree::<GenericGenoNode>::new(GENO_MAX_DEPTH)
        }
    }
}

impl BlobGeno{

    // TODO: this function is too complicate
    /// generate a random BlobGeno
    pub fn new_rand() -> BlobGeno{
        // init rng and tree
        let mut rng = thread_rng();
        let mut bg = BlobGeno::default();


        // root node can't be none
        let joint_limits = [
            rng.gen_range(-PI..0.0), 
            rng.gen_range(0.0..PI)
        ];
        let size = [
            rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[0]..RAND_SIZE_SCALER[1] * DEFAULT_BLOCK_SIZE[0]), 
            rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[1]..RAND_SIZE_SCALER[1] * DEFAULT_BLOCK_SIZE[1])
        ];
        let geno_node = GenoNode { joint_limits, size };
        bg.vec_tree.nodes[0] = Some(GenericGenoNode::Child(geno_node));


        // init the random vector over other nodes
        for node in &mut bg.vec_tree.nodes.iter_mut().skip(1){
            // if node is not empty
            if rng.gen_bool(RAND_NODE_NOT_NONE){
                let joint_limits = [
                    rng.gen_range(-PI..0.0), 
                    rng.gen_range(0.0..PI)
                ];
                let size = [
                    rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[0]..RAND_SIZE_SCALER[1] * DEFAULT_BLOCK_SIZE[0]), 
                    rng.gen_range(RAND_SIZE_SCALER[0] * DEFAULT_BLOCK_SIZE[1]..RAND_SIZE_SCALER[1] * DEFAULT_BLOCK_SIZE[1])
                ];
                let geno_node = GenoNode { joint_limits, size };
                *node = Some(GenericGenoNode::Child(geno_node));
            }
        }

        // clean the rand tree
        Self::rand_tree_clean(&mut bg,0);

        bg
    }

    /// clean the vector and add parent indicator to create a tree
    fn rand_tree_clean(bg: &mut BlobGeno, index: usize){
        let mut rng = thread_rng();

        let tree = &mut bg.vec_tree;

        // index out of bounds or leaf node 
        // (which should only contains parent indicator)
        if tree.nodes.get(index).is_none() || tree.is_leaf(index){
            return
        }
        // empty nodes
        if tree.nodes.get(index).unwrap().is_none(){
            tree.clean_subtree(index);
            return
        }
        // parent indicator
        if let GenericGenoNode::Parent = tree.nodes.get(index).unwrap().as_ref().unwrap(){
            tree.clean_subtree_without_self(index);
            return;
        }

        // add parent indicator
        let children_indices = tree.children(index);
        let parent_indicator_index = children_indices.choose(&mut rng);
        tree.nodes[*parent_indicator_index.unwrap()] = Some(GenericGenoNode::Parent);


        // Recursively clean the rest of the tree
        for &child_index in &children_indices {
            Self::rand_tree_clean(bg, child_index);
        }

    }
}


/// GenericGenoNode is the Node in the BlobGeno QuadTree.
/// Representing morphyology of each block inside blob.
#[derive(Debug)]

pub enum GenericGenoNode{
    /// parent indicator
    Parent,
    Child(GenoNode)
}

#[derive(Debug)]

pub struct GenoNode{
    joint_limits: [f32;2],
    size: [f32;2]
}

impl Default for GenoNode{
    fn default() -> Self {
        Self {
            joint_limits:[-PI,PI],
            size:DEFAULT_BLOCK_SIZE
        }
    }
}

/// Generate Blob according to Genotype
pub struct GenoBlobBuilder<'a>{
    builder: BlobBuilder<'a>,

}


/// QuadTree, Helper struct
pub struct QuadTree<T> {
    pub nodes: Vec<Option<T>>,
    pub max_depth: u32
}

impl<T> QuadTree<T> {
    pub fn new(max_depth: u32) -> Self {
        let capacity = usize::pow(4, max_depth);
        let nodes = (0..capacity).map(|_| None).collect();
        Self { max_depth, nodes }
    }
    
    pub fn parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / 4)
        }
    }

    pub fn children(&self, index: usize) -> [usize; 4] {
        let base = 4 * index;
        [base + 1, base + 2, base + 3, base + 4]
    }

    pub fn depth(&self, index: usize) -> u32 {
        (index as f64).log(4.0).floor() as u32
    }

    pub fn is_leaf(&self, index: usize) -> bool {
        let children_indices = self.children(index);
        children_indices.iter().all(|&child_index| {
            child_index >= self.nodes.len() || self.nodes[child_index].is_none()
        })
    }

    pub fn clean_subtree(&mut self, index: usize) {
        self.nodes[index] = None;
        let child_indices = self.children(index);

        // For each child, if the child exists, clean it recursively
        for &child_index in &child_indices {
            if child_index < self.nodes.len() && self.nodes[child_index].is_some() {
                self.clean_subtree(child_index);
            }
        }
    }

    pub fn clean_subtree_without_self(&mut self, index: usize) {
        let child_indices = self.children(index);

        // For each child, if the child exists, clean it recursively
        for &child_index in &child_indices {
            if child_index < self.nodes.len() && self.nodes[child_index].is_some() {
                self.clean_subtree(child_index);
            }
        }
    }
}

impl<T: Debug> Debug for QuadTree<T> {
    /// tree structure debug info
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node<T: Debug>(tree: &QuadTree<T>, index: usize, indent: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match tree.nodes.get(index) {
                None | Some(None) => Ok(()), // skip empty nodes
                Some(Some(node)) => {
                    writeln!(f, "{}- Node {}: {:?}", indent, index, node)?;
                    let children = tree.children(index);
                    for &child_index in &children {
                        print_node(tree, child_index, &format!("{}  ", indent), f)?;
                    }
                    Ok(())
                }
            }
        }

        writeln!(f, "QuadTree {{")?;
        print_node(self, 0, "  ", f)?;
        writeln!(f, "}}")
    }
}