use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    brain::neuron::{BlockNN, BrainNN, GenericNN},
    consts::*,
};

use super::{blob::*, block::*, geno_blob_builder::BlobGeno};

#[derive(Debug)]
pub struct BlobBlock {
    id: Entity,
    top: Option<usize>,
    bottom: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    vec_index: usize,
    size: Vec2,
    translation: Vec2,
    anchors: BlockAnchors,
    depth: u32,
    nn_id: usize,
}

pub struct BlobBuilder<'a> {
    // tools
    commands: Commands<'a, 'a>,
    nnvec: &'a mut Vec<GenericNN>,

    // builder info
    blob_bundle: Entity,
    pub blocks: Vec<BlobBlock>,

    // blob info
    info: BlobInfo,

    // position info
    /// current position index in block vector
    current_pos: Option<usize>,
}

// TODO: ugly long impl, can be simplified
impl<'a> BlobBuilder<'a> {
    /// BlobBuilder taks ownership of Commands,
    /// which means you can not use Commands anymore after using the BlobBuilder.
    /// To use commands, you need to preform it before creating BlobBuilder
    /// or just create another system.
    ///
    /// To generate multiple blobs, or want to use BlobBuilder in loops,
    /// please use `clean()` so that there won't be joints connects.
    pub fn from_commands(mut commands: Commands<'a, 'a>, nnvec: &'a mut Vec<GenericNN>) -> Self {
        Self {
            blob_bundle: commands.spawn(BlobBundle::default()).id(),
            commands: commands,
            nnvec: nnvec,
            blocks: Vec::new(),
            current_pos: None,
            info: BlobInfo::default(),
        }
    }

