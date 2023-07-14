use std::f32::consts::PI;
use std::fmt::{self, Debug};

use bevy::prelude::*;
use rand::prelude::*;

use crate::brain::neuron::GenericBlockNN;
use crate::consts::*;

use super::blob_builder::BlobBuilder;
use super::block::PhysiBlockBundle;

/// Generate Blob according to Genotype
/// Wrapper around BlobBuilder
pub struct GenoBlobBuilder<'a> {
    builder: BlobBuilder<'a>,
}

impl<'a> GenoBlobBuilder<'a> {
    pub fn from_commands(commands: Commands<'a, 'a>, nnvec: &'a mut Vec<GenericBlockNN>) -> Self {
        Self {
            builder: BlobBuilder::from_commands(commands, nnvec),
        }
    }

    /// generate blob according to its genotype
    pub fn build(&mut self, geno: &BlobGeno, center: [f32; 2]) {
        // Lambda function to use in child extraction
        fn lambda(node: &Option<GenericGenoNode>) -> Option<&GenoNode> {
            node.as_ref().and_then(|node| match node {
                GenericGenoNode::Parent => None,
                GenericGenoNode::Child(child) => Some(child),
            })
        }

        fn build_node(builder: &mut BlobBuilder, tree: &QuadTree<GenericGenoNode>, index: usize) {
            if let Some(Some(_)) = tree.nodes.get(index) {
                let children = tree.children(index);
                let (top_child, bottom_child, left_child, right_child) = (
                    tree.nodes.get(children[0]).and_then(lambda),
                    tree.nodes.get(children[1]).and_then(lambda),
                    tree.nodes.get(children[2]).and_then(lambda),
                    tree.nodes.get(children[3]).and_then(lambda),
                );

                if let Some(node) = top_child {
                    builder.add_to_top(
                        node.size[0],
                        node.size[1],
                        None,
                        Some(node.joint_limits),
                        (),
                    );
                    build_node(builder, tree, children[0]);
                    builder.bottom();
                }

                if let Some(node) = bottom_child {
                    builder.add_to_bottom(
                        node.size[0],
                        node.size[1],
                        None,
                        Some(node.joint_limits),
                        (),
                    );
                    build_node(builder, tree, children[1]);
                    builder.top();
                }

                if let Some(node) = left_child {
                    builder.add_to_left(
                        node.size[0],
                        node.size[1],
                        None,
                        Some(node.joint_limits),
                        (),
                    );
                    build_node(builder, tree, children[2]);
                    builder.right();
                }

                if let Some(node) = right_child {
                    builder.add_to_right(
                        node.size[0],
                        node.size[1],
                        None,
                        Some(node.joint_limits),
                        (),
                    );
                    build_node(builder, tree, children[3]);
                    builder.left();
                }
            }
        }

        // create first
        let builder = &mut self.builder;
        builder.create_first(
            geno.get_first()
                .unwrap()
                .to_bundle(center)
                .with_color(Color::BLUE),
            (),
        );

        // start recursion
        build_node(&mut self.builder, &geno.vec_tree, 0);

        // reset builder
        self.builder.clean();
    }
}

/// The Geno for morphyology of the blob.
/// The Geno is a QuadTree (it can be represented as TernaryTree as well).
/// index 0,1,2,3 means up,down,left,right (one of them can be ParentIndicator)
#[derive(Debug)]
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
                let joint_limits = [rng.gen_range(-PI..0.0), rng.gen_range(0.0..PI)];
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

    pub fn get_first(&self) -> Option<&GenoNode> {
        self.vec_tree.nodes[0].as_ref().and_then(|node| match node {
            GenericGenoNode::Parent => None,
            GenericGenoNode::Child(child) => Some(child),
        })
    }
}

/// GenericGenoNode is the Node in the BlobGeno QuadTree.
/// Representing morphyology of each block inside blob.
#[derive(Debug, Clone)]
pub enum GenericGenoNode {
    /// parent indicator
    Parent,
    Child(GenoNode),
}

#[derive(Debug, Clone)]
pub struct GenoNode {
    joint_limits: [f32; 2],
    size: [f32; 2],
    center: [f32; 2],
}

impl Default for GenoNode {
    fn default() -> Self {
        Self {
            joint_limits: [-PI, PI],
            size: DEFAULT_BLOCK_SIZE,
            center: [0.0, 0.0],
        }
    }
}

impl GenoNode {
    /// generate [`PhysiBlockBundle`] from GenoNode
    fn to_bundle(&self, center: [f32; 2]) -> PhysiBlockBundle {
        PhysiBlockBundle::from_xy_dx_dy(center[0], center[1], self.size[0], self.size[1])
    }
}

/// QuadTree, Helper struct
pub struct QuadTree<T> {
    pub nodes: Vec<Option<T>>,
    pub max_depth: u32,
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
