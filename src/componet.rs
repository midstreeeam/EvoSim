use bevy::prelude::*;

/// Every collider should have a type flag.
/// Then the sensor can know the collision type.
#[derive(Debug, Component, Clone)]
pub enum ColliderFlag {
    WALL,
    BLOCK
}