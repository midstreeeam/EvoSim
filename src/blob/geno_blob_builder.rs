//! implementation of blob's gene and builder that can build blob base on an given genotype

use std::f32::consts::PI;
use std::fmt::{self, Debug};

use bevy::prelude::*;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::blob::block::NeuronId;
use crate::brain::neuron::GenericNN;
use crate::consts::*;

use super::blob_builder::BlobBuilder;
use super::block::PhysiBlockBundle;

/// Generate Blob according to Genotype
/// Wrapper around BlobBuilder
pub struct GenoBlobBuilder<'a> {
    builder: BlobBuilder<'a>,
}

impl<'a> GenoBlobBuilder<'a> {
    pub fn from_commands(commands: Commands<'a, 'a>, nnvec: &'a mut Vec<GenericNN>) -> Self {
        Self {
            builder: BlobBuilder::from_commands(commands, nnvec),
        }
    }

    /// generate blob according to its genotype
    pub fn build(&mut self, geno: &mut BlobGeno, center: [f32; 2]) {

        // create first
        let builder = &mut self.builder;

        if let Some(nn_id) = geno.get_first().unwrap().nn_id {
            // if the geno is exported geno or mutated geno (with nn inside)
            let root_nn = NeuronId::new(nn_id,None);
            
            builder.create_first(
                geno.get_first()
                .unwrap()
                .to_bundle(center),
                root_nn);
            
            // start recursion
            build_node_with_nn(builder, &mut geno.vec_tree, 0, nn_id)

        } else {
            // if the geno is new rand geno (without nn inside)
            geno.assign_nn_id_to_root(
                builder.create_first(
                geno.get_first()
                    .unwrap()
                    .to_bundle(center)
                    .with_color(Color::BLUE),
                (),).unwrap()
            );

            // start recursion
            build_node(&mut self.builder, &mut geno.vec_tree, 0);
        }

        // save geno to blob
        self.builder.update_geno(geno.clone());

        // reset builder
        self.builder.clean();
    }
}

// Lambda function to use in child extraction
fn lambda(node: &mut Option<GenericGenoNode>) -> Option<&mut GenoNode> {
    node.as_mut().and_then(|node| match node {
        GenericGenoNode::Parent => None,
        GenericGenoNode::Child(child) => Some(child),
    })
}

fn build_node(
    builder: &mut BlobBuilder, 
    tree: &mut QuadTree<GenericGenoNode>, 
    index: usize, 
) {
    if let Some(Some(_)) = tree.nodes.get_mut(index) {
        let children = tree.children(index);
        // let (top_child, bottom_child, left_child, right_child) = (
        //     tree.nodes.get(children[0]).and_then(lambda),
        //     tree.nodes.get(children[1]).and_then(lambda),
        //     tree.nodes.get(children[2]).and_then(lambda),
        //     tree.nodes.get(children[3]).and_then(lambda),
        // );

        // top
        if let Some(mut node) = tree.nodes.get_mut(children[0]).and_then(lambda) {

            let nn_id = builder.add_to_top(
                node.size[0],
                node.size[1],
                None,
                Some(node.joint_limits),
                (),
            );

            // don't overwrite nn_id if it is not None
            // which means they have already had bounded NN
            if node.nn_id.is_none() {
                node.nn_id = nn_id
            }
            
            build_node(builder, tree, children[0]);
            builder.bottom();
        }

        // bottom
        if let Some(mut node) = tree.nodes.get_mut(children[1]).and_then(lambda) {
            let nn_id = builder.add_to_bottom(
                node.size[0],
                node.size[1],
                None,
                Some(node.joint_limits),
                (),
            );

            if node.nn_id.is_none() {
                node.nn_id = nn_id
            }

            build_node(builder, tree, children[1]);
            builder.top();
        }

        // left
        if let Some(node) = tree.nodes.get_mut(children[2]).and_then(lambda) {
            let nn_id = builder.add_to_left(
                node.size[0],
                node.size[1],
                None,
                Some(node.joint_limits),
                (),
            );

            if node.nn_id.is_none() {
                node.nn_id = nn_id
            }

            build_node(builder, tree, children[2]);
            builder.right();
        }

        // right
        if let Some(node) = tree.nodes.get_mut(children[3]).and_then(lambda) {
            let nn_id = builder.add_to_right(
                node.size[0],
                node.size[1],
                None,
                Some(node.joint_limits),
                (),
            );

            if node.nn_id.is_none() {
                node.nn_id = nn_id
            }

            build_node(builder, tree, children[3]);
            builder.left();
        }
    }
}


