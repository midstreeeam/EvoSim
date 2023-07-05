mod physics;
mod block;
mod graphics;

use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::*,
    rapier::prelude::JointAxis
};

use physics::physical_world;
use block::PhysiBlockBundle;
use graphics::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(physical_world::PhysiWorld)
        .add_plugin(Graphics)
        .add_startup_system(setup_test)
        .add_system(contorl)
        // .add_system(print_angle)
        .run();
}

fn setup_test(
    mut commands: Commands,
) {
    let body = commands
        .spawn(
            PhysiBlockBundle::from_xy_dx_dy(0.0, 0.0, 25.0, 50.0)
            .with_density(1.0)
    ).id();
    
    let right_1 = commands.spawn((
        PhysiBlockBundle::from_xy_dx_dy(75.0, 0.0, 50.0, 10.0)
        .with_density(0.1),
    )).id();

    let right_2 = commands.spawn(
        PhysiBlockBundle::from_xy_dx_dy(175.0, 0.0, 50.0, 10.0)
        .with_density(0.1)
    ).id();

    let left_1 = commands.spawn((
        PhysiBlockBundle::from_xy_dx_dy(-75.0, 0.0, 50.0, 10.0)
        .with_density(0.1),
    )).id();

    let left_2 = commands.spawn((
        PhysiBlockBundle::from_xy_dx_dy(-175.0, 0.0, 50.0, 10.0)
        .with_density(0.1),
    )).id();

    let joint_right_1 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(25.0, 0.0))
        .local_anchor2(Vec2::new(-50.0, 0.0))
        .limits([-85f32.to_radians(),85f32.to_radians()]);

    let joint_right_2 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(50.0, 0.0))
        .local_anchor2(Vec2::new(-50.0, 0.0))
        .limits([-135f32.to_radians(),0.0]);

    let joint_left_1 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-25.0, 0.0))
        .local_anchor2(Vec2::new(50.0, 0.0))
        .limits([-85f32.to_radians(),85f32.to_radians()]);

    let joint_left_2 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-50.0, 0.0))
        .local_anchor2(Vec2::new(50.0, 0.0))
        .limits([0.0,135f32.to_radians()]);

    bind_joint(&mut commands, body, right_1, joint_right_1, Some(ControlFlag::Right));
    bind_joint(&mut commands, right_1, right_2, joint_right_2, None);
    bind_joint(&mut commands, body, left_1, joint_left_1, Some(ControlFlag::Left));
    bind_joint(&mut commands, left_1, left_2, joint_left_2, None);

}

#[derive(Component)]
pub enum ControlFlag{
    Left,
    Right,
}

#[derive(Component)]
pub struct TestFlag;

fn bind_joint(
    commands: &mut Commands,
    parent: Entity,
    child: Entity,
    joint: RevoluteJointBuilder,
    control_flag: Option<ControlFlag>
){
    commands.entity(child).with_children(|cmd| {
        let mut new_joint = ImpulseJoint::new(parent, joint);
        new_joint.data.set_contacts_enabled(false);
        match control_flag {
            Some(flag) => {
                cmd.spawn((new_joint, flag));
            },
            None => {
                cmd.spawn(new_joint);
            },
        }
    });
}

pub fn contorl(
    keyboard_input: Res<Input<KeyCode>>,
    mut joint_query: Query<(&mut ImpulseJoint, &ControlFlag)>,
    // time: Res<Time>,
) {
    
    let a_press= keyboard_input.pressed(KeyCode::A);
    let d_press = keyboard_input.pressed(KeyCode::D);
    for (mut joint,flag) in joint_query.iter_mut(){
        match (a_press,d_press,flag) {
            (true,false,ControlFlag::Left) => {
                joint.data.set_motor(
                    JointAxis::AngX,90f32.to_radians(),0.0,100.0,3.0
                );
            },
            (true,false,ControlFlag::Right) => {
                joint.data.set_motor(
                    JointAxis::AngX,-90f32.to_radians(),0.0,100.0,3.0
                );
            },
            (false,true,ControlFlag::Left) => {
                joint.data.set_motor(
                    JointAxis::AngX,-90f32.to_radians(),0.0,100.0,3.0
                );
            },
            (false,true,ControlFlag::Right) => {
                joint.data.set_motor(
                    JointAxis::AngX, 90f32.to_radians(),0.0,100.0,3.0
                );
            },
            _ => {
                joint.data.set_motor(
                    JointAxis::AngX,0.0,0.0,0.0,0.0
                );
            }
        }
    }
    
}