    /// set color for blob
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.info.color = color;
        self.update_info();
        self
    }

    /// Clean all the things inside BlobBuilder
    ///
    /// Equvalent to drop the old builder and generate a new one
    ///
    /// `nnvec` will be kept
    pub fn clean(&mut self) -> &mut Self {
        self.blob_bundle = self.commands.spawn(BlobBundle::default()).id();
        self.blocks = Vec::new();
        self.current_pos = None;
        self.info = BlobInfo::default();
        self
    }

    /// send geno to blob so geno can be kept
    pub fn update_geno(&mut self, geno: BlobGeno) {
        self.commands.entity(self.blob_bundle).insert(geno);
    }

    /// move one step left from the current position
    pub fn left(&mut self) -> &mut Self {
        if self.current_pos.is_some() {
            let pos = self.current_pos.unwrap();
            if self.blocks[pos].left.is_some() {
                let index = self.blocks[pos].left.unwrap();
                self.current_pos = Some(index);
                return self;
            }
        }
        warn!("trying to reach a non-exist, return orginal block");
        self
    }

    /// move one step right from the current position
    pub fn right(&mut self) -> &mut Self {
        if self.current_pos.is_some() {
            let pos = self.current_pos.unwrap();
            if self.blocks[pos].right.is_some() {
                let index = self.blocks[pos].right.unwrap();
                self.current_pos = Some(index);
                return self;
            }
        };
        warn!("trying to reach a non-exist, return orginal block");
        self
    }

    /// move one step up from the current position
    pub fn top(&mut self) -> &mut Self {
        if self.current_pos.is_some() {
            let pos = self.current_pos.unwrap();
            if self.blocks[pos].top.is_some() {
                let index = self.blocks[pos].top.unwrap();
                self.current_pos = Some(index);
                return self;
            }
        };
        warn!("trying to reach a non-exist, return orginal block");
        self
    }

    /// move one step down from the current position
    pub fn bottom(&mut self) -> &mut Self {
        if self.current_pos.is_some() {
            let pos = self.current_pos.unwrap();
            if self.blocks[pos].bottom.is_some() {
                let index = self.blocks[pos].bottom.unwrap();
                self.current_pos = Some(index);
                return self;
            }
        };
        warn!("trying to reach a non-exist, return orginal block");
        self
    }

    /// reset the current position to the first block
    pub fn reset(&mut self) -> &mut Self {
        if self.current_pos.is_some() {
            self.current_pos = Some(0);
            return self;
        }
        warn!("trying to reset position for an empty BlobBuilder");
        self
    }

    /// create the first block and return the nn_id of it
    pub fn create_first<T: Bundle>(
        &mut self,
        phy_block_bundle: PhysiBlockBundle,
        others: T,
    ) -> Option<usize> {
        let nn = BrainNN::default();
        self.nnvec.push(GenericNN::BRAINNN(nn));
        // push first so the real id should minus one
        let nn_id = self.nnvec.len() - 1;
        // println!("nnid={}",nn_id);

        let id = self
            .commands
            .spawn(
                phy_block_bundle
                    .clone()
                    .with_color(self.info.color)
                    .with_nn_id(nn_id, None)
                    .with_blob(self.blob_bundle.index()),
            )
            .insert(others)
            .insert(CenterBlockFlag)
            .id();

        let block = BlobBlock {
            id: id,
            top: None,
            bottom: None,
            left: None,
            right: None,
            vec_index: 0,
            size: phy_block_bundle.sprite.sprite.custom_size.unwrap() / 2.0,
            translation: phy_block_bundle.sprite.transform.translation.truncate(),
            anchors: phy_block_bundle.anchors,
            depth: 0,
            nn_id: nn_id,
        };

        // update blob_info in bundle
        self.info.init(block.translation, block.size);
        self.update_info();

        self.commands
            .entity(self.blob_bundle)
            .push_children(&[block.id]);
        self.blocks.push(block);
        self.current_pos = Some(0);

        Some(nn_id)
    }

    /// add a new block to the left of the current block and move the current position to that block
    pub fn add_to_left<T: Bundle>(
        &mut self,
        dx: f32,
        dy: f32,
        motor_pos: Option<f32>,
        motor_limits: Option<[f32; 2]>,
        others: T,
    ) -> Option<usize> {
        if self.current_pos.is_none() {
            warn!("trying to add a block while no parent block exist");
            return None;
        }
        let pos = self.current_pos.unwrap();
        let block = &mut self.blocks[pos];

        if block.left.is_some() {
            warn!("trying to add a block to an occupied position");
            return None;
        }

        let nn = BlockNN::default();
        self.nnvec.push(GenericNN::BLOCKNN(nn));
        let nn_id = self.nnvec.len() - 1;

        let spawn_x = block.translation.x - block.size.x - dx;
        let spawn_y = block.translation.y;
        let phy_block_bundle = PhysiBlockBundle::from_xy_dx_dy(spawn_x, spawn_y, dx, dy)
            .with_color(self.info.color)
            .with_density(DEFAULT_DENSITY)
            .with_nn_id(nn_id, Some(block.nn_id))
            .with_blob(self.blob_bundle.index())
            .with_parent_anchor(2);
        let id = self
            .commands
            .spawn(phy_block_bundle.clone())
            .insert(others)
            .id();
        let new_block = BlobBlock {
            id: id,
            top: None,
            bottom: None,
            left: None,
            right: Some(pos),
            size: phy_block_bundle.sprite.sprite.custom_size.unwrap() / 2.0,
            translation: phy_block_bundle.sprite.transform.translation.truncate(),
            anchors: phy_block_bundle.anchors,
            depth: block.depth + 1,
            vec_index: self.blocks.len(),
            nn_id: nn_id,
        };

        let block = &mut self.blocks[pos];
        block.left = Some(new_block.vec_index);
        self.current_pos = Some(new_block.vec_index);
        self.commands
            .entity(new_block.id)
            .insert(BlockDepth(new_block.depth));

        // set joint motor
        let mut stiff = 0.0;
        let mut motor_target = 0.0;
        if motor_pos.is_some() {
            stiff = MOTOR_STIFFNESS;
            motor_target = motor_pos.unwrap();
        }

        // set joint limits
        let mut limits = [-PI * 0.9, PI * 0.9];
        if motor_limits.is_some() {
            limits = motor_limits.unwrap()
        }

        let joint = RevoluteJointBuilder::new()
            .local_anchor1(block.anchors.left)
            .local_anchor2(new_block.anchors.right)
            .motor_position(motor_target, stiff, MOTOR_DAMPING)
            .limits(limits);

        bind_joint(&mut self.commands, block.id, new_block.id, joint);

        // update info
        self.info.add(block.translation, block.size);
        self.update_info();

        self.commands
            .entity(self.blob_bundle)
            .push_children(&[new_block.id]);
        self.blocks.push(new_block);

        Some(nn_id)
    }

    /// add a new block to the right of the current block and move the current position to that block
    pub fn add_to_right<T: Bundle>(
        &mut self,
        dx: f32,
        dy: f32,
        motor_pos: Option<f32>,
        motor_limits: Option<[f32; 2]>,
        others: T,
    ) -> Option<usize> {
        if self.current_pos.is_none() {
            warn!("trying to add a block while no parent block exist");
            return None;
        }
        let pos = self.current_pos.unwrap();
        let block = &mut self.blocks[pos];

        if block.right.is_some() {
            warn!("trying to add a block to an occupied position");
            return None;
        }

        let nn = BlockNN::default();
        self.nnvec.push(GenericNN::BLOCKNN(nn));
        let nn_id = self.nnvec.len() - 1;

        let spawn_x = block.translation.x + block.size.x + dx;
        let spawn_y = block.translation.y;
        let phy_block_bundle = PhysiBlockBundle::from_xy_dx_dy(spawn_x, spawn_y, dx, dy)
            .with_color(self.info.color)
            .with_density(DEFAULT_DENSITY)
            .with_nn_id(nn_id, Some(block.nn_id))
            .with_blob(self.blob_bundle.index())
            .with_parent_anchor(3);
        let id = self
            .commands
            .spawn(phy_block_bundle.clone())
            .insert(others)
            .id();
        let new_block = BlobBlock {
            id: id,
            top: None,
            bottom: None,
            left: Some(pos),
            right: None,
            size: phy_block_bundle.sprite.sprite.custom_size.unwrap() / 2.0,
            translation: phy_block_bundle.sprite.transform.translation.truncate(),
            anchors: phy_block_bundle.anchors,
            depth: block.depth + 1,
            vec_index: self.blocks.len(),
            nn_id: nn_id,
        };

        let block = &mut self.blocks[pos];
        block.right = Some(new_block.vec_index);
        self.current_pos = Some(new_block.vec_index);
        self.commands
            .entity(new_block.id)
            .insert(BlockDepth(new_block.depth));

        // set joint motor
        let mut stiff = 0.0;
        let mut motor_target = 0.0;
        if motor_pos.is_some() {
            stiff = MOTOR_STIFFNESS;
            motor_target = motor_pos.unwrap();
        }

        // set joint limits
        let mut limits = [-PI * 0.9, PI * 0.9];
        if motor_limits.is_some() {
            limits = motor_limits.unwrap()
        }

        let joint = RevoluteJointBuilder::new()
            .local_anchor1(block.anchors.right)
            .local_anchor2(new_block.anchors.left)
            .motor_position(motor_target, stiff, MOTOR_DAMPING)
            .limits(limits);

        bind_joint(&mut self.commands, block.id, new_block.id, joint);

        // update info
        self.info.add(block.translation, block.size);
        self.update_info();

        self.commands
            .entity(self.blob_bundle)
            .push_children(&[new_block.id]);
        self.blocks.push(new_block);

        Some(nn_id)
    }

    /// add a new block to the top of the current block and move the current position to that block
    pub fn add_to_top<T: Bundle>(
        &mut self,
        dx: f32,
        dy: f32,
        motor_pos: Option<f32>,
        motor_limits: Option<[f32; 2]>,
        others: T,
    ) -> Option<usize> {
        if self.current_pos.is_none() {
            warn!("trying to add a block while no parent block exist");
            return None;
        }
        let pos = self.current_pos.unwrap();
        let block = &mut self.blocks[pos];

        if block.top.is_some() {
            warn!("trying to add a block to an occupied position");
            return None;
        }

        let nn = BlockNN::default();
        self.nnvec.push(GenericNN::BLOCKNN(nn));
        let nn_id = self.nnvec.len() - 1;

        let spawn_x = block.translation.x;
        let spawn_y = block.translation.y + block.size.y + dy;
        let phy_block_bundle = PhysiBlockBundle::from_xy_dx_dy(spawn_x, spawn_y, dx, dy)
            .with_color(self.info.color)
            .with_density(DEFAULT_DENSITY)
            .with_nn_id(nn_id, Some(block.nn_id))
            .with_blob(self.blob_bundle.index())
            .with_parent_anchor(0);
        let id = self
            .commands
            .spawn(phy_block_bundle.clone())
            .insert(others)
            .id();
        let new_block = BlobBlock {
            id: id,
            top: None,
            bottom: Some(pos),
            left: None,
            right: None,
            size: phy_block_bundle.sprite.sprite.custom_size.unwrap() / 2.0,
            translation: phy_block_bundle.sprite.transform.translation.truncate(),
            anchors: phy_block_bundle.anchors,
            depth: block.depth + 1,
            vec_index: self.blocks.len(),
            nn_id: nn_id,
        };

        let block = &mut self.blocks[pos];
        block.top = Some(new_block.vec_index);
        self.current_pos = Some(new_block.vec_index);
        self.commands
            .entity(new_block.id)
            .insert(BlockDepth(new_block.depth));

        // set joint motor
        let mut stiff = 0.0;
        let mut motor_target = 0.0;
        if motor_pos.is_some() {
            stiff = MOTOR_STIFFNESS;
            motor_target = motor_pos.unwrap();
        }

        // set joint limits
        let mut limits = [-PI * 0.9, PI * 0.9];
        if motor_limits.is_some() {
            limits = motor_limits.unwrap()
        }

        let joint = RevoluteJointBuilder::new()
            .local_anchor1(block.anchors.top)
            .local_anchor2(new_block.anchors.bottom)
            .motor_position(motor_target, stiff, MOTOR_DAMPING)
            .limits(limits);

        bind_joint(&mut self.commands, block.id, new_block.id, joint);

        // update info
        self.info.add(block.translation, block.size);
        self.update_info();

        self.commands
            .entity(self.blob_bundle)
            .push_children(&[new_block.id]);
        self.blocks.push(new_block);

        Some(nn_id)
    }

    /// add a new block to the bottom of the current block and move the current position to that block
    pub fn add_to_bottom<T: Bundle>(
        &mut self,
        dx: f32,
        dy: f32,
        motor_pos: Option<f32>,
        motor_limits: Option<[f32; 2]>,
        others: T,
    ) -> Option<usize> {
        if self.current_pos.is_none() {
            warn!("trying to add a block while no parent block exist");
            return None;
        }
        let pos = self.current_pos.unwrap();
        let block = &mut self.blocks[pos];

        if block.bottom.is_some() {
            warn!("trying to add a block to an occupied position");
            return None;
        }

        let nn = BlockNN::default();
        self.nnvec.push(GenericNN::BLOCKNN(nn));
        let nn_id = self.nnvec.len() - 1;

        let spawn_x = block.translation.x;
        let spawn_y = block.translation.y - block.size.y - dy;
        let phy_block_bundle = PhysiBlockBundle::from_xy_dx_dy(spawn_x, spawn_y, dx, dy)
            .with_color(self.info.color)
            .with_density(DEFAULT_DENSITY)
            .with_nn_id(nn_id, Some(block.nn_id))
            .with_blob(self.blob_bundle.index())
            .with_parent_anchor(1);
        let id = self
            .commands
            .spawn(phy_block_bundle.clone())
            .insert(others)
            .id();
        let new_block = BlobBlock {
            id: id,
            top: Some(pos),
            bottom: None,
            left: None,
            right: None,
            size: phy_block_bundle.sprite.sprite.custom_size.unwrap() / 2.0,
            translation: phy_block_bundle.sprite.transform.translation.truncate(),
            anchors: phy_block_bundle.anchors,
            depth: block.depth + 1,
            vec_index: self.blocks.len(),
            nn_id: nn_id,
        };

        let block = &mut self.blocks[pos];
        block.bottom = Some(new_block.vec_index);
        self.current_pos = Some(new_block.vec_index);
        self.commands
            .entity(new_block.id)
            .insert(BlockDepth(new_block.depth));

        // set joint motor
        let mut stiff = 0.0;
        let mut motor_target = 0.0;
        if motor_pos.is_some() {
            stiff = MOTOR_STIFFNESS;
            motor_target = motor_pos.unwrap();
        }

        // set joint limits
        let mut limits = [-PI * 0.9, PI * 0.9];
        if motor_limits.is_some() {
            limits = motor_limits.unwrap()
        }

        let joint = RevoluteJointBuilder::new()
            .local_anchor1(block.anchors.bottom)
            .local_anchor2(new_block.anchors.top)
            .motor_position(motor_target, stiff, MOTOR_DAMPING)
            .limits(limits);

        bind_joint(&mut self.commands, block.id, new_block.id, joint);

        // update info
        self.info.add(block.translation, block.size);
        self.update_info();

        self.commands
            .entity(self.blob_bundle)
            .push_children(&[new_block.id]);
        self.blocks.push(new_block);

        Some(nn_id)
    }

    /// update info inside the blob_bundle
    fn update_info(&mut self) {
        self.commands
            .entity(self.blob_bundle)
            .insert(self.info.clone());
    }
}

// helper function
pub fn bind_joint(
    commands: &mut Commands,
    parent: Entity,
    child: Entity,
    joint: RevoluteJointBuilder,
) {
    commands.entity(child).with_children(|cmd| {
        let mut new_joint = ImpulseJoint::new(parent, joint);
        new_joint.data.set_contacts_enabled(ENABLE_CONTACTS);
        cmd.spawn(new_joint);
    });
}
