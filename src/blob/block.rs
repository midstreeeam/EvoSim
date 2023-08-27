//! block, the smallest rigid body unit in the project, used to construct blobs

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{componet::{ColliderFlag, BlobEntityIndex}, consts::{DEFAULT_DAMPING_LINEAR, DEFAULT_DAMPING_ANGULAR}};

#[derive(Component)]
pub struct CenterBlockFlag;

/// Anchor points that other joints can connect to.
#[derive(Component, Clone, Debug)]
pub struct BlockAnchors {
    pub top: Vec2,
    pub bottom: Vec2,
    pub left: Vec2,
    pub right: Vec2,
}

impl BlockAnchors {
    pub fn from_xy(dx: f32, dy: f32) -> Self {
        Self {
            top: Vec2 { x: 0.0, y: dy },
            bottom: Vec2 { x: 0.0, y: -dy },
            left: Vec2 { x: -dx, y: 0.0 },
            right: Vec2 { x: dx, y: 0.0 },
        }
    }
}

/// BlockDepth is a u32 which represent the depth of the block
/// in the blob tree
///
/// Depth determines the processing order of Neural Network
#[derive(Component, Clone, Debug)]
pub struct BlockDepth(pub u32);


/// ParentAnchor can only be 0(up), 1(down), 2(left), 3(right)
/// 
/// Considering using enum
#[derive(Component, Clone, Debug)]
pub struct ParentAnchor(pub Option<usize>);

/// id for relate Neuron
#[derive(Component, Clone, Debug)]
pub struct NeuronId{
    pub id:usize,
    pub parent_id:Option<usize>
}

impl Default for NeuronId {
    fn default() -> Self {
        Self { id: 0, parent_id: None }
    }
}

impl NeuronId {
    pub fn new(id:usize, parent_id:Option<usize>) -> Self {
        Self { id: id, parent_id: parent_id }
    }
}

/// JointInfor for joint sensors
#[derive(Component, Clone)]
pub struct JointInfo{
    pub ang_pos:f32,
    pub ang_velocity:f32,
}

impl JointInfo {
    pub fn update(&mut self,pos:f32,v:f32){
        self.ang_pos=pos;
        self.ang_velocity=v;
    }
}

/// PhysiBlockBundle is the smallest unit in this simulation.
/// It is the cubiod that construct blobs.
#[derive(Bundle, Clone)]
pub struct PhysiBlockBundle {
    // basic config
    pub sprite: SpriteBundle,
    pub collider: Collider,
    pub rigbody: RigidBody,
    pub event_flag: ActiveEvents,
    pub depth: BlockDepth,

    // physical config
    pub velocity: Velocity,
    pub massprop: ColliderMassProperties,
    pub friction: Friction,
    pub restitution: Restitution,
    pub damping: Damping,

    pub ex_force: ExternalForce,
    pub ex_impulse: ExternalImpulse,

    // helper componet for builder
    pub anchors: BlockAnchors,

    // connect to which side of parent
    pub parent_anchor: ParentAnchor,

    /// neuron id
    /// 
    /// id=0 is the default id, means random neuron output
    pub neuron_id: NeuronId,
    pub type_falg: ColliderFlag,
    pub joint_info: JointInfo
}

impl Default for PhysiBlockBundle {
    fn default() -> Self {
        let default_rad = 1.0;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.55),
                    custom_size: Some(Vec2 {
                        x: default_rad,
                        y: default_rad,
                    }),
                    ..default()
                },
                transform: Transform::default(),
                ..default()
            },
            neuron_id: NeuronId::default(),
            depth: BlockDepth(0),
            anchors: BlockAnchors::from_xy(default_rad, default_rad),
            collider: Collider::cuboid(default_rad / 2.0, default_rad / 2.0),
            rigbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            massprop: ColliderMassProperties::Density(1.0),
            friction: Friction::default(),
            restitution: Restitution::default(),
            damping: Damping{
                linear_damping: DEFAULT_DAMPING_LINEAR,
                angular_damping: DEFAULT_DAMPING_ANGULAR
            },
            ex_force: ExternalForce::default(),
            ex_impulse: ExternalImpulse::default(),
            // contact_force_events for sensor
            event_flag: ActiveEvents::CONTACT_FORCE_EVENTS,
            // default JointInfo is all 0
            joint_info: JointInfo { ang_pos: 0.0, ang_velocity: 0.0 },
            type_falg: ColliderFlag::BLOCK(BlobEntityIndex(None)),
            parent_anchor: ParentAnchor(None)
        }
    }
}

impl PhysiBlockBundle {
    pub fn from_xy_dx_dy(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.55),
                    custom_size: Some(Vec2 {
                        x: 2.0 * dx,
                        y: 2.0 * dy,
                    }),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            collider: Collider::cuboid(dx, dy),
            anchors: BlockAnchors::from_xy(dx, dy),
            ..default()
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.sprite.sprite.color = color
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.sprite.sprite.color = color;
        self
    }

    pub fn with_density(mut self, density: f32) -> Self {
        self.massprop = ColliderMassProperties::Density(density);
        self
    }

    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = BlockDepth(depth);
        self
    }

    pub fn with_nn_id(mut self, nn_id: usize, parent_nn_id: Option<usize>) -> Self{
        self.neuron_id = NeuronId{id: nn_id, parent_id: parent_nn_id};
        self
    }

    pub fn with_blob(mut self, blob_id: u32) -> Self{
        self.type_falg = ColliderFlag::BLOCK(BlobEntityIndex(Some(blob_id)));
        self
    }

    pub fn with_parent_anchor(mut self, parent_anchor:usize) -> Self{
        self.parent_anchor = ParentAnchor(Some(parent_anchor));
        self
    }
}
