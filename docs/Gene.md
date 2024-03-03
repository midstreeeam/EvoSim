# Genes & Morphology

For our virtual creatures, affectionately termed "blobs", the goal is to encourage a diverse range of forms. We've built in a significant degree of freedom, allowing them to evolve into any shape or structure, as long as they adhere to certain guidelines:

- **Block Unit:** At its core, every blob is composed of a minimum of two blocks. These blocks, shaped like rectangles, represent the basic rigid-body units within our simulation. Depending on the creature's structure, a block can function as a bone, limb, or any other body part. Detailed documentation can be found [here](https://evodoc.kaiyuanlou.com/evosim/blob/block/index.html).
- **Neuron:** Each block unit has its own neural network, the id of NN will be stored in the blob's gene. Detailed explaination of NN is [here](Neural.md).
- **Joint & Anchor Points:** Every block is designed with four anchor points situated at the center of each of its edges. Each of these anchor points can connect to, at most, one other block via a joint.
- **Hierarchy:** The structure of a blob is analogous to a tree. Every block (or "node" in tree parlance) can have up to one parent and up to three children. The tree's depth - that is, the number of generational layers it can have - is configurable.
- **Validation:** For a blob's morphology to be considered valid, it should be possible to depict it in a 2D space without any overlapping components.

This framework ensures that while our blobs have the freedom to evolve and diversify, they remain within a coherent and manageable system.

## Representation of Gene

Given that a blob's structure resembles a tree, we've chosen the **QuadTree** as our data structure for representing a blob's gene. Interestingly, while each block can potentially connect to four other blocks, due to the restriction that one anchor point always links back to its parent, the resultant tree structure is a **ternary tree**.

Yet, we opt for a quad tree over a ternary tree. The reason lies in our need to preserve not only the hierarchical information of each block (i.e., identifying its parent) but also its positional details. Specifically, we need to know which anchor point of the block is linked to its parent. This nuanced representation allows us to identify block locations via indices.

Within our tree design:

- Indexes 1, 2, 3, and 4 correspond to the anchor points: up, down, left, and right, respectively.
- If no children are connected to an anchor point, its value is set to `None`.
- If an anchor is connected to valid block, its value is an enum [GenericGenoNode](https://evodoc.kaiyuanlou.com/evosim/blob/geno_blob_builder/enum.GenericGenoNode.html), which can be either `Parent` or `Child`, where parent is an indicator and child is a [GenoNode](https://evodoc.kaiyuanlou.com/evosim/blob/geno_blob_builder/enum.GenericGenoNode.html) that contains block's information.
![[quadtree.png]]
As shown the in the graph, the left-hand side QuadTree represents the right-hand side blob structure.

For a deeper dive into the implementation specifics, refer to our documentation on [BlobGeno](https://evodoc.kaiyuanlou.com/evosim/blob/geno_blob_builder/struct.BlobGeno.html) and [QuadTree](https://evodoc.kaiyuanlou.com/evosim/blob/geno_blob_builder/struct.QuadTree.html).

## Gene Validation

In our structure, every blob's morphology can be represented by a unique gene, but not every possible gene relate to a valid structure. So, when a gene is randomly generated, or mutated, we need to check if the new gene corresponds to a valid morphology structure.

Usually, there are two types of invalidate gene, as shown in the graph below:
![[validation.jpg]]

The first type of invalidation is easy to prevent if we simply limit the size of child blocks, but the second one is hard to prevent by limiting the generation rule. So, instead of modifying the genration rule, we just check the validation each time there are mophyological changes.

You can find checker function [here](https://evodoc.kaiyuanlou.com/evosim/blob/geno_blob_builder/struct.BlobGeno.html#method.is_valid).