/// build node that inherit `NeuronId` from geno
fn build_node_with_nn(
    builder: &mut BlobBuilder, 
    tree: &mut QuadTree<GenericGenoNode>, 
    index: usize,
    parent_nn_id: usize
) {
    if let Some(Some(_)) = tree.nodes.get_mut(index) {
        let children = tree.children(index);

        // top
        if let Some(node) = tree.nodes.get_mut(children[0]).and_then(lambda) {

            let nn_id = node.nn_id.unwrap();
            let neuron_id = NeuronId::new(nn_id,Some(parent_nn_id));

            builder.add_to_top(
                node.size[0],
                node.size[1],
                None,
                Some(node.joint_limits),
                neuron_id,
            );
            
            build_node_with_nn(builder, tree, children[0],nn_id);
            builder.bottom();
        }

        // bottom
        if let Some(node) = tree.nodes.get_mut(children[1]).and_then(lambda) {
            let nn_id = node.nn_id.unwrap();
            let neuron_id = NeuronId::new(nn_id,Some(parent_nn_id));
            
            builder.add_to_bottom(
                node.size[0],
                node.size[1],
                None,
                Some(node.joint_limits),
                neuron_id,
            );

            build_node_with_nn(builder, tree, children[1],nn_id);
            builder.top();
        }

        // left
        if let Some(node) = tree.nodes.get_mut(children[2]).and_then(lambda) {
            let nn_id = node.nn_id.unwrap();
            let neuron_id = NeuronId::new(nn_id,Some(parent_nn_id));

            builder.add_to_left(
                node.size[0],
                node.size[1],
                None,
                Some(node.joint_limits),
                neuron_id,
            );

            build_node_with_nn(builder, tree, children[2], nn_id);
            builder.right();
        }

        // right
        if let Some(node) = tree.nodes.get_mut(children[3]).and_then(lambda) {
            let nn_id = node.nn_id.unwrap();
            let neuron_id = NeuronId::new(nn_id,Some(parent_nn_id));

            builder.add_to_right(
                node.size[0],
                node.size[1],
                None,
                Some(node.joint_limits),
                neuron_id,
            );

            build_node_with_nn(builder, tree, children[3], nn_id);
            builder.left();
        }
    }
}

/// The Geno for morphyology of the blob.
/// 
/// The Geno is a QuadTree (it can be represented as TernaryTree as well).
/// 
/// index 0,1,2,3 means up,down,left,right (one of them can be ParentIndicator)
#[derive(Debug, Component, Clone, Serialize, Deserialize)]
pub struct BlobGeno {
    pub vec_tree: QuadTree<GenericGenoNode>,
}

impl Default for BlobGeno {
    fn default() -> Self {
        Self {
            vec_tree: QuadTree::<GenericGenoNode>::new(GENO_MAX_DEPTH),
        }
    }
}

