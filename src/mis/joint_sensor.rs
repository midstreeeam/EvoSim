use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_physics)
        .add_system(print_angle)
        .run();
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.55, 0.25, 0.25))),
            transform: Transform::from_translation(Vec3::new(150., 100., 0.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Friction::default(),
        Restitution::default(),
        Velocity{
            linvel: Vec2 { x: -300.0, y: 100.0 },
            angvel: 0.0
        }
    ));


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

fn print_angle(
    joints: Query<(&Transform, &ImpulseJoint, &Velocity)>,
    tvq: Query<(&Transform, &Velocity)>
){
    for (child_transform,joint, child_v) in joints.iter(){
        if let Ok((parent_transform,parent_v)) = tvq.get(joint.parent) {
            print!("{:#?}   ",get_relative_rotation(parent_transform, child_transform));
            println!("{:#?}",get_relative_angular_velocity(parent_v, child_v));
        }
    }
}

fn get_relative_rotation(transform1: &Transform, transform2: &Transform) -> f32{
    let r1 = transform1.rotation;
    let r2 = transform2.rotation;
    r1.z.atan2(r1.w)*360.0/PI-r2.z.atan2(r2.w)*360.0/PI
}

fn get_relative_angular_velocity(v1: &Velocity, v2:&Velocity) ->f32{
    (v1.angvel-v2.angvel)/PI*180.0
}



fn print_angle(
    tq: Query<&Transform>,
    mut q: Query<(&Parent, &mut ImpulseJoint, &ControlFlag)>
){
    for (entity, mut joint,flag) in q.iter_mut(){
        if let ControlFlag::Left = flag{
            let parent_transform = tq.get(joint.parent).ok().unwrap();
            let child_transform = tq.get(entity.get()).ok().unwrap();
            let theta = get_relative_rotation(parent_transform,child_transform);
           
        }
    }
}

fn get_relative_rotation(transform1: &Transform, transform2: &Transform) -> f32{
    let r1 = transform1.rotation;
    let r2 = transform2.rotation;
    r1.z.atan2(r1.w)*360.0/PI-r2.z.atan2(r2.w)*360.0/PI
}