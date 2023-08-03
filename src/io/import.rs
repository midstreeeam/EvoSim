use std::fs::{File, self};
use std::io::Read;

use bevy::prelude::*;
use bevy_rapier2d::prelude::ImpulseJoint;
use serde_json;

use crate::blob::blob::Blob;
use crate::blob::geno_blob_builder::GenoBlobBuilder;
use crate::brain::resource::BevyBlockNeurons;
use crate::componet::ColliderFlag;
use crate::consts::*;
use crate::physics::world::Wall;

use super::export::ExportFile;

pub fn load_blobs(
    commands: Commands,
    mut bbn: ResMut<BevyBlockNeurons>,
    input: Res<Input<KeyCode>>,
) {
    let mut load_fname = LOAD_FNAME.to_string();
    if LOAD_NEWEST_FILE {
        let path = newest_file_name_in_directory(LOAD_FOLDER);
        if let Some(path) = path {
            load_fname = LOAD_FOLDER.to_string() + &path;
        } else {
            panic!("empty load folder")
        }
    }
    
    if input.just_pressed(LOAD_ALL_BLOBS_FROM_JSON) {
        match File::open(&load_fname) {
            Ok(mut file) => {
                let mut file_str = String::new();

                // Handle read_to_string error
                if let Err(e) = file.read_to_string(&mut file_str) {
                    warn!("Failed to read from file {}: {:?}", load_fname, e);
                    return;
                }

                // Handle serde_json parsing error
                match serde_json::from_str::<ExportFile>(&file_str) {
                    Ok(ef) => {
                        ef.check();
                        overwrite(ef, commands, &mut bbn);
                    }
                    Err(e) => {
                        warn!("Failed to parse the file content as `ExportFile`: {:?}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to open file {}: {:?}", LOAD_FNAME, e);
            }
        }
    }
}

pub fn clean(
    mut commands: Commands,
    mut bbn: ResMut<BevyBlockNeurons>,
    blob_q: Query<Entity, With<Blob>>,
    collider_q: Query<Entity, (With<ColliderFlag>, Without<Wall>)>,
    joint_q: Query<Entity, With<ImpulseJoint>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(LOAD_ALL_BLOBS_FROM_JSON) || input.just_pressed(CLEAN_ALL_BLOBS_KEYCODE) {
        for entity in blob_q.iter().chain(collider_q.iter()).chain(joint_q.iter()) {
            commands.entity(entity).despawn()
        }
        bbn.clear();
    }
}

/// ignore and overwrite all blobs and NNs that exist
/// despawn all the entities except wall
fn overwrite(mut ef: ExportFile, commands: Commands, bbn: &mut BevyBlockNeurons) {
    let mut builder = GenoBlobBuilder::from_commands(commands, &mut bbn.nnvec);

    // build loaded blobs
    for (geno, pos, _nnvec) in ef.iter_mut() {
        // println!("{:#?}",geno);
        builder.build(geno, *pos);
        // println!("\n{:#?}",geno);
    }

    // set resource
    bbn.nnvec = ef.flatten_nnvec();
}

fn newest_file_name_in_directory(dir: &str) -> Option<String> {
    fs::read_dir(dir)
        .ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().extension()? == "json" {
                entry.path().file_name()?.to_str().map(String::from)
            } else {
                None
            }
        })
        .max()
}