## Note: This is a WIP

I still haven't fully thought through how to
go about the gravity, which is the entire purpose
of this simulation.

As of 26 June 2020, I decided, for now, that you
could represent the gravity as gravity "objects"
and that they could merge to increase attraction
in their current direction. However, a rising issue
was showing the growing amount of attraction
as an object's mass grows, not to mention handling
density and the reformation of combining masses.

---

The gravity in the game is represented as an
"object" or, for Rust + amethyst, a data point.
Each density of 1 spherical centimeter represents
100 gravitational waves.

A useful concept to remember is that gravity,
to our current knowledge, is neither dark matter
nor dark energy; thus, whatever is filling up
space should not be gravity.

Also, why does a person not become a miniature
planet? Why doesn't an astronaut just collapse
within oneself.

---
### Gravity

The way gravity works in this engine is by imagining
it as a spherical object surrounding a cube. (The graph
is split cubically). Each spherical "gravity" object
has a direction to it that points to its original object.
The spherical objects are one unit away from the center
of the cube. (The object you get in-game is a sphere,
not a cube, but this guide uses cubes for demonstration
purposes).

For the purposes of demonstration, I will explain the
game's gravity with 2D rather than 3D (since... text)
Additionally, in game, you're given spheres, not cubes.
Assume the cube has a gravity "object" inside of
it with direction "nil," as in no direction.

```
● ● ●
● ■ ●  < 10 gravity objs for one cube
● ● ●
```

Now let's add a second cube, two units away from the
first one, so that you get a gravity "object" that
interferes with another. Let's assume the two cubes
have enough density and are close enough together
to be able to attract.

```
● ● ● ● ●
● ■ ● ■ ●  < 15 + 3 due to intersection = 18. 
● ● ● ● ●
```

The middle sphere is actually two gravity "objects,"
and their "pull" is still alive. Eventually, in the
simulator, you're left with this (assuming the objects
merge):

```
  ● ●
● ● ● ●
● ■ ■ ●
● ● ● ●
  ● ●
```

Note: the above figure depicts a MERGE of the two cubes
-- assuming collision resulted in a "singular" object
that has more mass but the same amount of density.

More spheres have disappeared. Let's break it down.
Due to each object's "attraction", the cubes have
"merged," but the spheres are still intact. They have
simply merged into the cube giving them an arbitrary
"gravitational pull" constant. The same goes for any
obstructed gravity "object" that combines with another,
giving this combination an _arbitrary_ additional unit
of "gravitational attraction."

If you haven't noticed the pattern, for every spherical
unit per arbitrary in-game pixel count, you get 6 surrounding
gravity "objects" and an additional internal gravity
"object" for the generated sphere itself.

---
### Gravitational waves

As explained from Feynman's "[Sticky Bead](http://physicsbuzz.physicscentral.com/2016/06/gravitational-waves-explained-sticky.html)"
argument, gravity carries energy. This physics
engine portrays this argument internally since
gravity is now represented as an object.

(Moving at the speed of light and bending space-time
is mind bending... ba dum chhh. But seriously... how?)

The better, default representation of these waves
are in the form of the graph, which include arrows
to point in the direction of flow (if it isn't
obvious enough for one object; it may be helpful
with multiple objects).

A future implementation will show ripples with color
coordination; although, I'm not sure I should tie it
in too much with the gravity "object" concept since
gravitational waves are effected by gravity.

If you'd like to compare some data in the simulation
with real data with something like LIGO, you can visit
[Chirp](http://chirp.sr.bham.ac.uk).

---
### Said something wrong?
Of course! Like the "data point." I don't think
that's even the correct terminology or if I'm
thinking of this correctly. If I'm not, please
contact me or open up a GitHub issue.
