use bevy::ecs::system::{SystemParam};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::pipeline::BevyPhysicsHooks;
mod physical_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<Myhook>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(physical_world::PhysiWorld)
        .add_startup_system(setup_physics)
        .add_startup_system(spawn_worm)
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
            ActiveHooks::FILTER_CONTACT_PAIRS
        ))
        .id();

        if i>0{
            let parent_entity = *unit_entities.last().unwrap();
                let joint = RevoluteJointBuilder::new()
                    .local_anchor1(Vec2::new(40.0, 0.0))
                    .local_anchor2(Vec2::new(-40.0, 0.0));
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
                angvel: -10.0
            },
            ActiveHooks::FILTER_CONTACT_PAIRS
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
        .local_anchor2(Vec2::new(150.0, 0.0)).limits([-0.5,0.5]);
    
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
        ActiveHooks::FILTER_CONTACT_PAIRS
    ));
}

// fn get_linked_entities(mut commands: Commands,joint_query:Query<(&ImpulseJoint, &Collider)>){
//     for (joint,child_collider) in joint_query.iter(){
//         let parent_collider = commands.entity(joint.parent);
//     }
// }

#[derive(SystemParam)]
struct Myhook{}


impl BevyPhysicsHooks for Myhook{
    fn filter_contact_pair(&self, _context: PairFilterContextView) -> Option<SolverFlags> {
        None
    }
}
