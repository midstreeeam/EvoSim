mod physics;
mod block;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use physics::physical_world;
use block::PhysiBlockBundle;


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

#[derive(Component)]
pub struct ControlFlag;

fn setup_test(
    mut commands: Commands,
) {
    let body = commands
        .spawn(PhysiBlockBundle::from_xy_dx_dy(0.0, 0.0, 25.0, 50.0)).id();
    
    let right_1 = commands.spawn((
        PhysiBlockBundle::from_xy_dx_dy(75.0, 0.0, 50.0, 10.0)
        .with_density(0.5),
    )).id();

    let right_2 = commands.spawn((
        PhysiBlockBundle::from_xy_dx_dy(175.0, 0.0, 50.0, 10.0)
        .with_density(0.5),
    )).id();

    let left_1 = commands.spawn((
        PhysiBlockBundle::from_xy_dx_dy(-75.0, 0.0, 50.0, 10.0)
        .with_density(0.5),
    )).id();

    let left_2 = commands.spawn((
        PhysiBlockBundle::from_xy_dx_dy(-175.0, 0.0, 50.0, 10.0)
        .with_density(0.5),
    )).id();

    let joint_right_1 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(25.0, 0.0))
        .local_anchor2(Vec2::new(-50.0, 0.0))
        .limits([-2.0,2.0]);

    let joint_right_2 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(50.0, 0.0))
        .local_anchor2(Vec2::new(-50.0, 0.0))
        .limits([-2.0,0.0]);

    let joint_left_1 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-25.0, 0.0))
        .local_anchor2(Vec2::new(50.0, 0.0))
        .limits([-2.0,2.0]);

    let joint_left_2 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-50.0, 0.0))
        .local_anchor2(Vec2::new(50.0, 0.0))
        .limits([0.0,2.0]);

    bind_joint(&mut commands, body, right_1, joint_right_1);
    bind_joint(&mut commands, right_1, right_2, joint_right_2);
    bind_joint(&mut commands, body, left_1, joint_left_1);
    bind_joint(&mut commands, left_1, left_2, joint_left_2);

}

fn bind_joint(commands: &mut Commands, parent: Entity, child: Entity, joint: RevoluteJointBuilder){
    commands.entity(child).with_children(|cmd| {
        let mut new_joint = ImpulseJoint::new(parent, joint);
        new_joint.data.set_contacts_enabled(false);
        cmd.spawn(new_joint);
    });
}