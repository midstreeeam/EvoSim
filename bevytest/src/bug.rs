
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(test)
        .run()
}

fn test(mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>){

    commands.spawn(Camera2dBundle::default());
    rapier_config.gravity = Vec2::ZERO;

    let parent = commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 25.0),
        TransformBundle::from(Transform::from_translation(Vec3::new(150.0,0.0,0.0))),
        Velocity{
            linvel: Vec2 { x: 0.0, y: 0.0 },

            // Here, change the angvel will make joint limit act differently
            angvel: 50.0, // limit in PI
            // angvel: 5.0 // sometimes I can see it pass through the 'limit' and sometimes not
            // angvel: 1.0 // normal limit
        },
    )).id();

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(150.0, 0.0))
        .limits([-3.0,0.0]);
    
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 25.0),
        TransformBundle::from(Transform::from_translation(Vec3::new(0.0,0.0,0.0))),
        ImpulseJoint::new(parent, joint),
    ));
}