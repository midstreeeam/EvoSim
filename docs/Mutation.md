# Mutation

For an AI project that utulizing genetic algorithm for training, the mutation strategy can never be more important. Since the mutation is a process on top of virtual creatures' morphyology and neural network, I highly recommend you to read the [Gene](Gene.md) and [Neural](Neural.md) section before diving into the following sections.

Refering [this document](https://evodoc.kaiyuanlou.com/evosim/mutate/index.html) for detailed implementation.
## NN Mutation

This network must possess mutable characteristics to facilitate the exploration of novel solutions. Nevertheless, caution must be exercised to ensure that mutations aren't overly aggressive, as excessive mutation may jeopardize inherent features vital for evolutionary progress.

Given the intricate and variable nature of a blob's neural network, mutation efforts are confined to the most granular units of the network. The procedure for mutation involves generating random values from a normal distribution with a mean of $0$, which are then added to the respective weights and biases of the neural network.

To mirror the unpredictability of mutations in the natural world, certain constants—`MUTATE_NN_PORB`, `MUTATE_NN_WEIGHT_PROB`, and `MUTATE_NN_BIAS_PROB`—govern the likelihood of mutations occurring at the neural network, weight, or bias levels, respectively.

## Morphyology Mutation

While neural networks follow a unified mutation approach, the mutation of morphology is decidedly more intricate. A blob's physical structure is made up of individual blocks, each of which can undergo size mutations. The joints, pivotal for connecting these blocks, can experience alterations in their movement limits. Additionally, during the mutation process, blobs have the potential to either gain or lose blocks.

- **Gain & Lose Limbs:** Blobs can either gain or lose up to one block during mutation. This dynamic allows blobs to experiment with varied bodily configurations.
- **Joint Mutations:** Blocks are interconnected via joints. During mutation, the range of joint movement can change, but it's always restricted to lie between 0 and 360 degrees.
- **Block Size Mutation:** Blocks can vary in size, provided the alterations remain within a predefined range. There's a caveat: size changes must not lead to internal structural conflicts. To avoid this, a maximum number of retry attempts is established. Another tricky problem for blocks' size mutation is that it affects the position of all connected child blocks. The direction in which a mutated block attaches to its parent is significant. If mutated, children blocks in one particular direction will shift twice as much as those in the other two directions. This cascading movement can, in turn, introduce further potential for structural conflicts. The implementation of solving those questions are partially located in struct [BlobGeno](https://evodoc.kaiyuanlou.com/evosim/blob/geno_blob_builder/struct.BlobGeno.html), since lots of mutation are directly modifing the blob's gene.

Here is an example:
![[mutation.gif]]
## Synchronization

Ensuring synchronization between the neural network (NN) and the blobs, as well as the blocks within these blobs, is paramount. Given that blobs can gain or lose limbs during mutation, it's necessary to generate new neural networks during this process and subsequently remove the outdated ones. Additionally, as neurons are associated with blocks based on their indices, special measures must be put in place. This prevents disruptions to the indexing caused by the addition or removal of neural networks.

The intricacies of synchronization are encapsulated within the [sync_mutate](https://evodoc.kaiyuanlou.com/evosim/mutate/mutate/fn.sync_mutate.html) function. While this function is succinct, it houses complex operations.