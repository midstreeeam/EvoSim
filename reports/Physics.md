# Physical Simulation

To enable our virtual creatures to engage with their surroundings and other entities, we have implemented a robust physical simulation. This simulation allows collisions between objects, enabling creatures to exhibit behaviors like paddling, scratching, pushing, and pulling.

For foundational physical simulations, we have chosen the [Rapier](https://rapier.rs/) physics engine to manage collision events. However, we encountered challenges, especially with joint functionalities.

For full documentation, please visit document for [Module evosim::physics](https://evosim.kaiyuanlou.com/evosim/physics/index.html).

## Joints

Our [morphology design](Gene.md) accentuates the importance of joints in shaping the creatures' appearance and behavior. The joint capabilities offered by Rapier did not align entirely with our design requirements, particularly in:

- **Joint Limit:** Ideally, a joint should possess the flexibility to move within a defined range, analogous to how the human arm can't point in every conceivable direction. Yet, Rapier's joint limits only span from -90 to 90 degrees, restricting the possibility of a joint moving within a 180 to 360-degree range. For a comprehensive explanation, review this [issue](https://github.com/dimforge/rapier/issues/499).
- **Joint Motor:** Echoing the limitations of the joint limit, Rapier's joint motor is also confined to a 180-degree range. This doesn't suit our virtual creatures since joint motors act as their muscles, necessitating a broader range. Further details can be found in this [issue page](https://github.com/dimforge/rapier/issues/378).

To address these constraints without compromising the benefits of Rapier, we developed our own fork of the engine.

### Constraint and Constraint Solver

Physics engines like Rapier don't exclusively deal with collisions. For instance, the force experienced between two rigid bodies connected by a joint isn't a direct result of collision. In such scenarios, the joint serves as a constraint that delineates the permissible behaviors of the connected entities.

Rapier leverages the Jacobian matrix for evaluating and resolving these constraints. The constraint solver operates as follows:

- Predict the upcoming positions of objects based on existing velocities and forces.
- Assess how much the predictions violate the constraints.
- Adjust the object velocities using iterative relaxation to minimize constraint violations, with the Jacobian facilitating these corrections.

The rapier also use **Sequential Impulse** that iteratively applying impulses to objects to resolve constraint.

Our focus was to alter how Rapier interprets and manages joint limits and motors. To achieve this, we delved deep into Rapier's constraint builder and solver.

### Our Modification

Rapier is a finely-tuned physics engine, managing diverse constraints with dedicated solvers. As an illustrative example, let's consider our modifications to the `limit_angular` function. For a comprehensive view of all modifications, refer to our [github repo](https://github.com/midstreeeam/rapier).

The original `limit_angular` function accepts two physical units connected by a joint and their respective joint limits as inputs. It then **calculates the angular error, sets impulse bounds, constructs the angular Jacobian, computes the Right-Hand Side (RHS)**, derives the angular Jacobian for the connected bodies, and ultimately integrates the constraint into the `JointVelocityConstraint` structure.

```rust
pub fn limit_angular<const LANES: usize>(
        &self,
        params: &IntegrationParameters,
        joint_id: [JointIndex; LANES],
        body1: &SolverBody<N, LANES>,
        body2: &SolverBody<N, LANES>,
        limited_axis: usize,
        limits: [N; 2],
        writeback_id: WritebackId,
    ) -> JointVelocityConstraint<N, LANES> {
        let zero = N::zero();
        let half = N::splat(0.5);
        let s_limits = [(limits[0] * half).simd_sin(), (limits[1] * half).simd_sin()];
        #[cfg(feature = "dim2")]
        let s_ang = self.ang_err.im;
        #[cfg(feature = "dim3")]
        let s_ang = self.ang_err.imag()[limited_axis];
        let min_enabled = s_ang.simd_lt(s_limits[0]);
        let max_enabled = s_limits[1].simd_lt(s_ang);

        let impulse_bounds = [
            N::splat(-Real::INFINITY).select(min_enabled, zero),
            N::splat(Real::INFINITY).select(max_enabled, zero),
        ];

        #[cfg(feature = "dim2")]
        let ang_jac = self.ang_basis[limited_axis];
        #[cfg(feature = "dim3")]
        let ang_jac = self.ang_basis.column(limited_axis).into_owned();
        let dvel = ang_jac.gdot(body2.angvel) - ang_jac.gdot(body1.angvel);
        let rhs_wo_bias = dvel;

        let erp_inv_dt = N::splat(params.joint_erp_inv_dt());
        let cfm_coeff = N::splat(params.joint_cfm_coeff());
        let rhs_bias = ((s_ang - s_limits[1]).simd_max(zero)
            - (s_limits[0] - s_ang).simd_max(zero))
            * erp_inv_dt;

        let ang_jac1 = body1.sqrt_ii * ang_jac;
        let ang_jac2 = body2.sqrt_ii * ang_jac;

        JointVelocityConstraint {
            joint_id,
            mj_lambda1: body1.mj_lambda,
            mj_lambda2: body2.mj_lambda,
            im1: body1.im,
            im2: body2.im,
            impulse: N::zero(),
            impulse_bounds,
            lin_jac: na::zero(),
            ang_jac1,
            ang_jac2,
            inv_lhs: N::zero(), // Will be set during ortogonalization.
            cfm_coeff,
            cfm_gain: N::zero(),
            rhs: rhs_wo_bias + rhs_bias,
            rhs_wo_bias,
            writeback_id,
        }
    }
```

However, in the function, the limit `s_limits` is resized, which leads to the following calculation can only preform limitation between -90 degrees and 90 degrees.

Here is our modified version:
```rust
pub fn limit_angular<const LANES: usize>(
        &self,
        params: &IntegrationParameters,
        joint_id: [JointIndex; LANES],
        body1: &SolverBody<N, LANES>,
        body2: &SolverBody<N, LANES>,
        limited_axis: usize,
        limits: [N; 2],
        writeback_id: WritebackId,
    ) -> JointVelocityConstraint<N, LANES> {
        let zero = N::zero();
        let half = N::splat(0.5);
        let s_limits = [(limits[0] * half).simd_sin(), (limits[1] * half).simd_sin()];

        let s_ang_dist = self.ang_err.angle();
        let over_half_pi = s_ang_dist.simd_abs().simd_gt(N::simd_frac_pi_2());
        
        #[cfg(feature = "dim3")]
        let s_ang = self.ang_err.imag()[limited_axis];

        #[cfg(feature = "dim2")]
        let min_triggered = s_ang_dist.simd_lt(limits[0]);

        #[cfg(feature = "dim3")]
        let min_triggered = s_ang.simd_lt(s_limits[0]);

        #[cfg(feature = "dim2")]
        let max_triggered = limits[1].simd_lt(s_ang_dist);

        #[cfg(feature = "dim3")]
        let max_triggered = s_limits[1].simd_lt(s_ang);

        let i0_flag = (over_half_pi & max_triggered) | (!over_half_pi & min_triggered);
        let i1_flag = (over_half_pi & min_triggered) | (!over_half_pi & max_triggered);
        let impulse_bounds = [
            N::splat(-Real::INFINITY).select(i0_flag, zero),
            N::splat(Real::INFINITY).select(i1_flag, zero),
        ];

        #[cfg(feature = "dim2")]
        let ang_jac = self.ang_basis[limited_axis];

        #[cfg(feature = "dim3")]
        let ang_jac = self.ang_basis.column(limited_axis).into_owned();
        let dvel = ang_jac.gdot(body2.angvel) - ang_jac.gdot(body1.angvel);
        let rhs_wo_bias = dvel;

        let erp_inv_dt = N::splat(params.joint_erp_inv_dt());
        let cfm_coeff = N::splat(params.joint_cfm_coeff());

        let ni_rhs_bias = ((limits[1] - s_ang_dist).simd_max(zero)
            - (s_ang_dist - limits[0]).simd_max(zero))
            * erp_inv_dt;
        let i_rhs_bias = -ni_rhs_bias;
        let rhs_bias = i_rhs_bias.select(over_half_pi, ni_rhs_bias);
        
        let ang_jac1 = body1.sqrt_ii * ang_jac;
        let ang_jac2 = body2.sqrt_ii * ang_jac;

        JointVelocityConstraint {
            joint_id,
            mj_lambda1: body1.mj_lambda,
            mj_lambda2: body2.mj_lambda,
            im1: body1.im,
            im2: body2.im,
            impulse: N::zero(),
            impulse_bounds,
            lin_jac: na::zero(),
            ang_jac1,
            ang_jac2,
            inv_lhs: N::zero(), // Will be set during ortogonalization.
            cfm_coeff,
            cfm_gain: N::zero(),
            rhs: rhs_wo_bias + rhs_bias,
            rhs_wo_bias,
            writeback_id,
        }
    }
```

Although the adjustments made to the joint motor differ significantly from those made to the joint limit, we won't delve into them here due to their complexity and the scope of this discussion.

### SIMD programming

Adapting the function wasn't straightforward due to its parallel operation optimizations. Notably, the function is **devoid of branching**. It employs **SIMD** (Single Instruction, Multiple Data) to bolster parallel support, wherein each variable represents a list of elements.

Given the SIMD approach, traditional branching using `if` on booleans isn't feasible, as the boolean variable encapsulates an entire list of boolean values. Thus, we used bitwise calculations for branching. Consequently, our modified version is more extensive as we needed to compute different impulses and Jacobian values based on whether the angle exceeds 90 degrees or not.

## Fluid Simulation

To breathe life into our virtual creatures, it's essential they interact seamlessly with their environment. But when it comes to fluid simulation, which is pivotal for creatures that move in water, the challenge mounts. Fluid dynamics can be computationally intensive, demanding significant resources.

We weighed two primary methods during our initial phase of implementation:

### particle based simulation:

As the name suggests, this method relies on the simulation of individual particles to mimic fluid behavior. It boasts of being one of the most accurate methods available, and the simplicity of its implementation is an added advantage. However, the method is not without its drawbacks. High computational costs are a significant barrier. Using larger particles can compromise the accuracy of the fluid behavior, while opting for smaller ones can restrict the size of the world we aim to simulate.

### Viscosity Effect:

A viscosity effect is used for the simulations in underwater environments. For each exposed moving surface, a viscous force resists the normal component of its velocity, proportional to its sur-face area and normal velocity magnitude. According to Karl Sims, "This is a simple approximation that does not include the motion of the fluid itself, but is still sufficient for simulating realistic looking swimming and paddling dynamics." Compared to particle-based simulations, it's faster but demands meticulous and intricate implementation. Moreover, in this model, creatures can't influence water movement because the water itself doesn't 'move'.

A potantial bug case for viscosity effect simulation is that, for example, putting an propeller inside a box, the box can still move once the propeller starts even the box is a confined space.

Considering the pros and cons, we decided to employ the viscosity effect for every object in motion, thereby simulating an underwater environment. You can find our implementation [here](https://evosim.kaiyuanlou.com/evosim/physics/rules/fn.viscosity.html).

## Collision Rule