impl BlobGeno {
    // TODO: Clean the code. Ugly long function
    /// generate a random GenoType that don't have conflict limbs
    pub fn new_rand() -> BlobGeno {
        // prevent tree-structural block conflict
        let mut occupied_region = Vec::<[f32; 4]>::new();

        fn is_overlapped(
            center: [f32; 2],
            size: [f32; 2],
            occupied_region: &mut Vec<[f32; 4]>,
        ) -> bool {
            let x_min = center[0] - size[0];
            let x_max = center[0] + size[0];
            let y_min = center[1] - size[1];
            let y_max = center[1] + size[1];

            for region in occupied_region.iter() {
                let x_overlap = x_min <= region[1] && x_max >= region[0];
                let y_overlap = y_min <= region[3] && y_max >= region[2];
                if x_overlap && y_overlap {
                    occupied_region.push([x_min, x_max, y_min, y_max]);
                    return true;
                }
            }
            occupied_region.push([x_min, x_max, y_min, y_max]);
            return false;
        }

        /// function to acquire a new rand node
        fn rand_nodes(
            parent: &GenoNode,
            direction: usize,
            occupied_region: &mut Vec<[f32; 4]>,
        ) -> Option<GenericGenoNode> {
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

            if rng.gen_bool(RAND_NODE_NOT_NONE) {
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
                if is_overlapped(center, size, occupied_region) {
                    return None;
                } else {
                    return Some(GenericGenoNode::Child(GenoNode {
                        joint_limits,
                        size,
                        center,
                        nn_id: None
                    }));
                }
            };
            return None;
        }

        /// recursive function
        fn build(
            tree: &mut QuadTree<GenericGenoNode>,
            index: usize,
            occupied_region: &mut Vec<[f32; 4]>,
        ) {
            let mut rng = thread_rng();

            let children = tree.children(index);

            // index and children index should in range
            if tree.nodes.get(children[3]).is_none() {
                return;
            }

            // random init four nodes, avoid self-conflict
            if let Some(GenericGenoNode::Child(node)) = tree.nodes[index].clone() {
                for (i, &child) in children.iter().enumerate() {
                    tree.nodes[child] = rand_nodes(&node, i, occupied_region)
                }

                // one parent indicator
                let parent_idx = *children.choose(&mut rng).unwrap();
                tree.nodes[parent_idx] = Some(GenericGenoNode::Parent);

                // keep recursion
                for &i in children.iter() {
                    if i != parent_idx {
                        build(tree, i, occupied_region);
                    }
                }
            }
        }

        // init tree
        let mut bg = BlobGeno::default();
        // root node
        bg.vec_tree.nodes[0] = Some(GenericGenoNode::Child(GenoNode::default()));
        build(&mut bg.vec_tree, 0, &mut occupied_region);
        bg
    }

    /// get the first GenoNode
    pub fn get_first(&self) -> Option<&GenoNode> {
        self.vec_tree.nodes[0].as_ref().and_then(|node| match node {
            GenericGenoNode::Parent => None,
            GenericGenoNode::Child(child) => Some(child),
        })
    }

    /// checker function to check the genotype is valid or not.
    /// 
    /// Not valid means self-conflit limbs
    pub fn is_valid(&self) -> bool {

        fn is_overlapped(
            center: [f32; 2],
            size: [f32; 2],
            occupied_region: &mut Vec<[f32; 4]>,
        ) -> bool {
            let x_min = center[0] - size[0];
            let x_max = center[0] + size[0];
            let y_min = center[1] - size[1];
            let y_max = center[1] + size[1];

            // println!("{},{},{},{}",x_min,x_max,y_min,y_max);

            for region in occupied_region.iter() {
                let x_overlap = x_min < region[1] - POSITION_EPSILON && x_max - POSITION_EPSILON > region[0];
                let y_overlap = y_min < region[3] - POSITION_EPSILON && y_max - POSITION_EPSILON > region[2];
                if x_overlap && y_overlap {
                    occupied_region.push([x_min, x_max, y_min, y_max]);
                    return true;
                }
            }
            occupied_region.push([x_min, x_max, y_min, y_max]);
            return false;
        }

        /// recursively add to `occupied_region`
        fn check (
            tree: &QuadTree<GenericGenoNode>,
            mut occupied_region: &mut Vec<[f32; 4]>,
            idx: usize
        ) -> bool {
            // println!("is_valid checking {}", idx);
            // println!("occupied_region {:?}", occupied_region);
            if let Some(Some(GenericGenoNode::Child(cur))) = tree.nodes.get(idx) {
                if !is_overlapped(cur.center, cur.size, &mut occupied_region) {
                    tree.children(idx).iter().all(|&i| check(tree, occupied_region, i))
                } else {
                    // println!("not valid {}", idx);
                    false
                }
            } else {
                true
            }
        }

        let mut occupied_region: Vec<[f32; 4]> = Vec::new();
        check(&self.vec_tree, &mut occupied_region, 0)

    }


    /// all nodes don't have child, used for mutate to lose limb
    /// 
    /// can not return root, can not return parent indicator
    pub fn leaf_nodes(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for i in 1..self.vec_tree.nodes.len() {
            if let Some(GenericGenoNode::Parent) = self.vec_tree.nodes[i] {
                continue; // Skip if the node is of type GenericGenoNode::Parent
            }
            if self.vec_tree.nodes[i].is_some() && self.vec_tree.children(i).iter().all(
                |&child_idx| 
                child_idx >= self.vec_tree.nodes.len() || 
                self.vec_tree.nodes[child_idx].is_none() || 
                matches!(
                    self.vec_tree.nodes[child_idx], 
                    Some(GenericGenoNode::Parent)
                )
            ) {
                result.push(i);
            }
        }
        result
    }

