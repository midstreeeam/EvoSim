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

#[derive(SystemParam)]
struct Myhook{}

impl BevyPhysicsHooks for Myhook{
    fn filter_contact_pair(&self, _context: PairFilterContextView) -> Option<SolverFlags> {
        None
    }
}
