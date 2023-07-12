use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct BlockAnchors{
    pub top: Vec2,
    pub bottom: Vec2,
    pub left: Vec2,
    pub right: Vec2
}

impl BlockAnchors {
    pub fn from_xy(dx:f32, dy:f32) -> Self{
        Self { 
            top: Vec2 { x: 0.0, y: dy },
            bottom: Vec2 { x: 0.0, y: -dy },
            left: Vec2 { x: -dx, y: 0.0 },
            right: Vec2 { x: dx, y: 0.0 }
        }
    }
}

/// BlockDepth is a u32 which represent the depth of the block
/// in the blob tree
/// 
/// Depth determines the processing order of Neural Network
#[derive(Component, Clone, Debug)]
pub struct BlockDepth(pub u32);

/// PhysiBlockBundle is the smallest unit in this simulation.
/// It is the cubiod that construct blobs.
#[derive(Bundle,Clone)]
pub struct PhysiBlockBundle{
    // basic config
    pub sprite: SpriteBundle,
    pub collider: Collider,
    pub rigbody: RigidBody,
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
    pub anchors: BlockAnchors
}

impl Default for PhysiBlockBundle{
    fn default() -> Self {
        let default_rad = 1.0;
        Self { 
            sprite: SpriteBundle { 
                sprite: Sprite { 
                    color: Color::rgb(0.25, 0.25, 0.55),
                    custom_size: Some(Vec2 { x: default_rad, y: default_rad }),
                    ..default()
                },
                transform: Transform::default(),
                ..default()
            },
            depth: BlockDepth(0),
            anchors: BlockAnchors::from_xy(default_rad, default_rad),
            collider: Collider::cuboid(default_rad/2.0, default_rad/2.0),
            rigbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            massprop: ColliderMassProperties::Density(1.0),
            friction: Friction::default(),
            restitution: Restitution::default(),
            damping: Damping::default(),
            ex_force: ExternalForce::default(),
            ex_impulse: ExternalImpulse::default()
        }
    }
}

impl PhysiBlockBundle{
    pub fn from_xy_dx_dy(x:f32, y:f32, dx: f32, dy:f32) -> Self{
        Self { 
            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.55),
                    custom_size: Some(Vec2 { x: 2.0*dx , y: 2.0*dy }),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x,y,0.0)),
                ..default()
            },
            collider: Collider::cuboid(dx, dy),
            anchors: BlockAnchors::from_xy(dx, dy),
            ..default()
        }
    }

    pub fn set_color(&mut self, color:Color) {
        self.sprite.sprite.color=color
    }

    pub fn with_color(mut self, color:Color) -> Self{
        self.sprite.sprite.color=color;
        self
    }

    pub fn with_density(mut self, density:f32) -> Self{
        self.massprop = ColliderMassProperties::Density(density);
        self
    }

    pub fn with_depth(mut self, depth:u32) -> Self{
        self.depth = BlockDepth(depth);
        self
    }
}