    /// assign an nn_id to root (sometimes builder don't need new random geno)
    pub fn assign_nn_id_to_root(&mut self, id: usize) {
        if let Some(Some(GenericGenoNode::Child(node))) = self.vec_tree.nodes.get_mut(0) {
            if node.nn_id.is_none() {
                node.nn_id = Some(id);
            }
        } else {
            panic!()
        }
    }

    /// get reference for all nn_id, in usize rather than `Option<usize>`
    pub fn all_usize_nn_ids(&self) -> Vec<usize> {
        self.vec_tree.nodes.iter()
            .filter_map(|node_option|{
                match node_option {
                    Some(GenericGenoNode::Child(node)) => Some(node.nn_id.unwrap()),
                    _ => None
                }
            })
            .collect()
    }

    /// get mut reference for all nn_id in the geno
    pub fn all_nn_ids_mut(&mut self) -> Vec<&mut Option<usize>> {
        self.vec_tree.nodes.iter_mut()
            .filter_map(|node_option| {
                match node_option {
                    Some(GenericGenoNode::Child(child_node)) => Some(&mut child_node.nn_id),
                    _ => None,
                }
            })
            .collect()
    }

    pub fn all_nn_ids_indices(&self) -> Vec<usize> {
        self.vec_tree.nodes.iter().enumerate()
            .filter_map(|(idx, node_option)| {
                match node_option {
                    Some(GenericGenoNode::Child(_)) => Some(idx),
                    _ => None,
                }
            })
            .collect()
    }

    /// push all subtrees outside if the root block's size changed
    pub fn move_subtree_nodes_root(&mut self, old_size: [f32;2], new_size: [f32;2]) {
        let root_index: usize = 0;
        let xmove = new_size[0]-old_size[0];
        let ymove = new_size[1]-old_size[1];

        let children = self.vec_tree.children(root_index);
        for (direction, subtree_root_index) in children.iter().enumerate() {
            let subtree_indices: Vec<usize> = self.vec_tree.subtree_indices(*subtree_root_index);
            match direction {
                0 => {
                    for &i in &subtree_indices {
                        if let Some(Some(genericnode)) = self.vec_tree.nodes.get_mut(i) {
                            if let GenericGenoNode::Child(node) = genericnode{
                                node.center[1] += ymove;
                            }
                        } else {
                            panic!()
                        }
                    }
                },
                1 => {
                    for &i in &subtree_indices {
                        if let Some(Some(genericnode)) = self.vec_tree.nodes.get_mut(i) {
                            if let GenericGenoNode::Child(node) = genericnode{
                                node.center[1] -= ymove;
                            }
                        } else {
                            panic!()
                        }
                    }
                },
                2 => {
                    for &i in &subtree_indices {
                        if let Some(Some(genericnode)) = self.vec_tree.nodes.get_mut(i) {
                            if let GenericGenoNode::Child(node) = genericnode{
                                node.center[1] -= xmove;
                            }
                        } else {
                            panic!()
                        }
                    }
                },
                3 => {
                    for &i in &subtree_indices {
                        if let Some(Some(genericnode)) = self.vec_tree.nodes.get_mut(i) {
                            if let GenericGenoNode::Child(node) = genericnode{
                                node.center[1] += xmove;
                            }
                        } else {
                            panic!()
                        }
                    }
                }
                _ => {panic!()}
            }
        }
    }

