use std::fs::File;
use std::io::Write;
use std::{fs, path::Path};

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{Local, NaiveDateTime, Datelike, Timelike};

use crate::blob::blob::BlobInfo;
use crate::{
    blob::{block::NeuronId, geno_blob_builder::BlobGeno},
    brain::{resource::BevyBlockNeurons, neuron::GenericNN},
    consts::EXPORT_PATH,
};

#[derive(Serialize,Deserialize)]
pub struct ExportFile{
    genovec: Vec<BlobGeno>,
    nnvec: Vec<Vec<(GenericNN,usize)>>,
    posvec: Vec<[f32;2]>
}

impl ExportFile {
    fn new() -> Self {
        Self{
            genovec: Vec::<BlobGeno>::new(),
            nnvec: Vec::<Vec<(GenericNN,usize)>>::new(),
            posvec: Vec::<[f32;2]>::new()
        }
    }

    pub fn push_blob(&mut self, blob: (&BlobGeno,&BlobInfo)){
        self.genovec.push(blob.0.clone());
        self.posvec.push(blob.1.center_block_pos.into());
    }

    pub fn push_nn(&mut self, nnvec: Vec<(GenericNN,usize)>){
        self.nnvec.push(nnvec)
    }

    pub fn save(&self){
        assert_eq!(self.genovec.len(),self.nnvec.len());
        assert_eq!(self.genovec.len(),self.posvec.len());
        let file_str = serde_json::to_string(&self).unwrap();
        let fname = format!("{}{}",EXPORT_PATH,current_time_filename());
        let mut file = File::create(&fname).expect("Unable to create file");
        file.write_all(file_str.as_bytes()).expect("Unable to write data");
        info!("MODEL SAVED {}", &fname);
    }

    pub fn len(&self) -> usize{
        assert_eq!(self.genovec.len(),self.nnvec.len());
        self.genovec.len()
    }

    pub fn iter(&self) -> ExportFileIter {
        ExportFileIter {
            geno_iter: self.genovec.iter(),
            nn_iter: self.nnvec.iter(),
            pos_iter: self.posvec.iter()
        }
    }

    pub fn iter_mut(&mut self) -> ExportFileIterMut {
        ExportFileIterMut { 
            geno_iter: self.genovec.iter_mut(), 
            nn_iter: self.nnvec.iter_mut(), 
            pos_iter: self.posvec.iter_mut()
        }
    }

    pub fn check(&self) {
        assert_eq!(self.genovec.len(),self.nnvec.len());
        assert_eq!(self.genovec.len(),self.posvec.len());
    }

    /// Flattening and sorting by usize index, return cloned nnvec
    pub fn flatten_nnvec(&self) -> Vec<GenericNN>{
        let mut flattened_tuples: Vec<(GenericNN, usize)> = self.nnvec.clone().into_iter().flatten().collect();
        flattened_tuples.sort_by_key(|&(_, index)| index);
        flattened_tuples.into_iter().map(|(nn, _)| nn).collect()
    }
}

pub struct ExportFileIter<'a> {
    geno_iter: std::slice::Iter<'a, BlobGeno>,
    nn_iter: std::slice::Iter<'a, Vec<(GenericNN,usize)>>,
    pos_iter: std::slice::Iter<'a, [f32;2]>,
}

impl<'a> Iterator for ExportFileIter<'a> {
    type Item = (&'a BlobGeno, &'a [f32;2], &'a Vec<(GenericNN,usize)>);

    fn next(&mut self) -> Option<Self::Item> {
        let geno = self.geno_iter.next()?;
        let nn = self.nn_iter.next()?;
        let pos = self.pos_iter.next()?;
        Some((geno, pos, nn))
    }
}

pub struct ExportFileIterMut<'a> {
    geno_iter: std::slice::IterMut<'a, BlobGeno>,
    nn_iter: std::slice::IterMut<'a, Vec<(GenericNN,usize)>>,
    pos_iter: std::slice::IterMut<'a, [f32;2]>,
}

impl<'a> Iterator for ExportFileIterMut<'a> {
    type Item = (&'a mut BlobGeno, &'a mut [f32;2], &'a mut Vec<(GenericNN,usize)>);

    fn next(&mut self) -> Option<Self::Item> {
        let geno = self.geno_iter.next()?;
        let nn = self.nn_iter.next()?;
        let pos = self.pos_iter.next()?;
        Some((geno, pos, nn))
    }
}

pub fn export(
    input: Res<Input<KeyCode>>,
    blob_q: Query<(Entity, (&BlobGeno, &BlobInfo))>,
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

        for (blob_id, blob) in blob_q.iter(){
            ef.push_blob(blob);
            let mut blob_nn = Vec::<(GenericNN,usize)>::new();
            for (parent_id, neuron) in nn_q.iter(){
                if parent_id.get() != blob_id {
                    continue;
                }
                // unwrap since neuron mush in nnvec
                blob_nn.push((nnvec.get(neuron.id).unwrap().clone(), neuron.id))
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