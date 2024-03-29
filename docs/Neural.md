# Neural Network

Virtual creatures, also referred to as blobs, are endowed with significant morphological flexibility (as elaborated in the [Gene](Gene.md) section). To bestow these entities with the capability to manipulate their bodies and sense their surroundings, they are equipped with a neural system. This intricate neural network granting blobs both sensory and motor capabilities. You can find the implementation and documentation [here](https://evodoc.kaiyuanlou.com/evosim/brain/index.html).
## Structural Design

The design of a neural network, particularly for controlling our adaptable and evolving virtual blobs, hinges on two pivotal characteristics: structural flexibility and inheritability. These twin pillars shape our entire approach, distinguishing our system from more traditional neural network structures.
### Structural Flexibility

Blobs vary in size and function, and as a result, they differ in their sensory and motor requirements. For instance, a simpler blob might have just two blocks and one joint, while a more complex one might possess numerous blocks and joints. Hence, the neural network must be flexible, scaling according to the blob's individual requirements.

The architecture must accommodate varying input lengths (sensors) and provide outputs (motors) of differing lengths accordingly. This dynamic architecture is pivotal for blobs to adequately interact with their environment regardless of their size.

Leveraging structures like RNN, LSTM, or Transformer can achieve such flexibility. These architectures are adept at managing sequential data of variable lengths.
### Inheritability

Given that our system is driven by genetic algorithms, the neural network must be robust against genetic mutations since the neural network will mutate in both structure and weight. In particular:

- **Resilience to Mutation:** The neural system should function in a modular fashion, where a change or mutation in one section doesn't drastically impact the overall behavior. This modularity ensures that if a blob undergoes a mutation, leading to a loss of a limb, the rest of its body can still function appropriately.
- **Interdependency Management:** While connections and dependencies between nodes in a network are typical, excessive interdependency can be detrimental in our case. If one part of the network gets mutated, and if there's a high degree of interdependence, the entire network might behave erratically. Thus, the architecture should strike a balance, ensuring nodes operate somewhat independently while still maintaining network cohesion.

See detailed information about mutation stratigies [here](Mutation.md).

### Octopus Net

Emulating the decentralized intelligence seen in real-world cephalopods like octopuses, the "Octopus Net" seeks to solve the dual challenges of inheritability and structural flexibility. This neural architecture, crafted specifically for our blobs, ensures adaptability without compromising integrity. 

The architecture comprises a **Central Brain (CB)** positioned at the root block, paired with numerous **Peripheral Neural Units (PNU)** present in all other blocks. The functional dynamics of the Octopus Net are categorized into **Inward Propagation** and **Outward Propagation** phases. 

See detailed explaination of Octpous Net [here](OctopusNet.md).

## Neural Signals

To interact with and respond to their environment, blobs are equipped with sensors and muscles. The data generated by active sensors and the signals that can regulate muscles are termed as neural signals. Within our simulation, three distinct neural signals have been implemented: `BrainSignal`, `InwardNNInputSignal`, and `OutwardNNInputSignal`. More details can be found in the [documentation](https://evodoc.kaiyuanlou.com/evosim/brain/signal/index.html).

In the neural architecture of our blobs, muscles are operational during the Outward Propagation process, while sensors play a role in both inward and outward propagations.

Signals for Inward Propagation (`InwardNNInputSignal`):

- **Collision Type:** Information about the type of collision is collected when a block interacts with any rigid body other than its immediate parent or offspring. This could involve collisions with walls, other blobs, or even its own separate limbs.
- **Collision Vector:** For any collision involving a block, its PNU captures data describing the collision's direction and force.
- **Collision Magnitude:** Even though the neural network can derive the intensity of a collision based on its vector, a single float value representing the collision's magnitude is provided to ensure quicker and more efficient information processing.
- **Current Joint Motor's Data:** The joint motor, acting as the blob's muscle, relays information that includes its target position and velocity.
- **Joint Information:** The blob is also sensitive to its joint's metrics, such as its current angular velocity and position.
- **Block's Location (under implementation):** PNUs may require the geographical positioning of their respective blocks.
- **Children's Outputs:** When a block has children blocks, the synthesis of its own data with its progenies' is crucial. Therefore, a child's output is a necessary input for the current block, ensuring that the Central Brain (CB) receives comprehensive data.

For the **Outward Propagation** via the PNU's OutwardNN, the input signals remain identicle with those of the InwardNN, except that the outputs from children are replaced with those from the parent. This is because the outward propagation follows a top-down approach.

Additionally, the OutwardNN of the PNU not only relays general commands meant for the child nodes but also outputs signals governing the joint motor. Each OutwardNN solely produces two control signals: the joint motor's target position and its target velocity. These two outputs determine the joint motor's force and direction.

Signals for **Central Brain** (CB), or `BrainSignal`:

- **Collision Data:** Positioned at the blob's root block, the CB is susceptible to collisions. Therefore, it's vital for the CB to process collision-related inputs, encompassing aspects like collision type, vector, and magnitude, akin to the PNUs.
- **Blob Metrics:** Given the CB's role in issuing overarching directives, it's equipped to discern holistic blob details. Key metrics such as the blob's center of mass and current velocity fall under its purview.
- **Number Generator (under implementation):** Emulating real-world organisms, our virtual entities will incorporate a random number generator and oscillator. This feature allows them to make stochastic decisions and introduces an intrinsic rhythm, facilitating recurring movements.