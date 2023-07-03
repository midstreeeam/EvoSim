use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};
use bevy_rapier2d::prelude::*;

mod physics;
use physics::physical_world;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(physical_world::PhysiWorld)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_test)
        .run();
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let sq = commands
        .spawn((
            SpriteBundle{
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.55, 0.25),
                    custom_size: Some(Vec2 { x: 100.0, y: 50.0 }),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(150.0,0.0,0.0)),
                ..default()
            },
            Collider::cuboid(50.0, 25.0),
            RigidBody::Dynamic,
            Velocity{
                linvel: Vec2 { x: 0.0, y: 0.0 },
                angvel: 1.0
            },
        )).id();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.55, 0.25, 0.25))),
            transform: Transform::from_translation(Vec3::new(150., 100., 0.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Velocity{
            linvel: Vec2 { x: -300.0, y: 100.0 },
            angvel: 0.0
        }
    ));

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(150.0, 0.0))
        // .motor(PI/2.0, 0.0, 10.0, 0.3)
        .limits([-3.0,3.0]);
    
    commands.spawn((
        SpriteBundle{
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.55),
                custom_size: Some(Vec2 { x: 100.0, y: 50.0 }),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 25.0),
        ImpulseJoint::new(sq, joint),
        Velocity{
            linvel: Vec2 { x: 0.0, y: 0.0 },
            angvel: 0.0
        }
    ));
}