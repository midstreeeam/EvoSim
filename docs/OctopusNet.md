# Octopus Net

Octopus Net is a neural architecture, crafted specifically for our blobs, ensures adaptability without compromising integrity. 
## Granular & Hierarchical Structure:

-  **Peripheral Neural Units (PNU):** Associated with every block of the blob, these are the smallest computational units. Each PNU processes sensory input unique to its respective block and issues commands to its specific joint motor. This allows for autonomous decision-making at a micro level.
-  **Central Brain (CB):** Positioned in the root block, the CB operates as a coordinator. It receives sensory information from all blocks, processes it, and emits overarching commands. These signals can modify or influence behaviors across all blocks, ensuring coordination.

Implementation documents [here](https://evodoc.kaiyuanlou.com/evosim/brain/neuron/index.html).

## Signal Transmission & Behavior Modulation:

- **Bottom-up Transmission (Inward Propagation):** PNUs send specific sensory information hierarchially to the CB. This aids in holistic decision-making by the CB, considering inputs from all over the blob's body.
- **Top-down Modulation (Outward Propagation):** The CB issues general commands to it children blocks, and hierarchially passes to every block. While a block's PNU makes independent decisions, these decisions are modulated based on its parents' overarching signals, ensuring harmony in the blob's behavior.

The following graph shows the general information flow in both Inward Propagation and Outward Propagation on a single blob.
![[propagation.png]]
See detailed explanation in **Inward & Outward Propagation** section.

## Advantages:

- **Decentralized Processing:** Like an octopus, where each tentacle can act semi-independently but is influenced by a central brain, the blob benefits from a blend of localized decision-making and coordinated control.
- **Adaptability:** If a block or limb gets mutated or is non-functional, its corresponding PNU gets impacted, but the overall neural architecture remains resilient. The CB can adjust its strategies based on inputs from functioning PNUs.
- **Evolutionary Fitness:** In the world of virtual evolution, Octopus Net's modular design ensures that mutations affecting one part don't lead to the complete breakdown of the entire neural system, offering better survival and adaptability chances.

## Flat vs. Hierarchical: A Comparative Discussion

When we ponder the design of Neural Processing Units (NPU) in our blobs, two structural possibilities emerge: maintaining a hierarchical tree structure or adopting a flattened approach where all NPUs directly interface with the Central Brain (CB). Both designs have their own merits and challenges:

### Hierarchical

Strengths:

- **Mutation Resilience:** This design showcases superior resilience to structural mutations. Whether a limb is gained or lost, the CB remains unaffected in terms of its structural integrity and functioning.
- **Harmonized Decision-Making:** The hierarchical structure supports synchronized decision-making. Each node emits general commands to its immediate descendants. As a result, the CB can release broad directives without being entangled in the intricacies of specific block levels.

Challenges:

- **Signal Distortion:** As signals traverse a labyrinth of nodes before reaching the CB, they can get distorted. This limits the CB's ability to accurately gauge stimuli from distal blocks, potentially making the blob less responsive to subtle, yet vital cues.
- **Processing Complexities:** Given the intertwined dependencies within NPUs, some function almost as mini-brains, reliant on outputs from other NPUs. This can complicate parallel processing efforts, delaying responses.

### Flat

Strengths:

- **Simpler Parallel Processing:** Absence of internal NPU dependencies enables simultaneous processing of all NPUs. This can lead to faster reactions and adaptability.
- **Clearer Signal Transmission:** Direct connections to the CB eliminate the long circuitous paths of the hierarchical setup. This means that the CB receives more pristine, less-distorted signals, enhancing its sensitivity to nuanced stimuli.

Challenges:

- **Lack of Gradation:** While signals are clearer, there's a potential loss in the depth and gradation of information. Everything is relayed directly to the CB, which could overwhelm it with a barrage of data.
- **Inconstant CB structure:** Due to the direct linkage, the BC's shape and size changes while morphyological mutation happens, which might leads to a wide disable of blocks.

### Conclusion & Our Choice

In conclusion, the Octopus Net offers a harmonious blend of decentralization and coordination, drawing inspiration from nature's evolutionary marvels and tailoring it to the unique requirements of our virtual blobs.

While in our situation, we finally choose to use the hierarchical structure since we have the higher prority of mutation resilience for training. Keeping a relatively stable CB structure is essencial to our project regards of other benefits provided by flat design.

## Inward & Outward Propagation

Given our selection of a hierarchical neural network model, all illustrations within this section pertain to the hierarchical architecture.

As previously highlighted, each Peripheral Neural Unit (PNU) is adept at both inward and outward propagation—capabilities beyond the scope of a singular neural network. Consequently, every PNU, representing the smallest independent neural network unit, houses two distinct networks: **Inward NN** and **Outward NN**. Refer to the [implementation details](https://evodoc.kaiyuanlou.com/evosim/brain/neuron/struct.BlockNN.html).

The Inward NN receives input from two sources: signals gathered from blocks and joints, and the outputs from the child nodes' Inward NNs. This network generates a singular output, a synthesized signal derived from all its inputs. The output's dimensionality is notably smaller than its input, as only crucial information is relayed to parent nodes. Should a block be equipped to process specific data independently, the Outward NN decides on the course of action without escalating every detail to higher-tier nodes or the Central Brain (CB).

Conversely, the Outward NN accepts input from its block and joint and amalgamates it with directives descending from its parent nodes. It produces a dual-faceted output: one segment directs the joint motor—serving as the blob's muscle—while the other conveys commands to its child nodes.
![[info_transmission.png]]
The graph delineates the intricate pathways of information transfer from one block to another. It's essential to note that, for clarity in the illustration, each block is depicted with a single child. However, this simplification doesn't typically reflect reality. In actual simulations, a block can have up to three child blocks. Consequently, the PNUs both receive and relay information to all its associated child blocks.
