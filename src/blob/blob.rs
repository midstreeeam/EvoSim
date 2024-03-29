//! `BlobInfo` componet and `BlobBundle`

use std::f32::NAN;

use bevy::prelude::*;

/// flag of a blob entity
#[derive(Component)]
pub struct Blob;

/// informations about the blob.
/// 
/// Notice: `xbound` and `ybound` are deprecated.
#[derive(Component, Clone, Debug)]
pub struct BlobInfo {
    pub center_block_pos: Vec2,
    // bound: [min,max] to represent size
    pub xbound: [f32; 2],
    pub ybound: [f32; 2],
    pub color: Color,
    pub mass_center: [f32;2],
    /// velocity is base on `mass_center`
    pub velocity: [f32;2],
    /// cumulated moving distance,
    /// base on `mass_center`
    pub move_distance: [f32;2],
    pub crowding_distance: f32
}

impl Default for BlobInfo {
    fn default() -> Self {
        Self {
            center_block_pos: Vec2::NAN,
            xbound: [NAN, NAN],
            ybound: [NAN, NAN],
            color: Color::LIME_GREEN,
            mass_center: [0.0, 0.0],
            velocity: [0.0,0.0],
            move_distance: [0.0,0.0],
            crowding_distance: 0.0
        }
    }
}

impl BlobInfo {
    pub fn init(&mut self, center: Vec2, size: Vec2) {
        self.center_block_pos = center;
        self.xbound = [center.x - size.x, center.x + size.x];
        self.ybound = [center.y - size.y, center.y + size.y];
    }

    /// Add geometric infomation of new blocks in blob,
    /// update blobinfo accordingly
    ///
    /// This function should only be called by BlobBuilder
    pub fn add(&mut self, translation: Vec2, size: Vec2) {
        let large = translation + size;
        let small = translation - size;

        self.xbound[0] = self.xbound[0].min(small.x);
        self.xbound[1] = self.xbound[1].max(large.x);
        self.ybound[0] = self.ybound[0].min(small.y);
        self.ybound[1] = self.ybound[1].max(large.y);
    }
}

/// also contains blobgeno, but been added in `BlobBuilder::update_geno` function
#[derive(Bundle)]
pub struct BlobBundle {
    // flag
    blob_flag: Blob,

    // set visibility so it's children can be seen
    visibility: Visibility,
    computed_visibility: ComputedVisibility,

    // identity transform
    transform_bundle: TransformBundle,

    // real blob information
    info: BlobInfo,
}

impl Default for BlobBundle {
    fn default() -> Self {
        Self {
            blob_flag: Blob,
            visibility: Visibility::Visible,
            computed_visibility: ComputedVisibility::HIDDEN,
            transform_bundle: TransformBundle::IDENTITY,
            info: BlobInfo::default(),
        }
    }
}