    /// only used for morphyology mutation, update the `center` in each genonode, 
    /// so that the validation check can preform
    /// 
    /// this function only works for non-root block's mutation
    pub fn move_subtree_nodes(&mut self, root_index: usize, move_vec: ([f32;2],[f32;2],[f32;2])) {
        if root_index == 0 {
            panic!()
        }

        let forward = move_vec.0;
        let left_movec = move_vec.1;
        let right_movec = move_vec.2;

        let subtree_indices: Vec<usize> = self.vec_tree.subtree_indices(root_index);
    
        // move 1 units of all nodes
        for &i in &subtree_indices {
            if let Some(Some(genericnode)) = self.vec_tree.nodes.get_mut(i) {
                if let GenericGenoNode::Child(node) = genericnode{
                    node.center[0] += forward[0];
                    node.center[1] += forward[1];
                }
            } else {
                panic!()
            }
        }

        let direction = self.vec_tree.child_direction(root_index).unwrap();
        let children = self.vec_tree.children(root_index);

        // root of subtree that need to move 1 unit more
        let forward_root = children[direction];
        let subtree_indices: Vec<usize> = self.vec_tree.subtree_indices(forward_root);


        // move 1 more unit for selected nodes
        for &i in &subtree_indices {
            if let Some(Some(genericnode)) = self.vec_tree.nodes.get_mut(i) {
                if let GenericGenoNode::Child(node) = genericnode{
                    node.center[0] += forward[0];
                    node.center[1] += forward[1];
                }
            } else {
                panic!()
            }
        }

        // move left
        let left_root_index = children[get_left_right_direction(direction).0];
        let subtree_indices = self.vec_tree.subtree_indices(left_root_index);
        for &i in &subtree_indices {
            if let Some(Some(genericnode)) = self.vec_tree.nodes.get_mut(i) {
                if let GenericGenoNode::Child(node) = genericnode{
                    node.center[0] += left_movec[0];
                    node.center[1] += left_movec[1];
                }
            } else {
                panic!()
            }
        }

        // move right
        let right_root_index = children[get_left_right_direction(direction).1];
        let subtree_indices = self.vec_tree.subtree_indices(right_root_index);
        for &i in &subtree_indices {
            if let Some(Some(genericnode)) = self.vec_tree.nodes.get_mut(i) {
                if let GenericGenoNode::Child(node) = genericnode{
                    node.center[0] += right_movec[0];
                    node.center[1] += right_movec[1];
                }
            } else {
                panic!()
            }
        }

    }

    pub fn change_node_size(&mut self, index: usize, new_size: [f32;2]) {
        if let Some(Some(GenericGenoNode::Child(node))) = self.vec_tree.nodes.get_mut(index) {
            node.size = new_size;
        }
    }
    
}

/// GenericGenoNode is the Node in the BlobGeno QuadTree.
/// 
/// Representing morphyology of each block inside blob.
/// 
/// `Parent` is an indicator to show the parent direction of current node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenericGenoNode {
    /// parent indicator
    Parent,
    Child(GenoNode),
}

/// minium signle geno node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenoNode {
    pub joint_limits: [f32; 2],
    pub size: [f32; 2],
    pub center: [f32; 2],
    pub nn_id: Option<usize>,
}

impl Default for GenoNode {
    fn default() -> Self {
        Self {
            joint_limits: [-PI, PI],
            size: DEFAULT_BLOCK_SIZE,
            center: [0.0, 0.0],
            nn_id: None
        }
    }
}

impl GenoNode {
    pub fn from_nn_id(nn_id: usize) -> Self {
        Self {
            joint_limits: [-PI, PI],
            size: DEFAULT_BLOCK_SIZE,
            center: [0.0, 0.0],
            nn_id: Some(nn_id)
        }
    }
    /// generate `PhysiBlockBundle` from GenoNode
    fn to_bundle(&self, center: [f32; 2]) -> PhysiBlockBundle {
        PhysiBlockBundle::from_xy_dx_dy(center[0], center[1], self.size[0], self.size[1])
    }
}

/// QuadTree, Helper struct
#[derive(Clone, Serialize, Deserialize)]
pub struct QuadTree<T> {
    pub nodes: Vec<Option<T>>,
    pub max_depth: u32,
}

