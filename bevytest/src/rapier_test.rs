use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub fn bounce() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration{
            gravity: Vec2 { x: 0.0, y: -98.1 },
            ..default()
        })
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .add_system(modify_body_damping)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Collider::ball(30.0))
        .insert(ColliderMassProperties::Density(2.0))
        .insert(Restitution::coefficient(1.5))
        .insert(Velocity::angular(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn modify_body_damping(mut dampings: Query<&mut Damping>) {
    for mut rb_damping in dampings.iter_mut() {
        rb_damping.linear_damping = 1.0;
        rb_damping.angular_damping = 1.0;
    }
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}