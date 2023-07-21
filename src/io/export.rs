use std::fs::File;
use std::io::Write;
use std::{fs, path::Path};

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{Local, NaiveDateTime, Datelike, Timelike};

use crate::{
    blob::{block::NeuronId, geno_blob_builder::BlobGeno},
    brain::{resource::BevyBlockNeurons, neuron::GenericNN},
    consts::EXPORT_PATH,
};

#[derive(Serialize,Deserialize)]
struct ExportFile{
    genovec: Vec<BlobGeno>,
    nnvec: Vec<Vec<GenericNN>>
}

impl ExportFile {
    fn new() -> Self {
        Self{
            genovec: Vec::<BlobGeno>::new(),
            nnvec: Vec::<Vec<GenericNN>>::new()
        }
    }

    pub fn push_geno(&mut self, geno: &BlobGeno){
        self.genovec.push(geno.clone())
    }

    pub fn push_nn(&mut self, nnvec: Vec<GenericNN>){
        self.nnvec.push(nnvec)
    }

    pub fn save(&self){
        assert_eq!(self.genovec.len(),self.nnvec.len());
        let file_str = serde_json::to_string(&self).unwrap();
        let fname = format!("{}{}",EXPORT_PATH,current_time_filename());
        let mut file = File::create(&fname).expect("Unable to create file");
        file.write_all(file_str.as_bytes()).expect("Unable to write data");
        info!("MODEL SAVED {}", &fname);
    }
}

pub fn export(
    input: Res<Input<KeyCode>>,
    blob_q: Query<(Entity, &BlobGeno)>,
    nn_q: Query<(&Parent, &NeuronId)>,
    bbn: Res<BevyBlockNeurons>,
) {
    if blob_q.is_empty() || nn_q.is_empty() {
        return;
    }

    if input.just_pressed(KeyCode::S) {
        create_if_not_exist();
        let mut ef = ExportFile::new();
        let nnvec = &bbn.nnvec;

        for (blob_id, geno) in blob_q.iter(){
            ef.push_geno(geno);
            let mut blob_nn = Vec::<GenericNN>::new();
            for (parent_id, neuron) in nn_q.iter(){
                if parent_id.get() != blob_id {
                    continue;
                }
                // unwrap since neuron mush in nnvec
                blob_nn.push(nnvec.get(neuron.id).unwrap().clone())
            }
            ef.push_nn(blob_nn);
        }
        ef.save();
    }
}

fn create_if_not_exist() {
    let path = EXPORT_PATH;

    // Check if the path exists
    if !Path::new(path).exists() {
        // Create the directory if it doesn't exist
        match fs::create_dir_all(path) {
            Ok(_) => println!("Directory created {}.", path),
            Err(e) => eprintln!("Error creating directory: {}", e),
        }
    }
}

fn current_time_filename() -> String {
    let now: NaiveDateTime = Local::now().naive_local();
    format!("{:04}-{:02}-{:02}T{:02}-{:02}-{:02}.json",
            now.year(), now.month(), now.day(),
            now.hour(), now.minute(), now.second())
}