impl<T> QuadTree<T> {
    pub fn new(max_depth: u32) -> Self {
        let capacity = usize::pow(4, max_depth)+1;
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

    /// all nodes have at least one `none` child, using for mutate to gain limb
    pub fn branch_nodes(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for i in 0..self.nodes.len() {
            if self.nodes[i].is_some() 
                && self.depth(i) < self.max_depth - 1 // Ensure the node is not at the last layer
                && self.children(i).iter().any(
                    |&child_idx| 
                    child_idx >= self.nodes.len() || self.nodes[child_idx].is_none()
                ) {
                result.push(i);
            }
        }
        result
    }

    /// all the not-None indices of the subtree
    pub fn subtree_indices(&self, index: usize) -> Vec<usize> {
        let mut result = Vec::new();

        // Recursive function to collect indices
        fn collect_indices<T>(quad_tree: &QuadTree<T>, index: usize, result: &mut Vec<usize>) {
            if quad_tree.nodes.get(index).is_some() && quad_tree.nodes[index].is_some() {
                result.push(index);

                for &child_index in &quad_tree.children(index) {
                    if child_index < quad_tree.nodes.len() {
                        collect_indices(quad_tree, child_index, result);
                    }
                }
            }
        }

        collect_indices(self, index, &mut result);
        result
    }

    pub fn child_direction(&self, index: usize) -> Option<usize>{
        if index == 0 || index > self.nodes.len() || self.nodes[index].is_none(){
            None
        } else {
            Some((index - 1) % 4)
        }
    }

    pub fn is_empty(&self, index: usize) -> bool {
        index < self.nodes.len() && self.nodes[index].is_none()
    }

    /// Calculates the Tree Edit Distance (TED) between two QuadTrees.
    ///
    /// The Tree Edit Distance is a measure of the similarity between two trees, defined as the minimum
    /// cost sequence of node deletions, insertions, and substitutions that transform one tree into the other.
    /// This function uses dynamic programming to efficiently compute the TED between the current tree and another.
    ///
    /// # Parameters
    ///
    /// * `other`: A reference to the other `QuadTree` with which the edit distance is to be calculated.
    ///
    /// # Returns
    ///
    /// Returns a `usize` representing the Tree Edit Distance between the two trees.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree1 = QuadTree::new(...);
    /// let tree2 = QuadTree::new(...);
    /// let distance = tree1.tree_edit_distance(&tree2);
    /// ```
    pub fn tree_edit_distance(&self, other: &QuadTree<T>) -> usize {
        // Create a cache to store previous computed results
        let mut dp = vec![vec![None; other.nodes.len()]; self.nodes.len()];

        self._tree_edit_distance(0, 0, other, &mut dp)
    }

    fn _tree_edit_distance(
        &self, 
        i: usize, 
        j: usize, 
        other: &QuadTree<T>, 
        dp: &mut Vec<Vec<Option<usize>>>
    ) -> usize {
        if i >= self.nodes.len() && j >= other.nodes.len() {
            return 0;
        }
        if i >= self.nodes.len() {
            return 1 + other.children(j).iter().map(|&child_j| self._tree_edit_distance(i, child_j, other, dp)).sum::<usize>();
        }
        if j >= other.nodes.len() {
            return 1 + self.children(i).iter().map(|&child_i| self._tree_edit_distance(child_i, j, other, dp)).sum::<usize>();
        }
        if let Some(val) = dp[i][j] {
            return val;
        }

        let cost = if self.nodes[i].is_some() && other.nodes[j].is_some() {
            let children_i = self.children(i);
            let children_j = other.children(j);

            (0..4).map(|k| self._tree_edit_distance(children_i[k], children_j[k], other, dp)).sum::<usize>()
        } else if self.nodes[i].is_some() {
            1 + self.children(i).iter().map(|&child_i| self._tree_edit_distance(child_i, j, other, dp)).sum::<usize>()
        } else if other.nodes[j].is_some() {
            1 + other.children(j).iter().map(|&child_j| self._tree_edit_distance(i, child_j, other, dp)).sum::<usize>()
        } else {
            0
        };

        dp[i][j] = Some(cost);
        cost
    }
}

impl<T: Debug> Debug for QuadTree<T> {
    /// tree structure debug info
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node<T: Debug>(
            tree: &QuadTree<T>,
            index: usize,
            indent: &str,
            f: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
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

/// input the forward direction.
/// output the left and right direction in tuple
fn get_left_right_direction(direction:usize) -> (usize,usize) {
    match direction {
        0 => (2,3),
        1 => (3,2),
        2 => (1,0),
        3 => (0,1),
        _ => {panic!()}
    }
}


#[cfg(test)]
mod builder_validation_test {
    use super::*;

    #[test]
    fn test_geno_builder_validation() {
        for _ in 0..100 {
            let geno = BlobGeno::new_rand();
            assert!(geno.is_valid());
        }
    }
}