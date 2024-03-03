# Intro

Drawing inspiration from Karl Sims' pioneering work on [Evolved Virtual Creatures](https://www.karlsims.com/evolved-virtual-creatures.html), [EvoSim](https://github.com/midstreeeam/EvoSim) is a simulation project. It endeavors to craft a dynamic virtual ecosystem populated with virtual creatures.

In this simulated realm, these virtual beings engage with one another in a manner reminiscent of natural ecosystems. The creatures are endowed with significant adaptability, empowering them to evolve both their morphology and neural frameworks, leading to the emergence of diverse species.

## Environment

[EvoSim](https://github.com/midstreeeam/EvoSim) harnesses the capabilities of the [Bevy game engine](https://bevyengine.org/) for its core operations. To ensure realistic interactions within the environment, the project integrates the [Rapier](https://rapier.rs/) physics engine. However, to meet our specific demands for precision and efficiency in physical simulations, we have implemented our own tailored [fork](https://github.com/midstreeeam/rapier/tree/mid) of Rapier.

At its foundation, the entire project is meticulously crafted using the Rust programming language.

## Fundamental Architecture

The core architecture of the simulation is segmented into several pivotal components: physical simulation, creature morphology, neural networks, and the training regimen.
### Physical Simulation

Moving away from the traditional evolution simulations that provide pre-defined movement functions for virtual entities, we aspire for our creatures to autonomously discover means of motion within a simulated physical realm, mirroring the evolutionary trajectory of cells in the real world. Thus, we've developed an underwater physics simulation, encouraging virtual entities to adapt and learn to swim.

Dive deeper into the intricacies of our physical simulation design and its implementation by visiting [this dedicated page](Physics.md).
### Morphologoly

Creatures are architecturally composed of rectangular rigid bodies interconnected by joints. These rigid bodies serve as the physical elements facilitating movement, while the joints, constrained by specific angles and equipped with controllable motors, enable articulated action.

In this morphology framework, creatures harness rectangles of varying dimensions as fundamental building blocks, shaping their forms and influencing their behaviors. Each being is birthed with a distinct genetic code encompassing its entire morphological blueprint. Mirroring nature, this gene is mutable and replicable.

For a deeper understanding, peruse our detailed breakdown [here](Gene.md).
### Neural Network

Each creature possesses a unique neural network steering its every behavior. Given the varied morphologies of these beings, their neural networks are inherently dynamic, both in magnitude and design.

Echoing the complex neural architecture found in living organisms, our virtual entities' neural framework is bifurcated into the Central Nervous System (CNS) and the Peripheral Nervous System (PNS)—in simpler terms, the central brain and the peripheral neurons. The central brain, or CNS, remains distinct, while every limb or rigid body segment contains elements of the PNS, together weaving intricate neural pathways.

Our creatures' neural configurations deviate markedly from conventional neural network designs. Firstly, sensors embedded within body segments and joints amass data, transmitting it to the central brain—a process termed "Inward Propagation." Subsequently, the brain process these signals, dispatching output hierarchically throughout the body to trigger muscles (in our simulation, the joint motors)—called "Outward Propagation."

Owing to this unique neural makeup, existing frameworks fall short. As a solution, we've crafted our own from the ground up. Delve into the nuances of our neural structure [here](Neural.md).
### Training

Evolutionary training stands at the heart of our project. Without this, our virtual entities would merely be random, inanimate blocks, and their intricate neural networks would lack purpose and function.

The adaptability of our training regimen is noteworthy. By designing a myriad of tasks, we can guide creatures towards various objectives—be it movement, terrestrial ambulation, aquatic swimming, or other specialized actions. To facilitate the evolution of these creatures, we employ genetic algorithms. Given the intricate nature of our dynamic neural networks, conventional back-propagation proves too mathematically challenging. Thus, genetic algorithms offer a more viable and efficient approach.

For a deeper dive into the nuances of our evolutionary training process, explore [training section](Training.md) and [mutation section](Mutation.md).

## Design Pattern

To harness the advantages of parallel computing, our design embraces the Entity Component System (ECS) paradigm over the traditional Object-Oriented Programming (OOP) approach.

Our virtual creatures boast a distinct hierarchical neural network. Organizing such a network in a conventional nested manner would immensely complicate parallel computation due to the intricate interdependencies present within the neural network's segments. To navigate this complexity, we've restructured the neural network. Each expansive network is fragmented into its smallest functional units, which are then interlinked. An auxiliary scheduling system is incorporated to dictate the sequence of processing. This reconfiguration significantly simplifies the parallel processing of the numerous neural network units.

For managing other resources and datasets, we lean into Bevy's ECS framework, which offers robust support for parallel operations.

Codebase structure shown below:

```console
crate evosim
├── mod blob: pub(crate)
│   ├── mod blob: pub
│   │   ├── struct Blob: pub
│   │   ├── struct BlobBundle: pub
│   │   └── struct BlobInfo: pub
│   ├── mod blob_builder: pub
│   │   ├── struct BlobBlock: pub
│   │   └── struct BlobBuilder: pub
│   ├── mod block: pub
│   │   ├── struct BlockAnchors: pub
│   │   ├── struct BlockDepth: pub
│   │   ├── struct CenterBlockFlag: pub
│   │   ├── struct JointInfo: pub
│   │   ├── struct NeuronId: pub
│   │   ├── struct ParentAnchor: pub
│   │   └── struct PhysiBlockBundle: pub
│   └── mod geno_blob_builder: pub
│       ├── struct BlobGeno: pub
│       ├── enum GenericGenoNode: pub
│       ├── struct GenoBlobBuilder: pub
│       ├── struct GenoNode: pub
│       ├── struct QuadTree: pub
│       └── mod builder_validation_test: pub(self) #[cfg(test)]
├── mod brain: pub(crate)
│   ├── mod neuron: pub
│   │   ├── struct BlockNN: pub
│   │   ├── struct BrainNN: pub
│   │   ├── enum GenericNN: pub
│   │   ├── struct InwardNN: pub
│   │   └── struct OutwardNN: pub
│   ├── mod nn: pub
│   │   ├── enum Activation: pub
│   │   ├── struct BaseLayer: pub
│   │   └── struct BaseNN: pub
│   ├── mod resource: pub
│   │   └── struct BevyBlockNeurons: pub
│   └── mod signal: pub
│       ├── struct BrainSignal: pub
│       ├── struct BrainSignalUnit: pub
│       ├── struct InwardNNInputSignal: pub
│       ├── struct InwardNNInputSignalUnit: pub
│       ├── struct OutwardNNInputSignal: pub
│       └── struct SignalHandler: pub
├── mod componet: pub(crate)
│   ├── struct BlobEntityIndex: pub
│   └── enum ColliderFlag: pub
├── mod consts: pub(crate)
│   └── mod mutate_consts: pub #[cfg(feature = "move")]
├── mod contorl: pub(crate)
│   ├── mod contorl: pub
│   │   └── struct BlobContorlPlugin: pub
│   ├── mod resource: pub
│   │   ├── struct Frames: pub
│   │   ├── struct TED: pub
│   │   └── struct TrainMutPipe: pub
│   ├── mod train_move: pub
│   └── mod update: pub
├── mod graphics: pub(crate)
│   ├── struct EvoGraphicsPlugin: pub
│   └── struct MainCamera: pub
├── mod io: pub(crate)
│   ├── mod evoio: pub
│   │   └── struct EvoIOPlugin: pub
│   ├── mod export: pub
│   │   ├── struct ExportFile: pub
│   │   ├── struct ExportFileIter: pub
│   │   └── struct ExportFileIterMut: pub
│   └── mod import: pub
├── mod logger: pub(crate)
│   └── mod tests: pub(self) #[cfg(test)]
├── mod mutate: pub(crate)
│   ├── mod geno_mutate: pub(self)
│   ├── mod mutate: pub
│   │   └── struct MutatePlugin: pub
│   └── mod nn_mutate: pub(self)
└── mod physics: pub(crate)
    ├── mod physical_world: pub
    │   └── struct PhysiWorldPlugin: pub
    ├── mod rules: pub
    └── mod world: pub
        └── struct Wall: pub
```
