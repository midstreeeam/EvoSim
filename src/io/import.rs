use std::fs::File;
use std::io::Read;

use bevy::prelude::*;
use serde_json;

use crate::consts::LOAD_FNAME;

use super::export::ExportFile;

pub fn load_blobs(

){
    if let Ok(mut file) = File::open(LOAD_FNAME){
        let mut file_str = String::new();
        file.read_to_string(&mut file_str).unwrap();
        let ef: ExportFile = serde_json::from_str(&file_str).unwrap();
    } else {
        warn!("Fail to open file {}", LOAD_FNAME)
    }
}