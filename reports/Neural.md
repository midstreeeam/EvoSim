# Neural Network

Virtual creatures, also referred to as blobs, are endowed with significant morphological flexibility (as elaborated in the [Gene](Gene.md) section). To bestow these entities with the capability to manipulate their bodies and sense their surroundings, they are equipped with a neural system. This intricate neural network granting blobs both sensory and motor capabilities. You can find the implementation and documentation [here](https://evosim.kaiyuanlou.com/evosim/brain/index.html).
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

See detailed explaination of Octpous Net [here](OctopusNet.md).

## Neural Signals

In order to sense the surroundings, the creatures need to 



