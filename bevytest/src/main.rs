use std::f32::consts::PI;

// use bevy::ecs::system::{SystemParam};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};
use bevy_rapier2d::prelude::*;
// use bevy_rapier2d::pipeline::BevyPhysicsHooks;
mod physical_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(physical_world::PhysiWorld)
        .add_startup_system(setup_physics)
        //.add_startup_system(spawn_worm)
        .add_system(print_angle)
        .run();
}

fn spawn_worm(mut commands: Commands){
    let worm_length = 8;
    let worm_unit_width = 10 as f32;
    let worm_unit_length = 20 as f32;
    let mut unit_entities: Vec<Entity> = Vec::new();

    for i in 0..worm_length{
        let unit_entity = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(i as f32 * 80.0, -200.0, 0.0)),
            RigidBody::Dynamic,
            Collider::cuboid(worm_unit_length, worm_unit_width),
        ))
        .id();

        if i>0{
            let parent_entity = *unit_entities.last().unwrap();
                let joint = RevoluteJointBuilder::new()
                    .local_anchor1(Vec2::new(20.0, 0.0))
                    .local_anchor2(Vec2::new(-20.0, 0.0))
                    .limits([-1.0,5.0]);
                commands.entity(unit_entity).with_children(|cmd| {
                    // NOTE: we want to attach multiple impulse joints to this entity, so
                    //       we need to add the components to children of the entity. Otherwise
                    //       the second joint component would just overwrite the first one.
                    let mut new_joint = ImpulseJoint::new(parent_entity, joint);
                    new_joint.data.set_contacts_enabled(false);
                    cmd.spawn(new_joint);
                });
        }
        unit_entities.push(unit_entity);
    }
}


fn setup_physics(
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
// #[derive(SystemParam)]
// struct Myhook{}

// impl BevyPhysicsHooks for Myhook{
//     fn filter_contact_pair(&self, _context: PairFilterContextView) -> Option<SolverFlags> {
//         None
//     }
// }
