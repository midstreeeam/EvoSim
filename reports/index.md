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

Dive deeper into the intricacies of our physical simulation design and its implementation by visiting [this dedicated page](Physical.md).
### Morphylogoly

Creatures are architecturally composed of rectangular rigid bodies interconnected by joints. These rigid bodies serve as the physical elements facilitating movement, while the joints, constrained by specific angles and equipped with controllable motors, enable articulated action.

In this morphology framework, creatures harness rectangles of varying dimensions as fundamental building blocks, shaping their forms and influencing their behaviors. Each being is birthed with a distinct genetic code encompassing its entire morphological blueprint. Mirroring nature, this gene is mutable and replicable.

For a deeper understanding, peruse our detailed breakdown [here](Gene.md).
### Neural Network

Each creature possesses a unique neural network steering its every behavior. Given the varied morphologies of these beings, their neural networks are inherently dynamic, both in magnitude and design.

Echoing the complex neural architecture found in living organisms, our virtual entities' neural framework is bifurcated into the Central Nervous System (CNS) and the Peripheral Nervous System (PNS)—in simpler terms, the central brain and the peripheral neurons. The central brain, or CNS, remains distinct, while every limb or rigid body segment contains elements of the PNS, together weaving intricate neural pathways.

Our creatures' neural configurations deviate markedly from conventional neural network designs. Firstly, sensors embedded within body segments and joints amass data, transmitting it to the central brain—a process termed "Inward Propagation." Subsequently, the brain process these signals, dispatching output hierarchically throughout the body to trigger muscles (in our simulation, the joint motors)—called "Outward Propagation."

Owing to this unique neural makeup, existing frameworks fall short. As a solution, we've crafted our own from the ground up. Delve into the nuances of our neural structure [here](Neural.md).
### Training
