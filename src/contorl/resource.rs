use bevy::prelude::*;

use crate::{blob::geno_blob_builder::BlobGeno, brain::neuron::GenericNN};

#[derive(Resource)]
pub struct TrainMutPipe {
    genovec: Vec<BlobGeno>,
    nnvec: Vec<GenericNN>,
}

impl Default for TrainMutPipe {
    fn default() -> Self {
        Self {
            genovec: Vec::<BlobGeno>::new(),
            nnvec: Vec::<GenericNN>::new(),
        }
    }
}

impl TrainMutPipe {
    pub fn push(&mut self, genovec: Vec<BlobGeno>, nnvec: Vec<GenericNN>) {
        assert!(self.genovec.is_empty());
        assert!(self.nnvec.is_empty());
        self.genovec = genovec;
        self.nnvec = nnvec;
    }

    pub fn pop(&mut self) -> (Vec<BlobGeno>, Vec<GenericNN>) {
        assert!(!self.genovec.is_empty());
        assert!(!self.nnvec.is_empty());
        let res: (Vec<BlobGeno>, Vec<GenericNN>) = (self.genovec.clone(), self.nnvec.clone());
        self.genovec.clear();
        self.nnvec.clear();
        res
    }
}
