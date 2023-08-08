use bevy::prelude::*;

use crate::{
    blob::{blob::BlobInfo, geno_blob_builder::BlobGeno},
    brain::neuron::GenericNN,
};

/// count how many frames been passed since simulation start
#[derive(Resource)]
pub struct Frames(pub u128);

impl Default for Frames {
    fn default() -> Self {
        Self(0)
    }
}

// TODO: TED should be normalized by avg blob depth
/// The Tree Edit Distance, used as indicator for diversity
#[derive(Resource)]
pub struct TED(pub f32);

impl Default for TED {
    fn default() -> Self {
        Self(0.0)
    }
}

/// A temp storage for all blob and it's relate neuron's information.
/// 
/// After reproduction and mutation, all NN and Blobs will be cleaned,
/// new blobs and nn are generated after that.
/// 
/// When doing cleaning, this Resource stores 
/// all the blobs and neurons waiting to be spawned
#[derive(Resource)]
pub struct TrainMutPipe {
    genovec: Vec<BlobGeno>,
    infovec: Vec<BlobInfo>,
    nnvec: Vec<GenericNN>,
}

impl Default for TrainMutPipe {
    fn default() -> Self {
        Self {
            genovec: Vec::<BlobGeno>::new(),
            infovec: Vec::<BlobInfo>::new(),
            nnvec: Vec::<GenericNN>::new(),
        }
    }
}

impl TrainMutPipe {
    pub fn push(&mut self, genovec: Vec<BlobGeno>, infovec: Vec<BlobInfo>, nnvec: Vec<GenericNN>) {
        assert!(self.genovec.is_empty());
        assert!(self.infovec.is_empty());
        assert!(self.nnvec.is_empty());
        self.genovec = genovec;
        self.infovec = infovec;
        self.nnvec = nnvec;
        assert!(!self.genovec.is_empty());
        assert!(!self.infovec.is_empty());
        assert!(!self.nnvec.is_empty());
    }

    pub fn pop(&mut self) -> (Vec<BlobGeno>, Vec<BlobInfo>, Vec<GenericNN>) {
        assert!(!self.genovec.is_empty());
        assert!(!self.infovec.is_empty());
        assert!(!self.nnvec.is_empty());
        let res: (Vec<BlobGeno>, Vec<BlobInfo>, Vec<GenericNN>) = (
            self.genovec.clone(),
            self.infovec.clone(),
            self.nnvec.clone(),
        );
        self.genovec.clear();
        self.infovec.clear();
        self.nnvec.clear();
        res
    }

    pub fn is_empty(&self) -> bool {
        self.genovec.is_empty()
    }
}
