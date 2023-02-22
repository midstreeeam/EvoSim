# Notes

## Physical simulation

### fluid simulation(underwater environment)

**Karl Sims strategy**: A  viscosity  effect  is  used  for  the  simulations  in  underwater environments.  For  each  exposed  moving  surface,  a  viscous  force resists the normal component of its velocity, proportional to its sur-face area and normal velocity magnitude. This is a simple approximation  that  does  not  include  the  motion  of  the  fluid  itself,  but  is still sufficient for simulating realistic looking swimming and paddling dynamics.

**implementation1**: implement drag force for every exposed moving surface.

**drag equation**: $F_{D}\,=\,{\tfrac {1}{2}}\,\rho \,v^{2}\,C_{D}\,A$

**implementation2**: particle based liquid world.
