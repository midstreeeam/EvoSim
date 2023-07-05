use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


#[derive(Bundle)]
pub struct PhysiBlockBundle{
    pub sprite: SpriteBundle,
    pub collider: Collider,
    pub rigbody: RigidBody,

    pub velocity: Velocity,
    pub massprop: ColliderMassProperties,
    pub friction: Friction,
    pub restitution: Restitution,
    pub damping: Damping,

    pub ex_force: ExternalForce,
    pub ex_impulse: ExternalImpulse
}

impl Default for PhysiBlockBundle{
    fn default() -> Self {
        let default_rad = 10.0;
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
                    custom_size: Some(Vec2 { x: 2.0*dx, y: 2.0*dy }),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x,y,0.0)),
                ..default()
            },
            collider: Collider::cuboid(dx, dy),
            ..default()
        }
    }

    // pub fn set_color(&mut self, color:Color){
    //     self.sprite.sprite.color=color
    // }

    pub fn with_density(mut self, density:f32) -> Self{
        self.massprop = ColliderMassProperties::Density(density);
        self
    }
}