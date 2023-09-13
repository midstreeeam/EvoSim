# Training

Due to the different tasks we want our virtual creatures to do, we can design different training strategies.

## Implementation

Distinct tasks can demand vastly different training approaches. In our design, all training tasks are modular, allowing for seamless transitions between them by merely modifying the configuration file.

To achieve this flexibility, we introduced the [BlobControlPlugin](https://evodoc.kaiyuanlou.com/evosim/control/control/struct.BlobControlPlugin.html). This essential component orchestrates all aspects of the application, encompassing resources, systems, and the sequence of function executions (determining which functions execute first, and which ones can run concurrently).

Below is a code snippet illustrating the varied control flows for training blobs in "walk" and "swim" scenarios:

```rust
#[cfg(feature = "move")]
fn build(&self, app: &mut App) {
    if TRAINING_MODE == "swim" {
        // train swim
        app.add_systems(Startup, move_setup)
            .add_systems(
                Update,
                (
                    update_iteration_frames.before(update_blob_info),
                    block_action,
                    update_blob_info,
                    update_joint_info,
                    update_crowding_distance,
                    log_train_move_swim.after(block_action),
                    train_move_swim.after(log_train_move_swim),
                    mutate_and_refresh_after_train.after(train_move_swim),
                ),
            )
            .init_resource::<TrainMutPipe>()
            .init_resource::<Frames>()
            .init_resource::<TED>();
    } else if TRAINING_MODE == "walk" {
        // train walk
        app.add_systems(Startup, move_setup)
            .add_systems(
                Update,
                (
                    update_iteration_frames.before(update_blob_info),
                    block_action,
                    update_blob_info,
                    update_joint_info,
                    update_crowding_distance,
                    log_train_move_walk.after(block_action),
                    train_move_walk.after(log_train_move_walk),
                    mutate_and_refresh_after_train.after(train_move_walk),
                ),
            )
            .init_resource::<TrainMutPipe>()
            .init_resource::<Frames>()
            .init_resource::<TED>();
    } else {
        panic!()
    }
}
```


## Tournament

As with many genetic algorithms, our training process incorporates tournament selection. The pivotal question is: how do we execute this? And how do we determine if one blob is superior to another?

In the tournament selection process, we combine two metrics for decision-making: a custom metric and the crowding distance.

### Costume measurement

The custom metric is tailored based on the specific objective of the training. For instance, in our experiment where blobs were trained to navigate underwater, the cumulative distance traveled served as the primary metric. This encouraged the blobs to cover greater distances.

Depending on the task at hand, this custom metric can be redefined to align with the desired outcomes.

### Crowding distance

Crowding distance gauges the similarity of an individual in relation to the broader population. A larger crowding distance indicates that the individual is markedly different from the majority. Applying crowding distance as a measurement can ensure the diversity of the population, which can prevent the solution from falling into a local solution.

In our framework, the crowding distance for a blob is determined using the tree edit distance (TED) for its gene since the blobs gene are represented by a QuadTree.

Function document can be found [here](https://evodoc.kaiyuanlou.com/evosim/blob/geno_blob_builder/struct.QuadTree.html#method.tree_edit_distance). It is a dynamic programming approach.

### Selection process

Unlike widely-used genetic algorithms such as NSGA-II or NSGA-III, which employ crowding distance to maintain population diversity, our approach places greater emphasis on crowding distance, as we do not implement non-dominance selection.

At the conclusion of each epoch, two tournaments (the custom metric tournament and the crowding distance tournament) operate concurrently, producing two sets of survivors. The final set of survivors is derived from a random selection among these candidates. Individuals who rank higher in each tournament have an increased likelihood of being chosen. Notably, if an individual emerges as a survivor in both tournaments, its chances of selection in the final round are significantly amplified.

The constant `HYBRID_RATE` determines the balance between the custom metric tournament and the crowding distance tournament. This flexibility empowers users with more control over the training process.