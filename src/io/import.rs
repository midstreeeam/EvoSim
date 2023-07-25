use std::fs::File;
use std::io::Read;

use bevy::prelude::*;
use bevy_rapier2d::prelude::ImpulseJoint;
use serde_json;

use crate::blob::blob::Blob;
use crate::blob::geno_blob_builder::GenoBlobBuilder;
use crate::brain::resource::BevyBlockNeurons;
use crate::componet::ColliderFlag;
use crate::consts::LOAD_FNAME;
use crate::physics::world::Wall;

use super::export::ExportFile;

pub fn load_blobs(
    commands: Commands,
    mut bbn: ResMut<BevyBlockNeurons>,
    input: Res<Input<KeyCode>>,
) {
    // if input.just_pressed(KeyCode::L){
    //     if let Ok(mut file) = File::open(LOAD_FNAME){
    //         let mut file_str = String::new();
    //         file.read_to_string(&mut file_str).unwrap();
    //         let ef: ExportFile = serde_json::from_str(&file_str).unwrap();
    //         ef.check();
    //         overwrite(ef, commands, &mut bbn);
    //     } else {
    //         warn!("Fail to open file {}", LOAD_FNAME)
    //     }
    // }

    if input.just_pressed(KeyCode::L) {
        match File::open(LOAD_FNAME) {
            Ok(mut file) => {
                let mut file_str = String::new();

                // Handle read_to_string error
                if let Err(e) = file.read_to_string(&mut file_str) {
                    warn!("Failed to read from file {}: {:?}", LOAD_FNAME, e);
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
    if input.just_pressed(KeyCode::L) || input.just_pressed(KeyCode::X) {
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
