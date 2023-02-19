use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::config::*;

#[derive(Component)]
pub struct Blob;

#[derive(Bundle)]
pub struct BlobRapierBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,

    pub mass_properties: ColliderMassProperties,
    pub velocity: Velocity,
    pub restitution: Restitution,
    pub damping: Damping,

    pub sleeping: Sleeping,
    pub ccd: Ccd,
}

#[derive(Bundle)]
pub struct BlobBundle {
    pub blob: Blob,
    pub rapier: BlobRapierBundle,
    pub material: MaterialMesh2dBundle<ColorMaterial>,
}

impl BlobBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> Self {
        Self {
            blob: Blob,
            rapier: BlobRapierBundle {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::ball(BLOB_RADIUS),
                // transform_bundle: TransformBundle::from(Transform::from_xyz(0.0, 100.0, 0.0)),

                mass_properties: ColliderMassProperties::Density(BLOB_DENSITY),
                restitution: Restitution::coefficient(1.5),
                velocity: Velocity::angular(1.0),

                damping: Damping {
                    linear_damping: LINEAR_DAMPING,
                    angular_damping: ANGULAR_DAMPING,
                },

                sleeping: Sleeping::disabled(),
                ccd: Ccd::enabled(),
            },
            material: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(BLOB_RADIUS))).into(),
                material: materials.add(ColorMaterial::from(BLOB_COLOR)),
                transform: Transform{
                    translation: Vec3::new(0.0, 100.0, 1.0),
                    ..default()
                },
                ..default()
            }
        }
    }
}
