# Notes

## Physical simulation

### fluid simulation(underwater environment)

#### implementation1: 

implement drag force for every exposed moving surface.

**Karl Sims strategy**: A  viscosity  effect  is  used  for  the  simulations  in  underwater environments.  For  each  exposed  moving  surface,  a  viscous  force resists the normal component of its velocity, proportional to its sur-face area and normal velocity magnitude. This is a simple approximation  that  does  not  include  the  motion  of  the  fluid  itself,  but  is still sufficient for simulating realistic looking swimming and paddling dynamics.

**drag equation**: $F_{D}\,=\,{\tfrac {1}{2}}\,\rho \,v^{2}\,C_{D}\,A$

#### implementation2:

particle based liquid world.

**John Buffer SFML implementation**: https://github.com/johnBuffer/VerletSFML, https://www.youtube.com/watch?v=lS_qeBy3aQI

**Salva_2d**: https://github.com/dimforge/salva

### Vacuo Simulation

much easier and cheaper than fluid.



## Brain Design

### Normal DNN

can use python implementation

can preform reinforcement learning:

- Using Monte Carlo can be slow, but it might still faster than genetic algorithm.

- Q-learning: 
  - problem 1: infinite possible stages and actions
    - function approximation, using DQN
  - problem2: complex action (give a vector instead of choose the one with highest q value)

hard to implement genetic algorithm (too much parameters)
