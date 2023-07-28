# dev log

## physical simulation

### basic physical simulation

- using `bevy_rapier2d`
- bugs about joint motor and joint limit and its fix
  - collision detection
  - collision solver
  - constrain builder
  - constrain solver

### fluid simulation

- viscosity simulation

### parallel acceleration

- SIMD programming
- avoiding branch structure using bitwise operation
- ECS way instead of OOP way
- parallel systems access
- parallel iterator

### parallel connection problem
- the first output is received after the last input is sent, but it runs in a loop

### timestep

- game tick vs framerate, delta_time
- fixed timestep configuration

## virtual world construction

### creature structure

- blob construction & rules
- random blob rules, self-conflict situation (block size, tree structure)
- self-confliction in mutation

## brain structure

### Centrol, Inward, Ourward

### Brain parallel compute management