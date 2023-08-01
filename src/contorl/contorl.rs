use bevy::prelude::*;

use super::update::{block_action, update_joint_info, update_blob_info};

pub struct BlobContorlPlugin;

impl Plugin for BlobContorlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            block_action,
            update_blob_info,
            update_joint_info
        ));
    }
}