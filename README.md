<h1 align="center">Spikes</h1>
<p align="center">
A simple game made using <a href="https://bevyengine.org">Bevy</a>. 
</p> 
<p align="center">
It may or may not be a bit over engineered in order to experiment with
Bevy's design patterns.
</p> 

<p align="center">
<img src="https://user-images.githubusercontent.com/43417195/148283481-7956132d-a3e8-4366-b0f8-5d4aba905372.gif" width="300px">
</p> 



## TODO
- [x] Basic Game State
- [x] Basic File Structure
- [x] Display something
- [x] Manage Assets / Resources (works for now)
  - [ ] Figure out a better way to handle global resources
- [x] Multiple scenes with their own systems 
  - [x] How to destroy elements from a scene
- [x] Manage input in different scenes
- [x] Player that moves 
- [x] Walls
  - [ ] Maybe change Direction and use just rotation when spawning the spikes
  - [x] ~~Maybe separate the walls and their colliders~~ Implement relative positioning for colliders
- [x] Collisions
- [x] Spike generation, movement and destruction
- [ ] Implement score and lose conditions
- [ ] Sprites / Animated sprites
- [ ] Music ([@Elkowar](https://github.com/elkowar)...?) 
- [ ] Resize

# Maybe...
- [ ] Look into SystemParam
- [ ] Look into system chainning (specially for collision -> movement)
- [ ] Maybe implement collider in a separate crate (overkill? nah)
- [ ] Abstract sprites and other components into a component file / directory
- [ ] Look into heron / rapier
- [ ] Look into debug plugins
- [ ] Look into physics plugins
- [ ] Look into asset loading plugins
- [ ] Decouple game actions from inputs
