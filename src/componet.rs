use bevy::prelude::*;

/// Every collider should have a type flag.
/// Then the sensor can know the collision type.
#[derive(Debug, Component, Clone)]
pub enum ColliderFlag {
    WALL,
    BLOCK(BlobEntityIndex)
}

/// denote which blob it belongs to.
/// The u32 value is the idx value inside `Entity` class
#[derive(Component, Clone, Debug)]
pub struct BlobEntityIndex(pub Option<u32>);