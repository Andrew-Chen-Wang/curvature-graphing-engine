# Natural Gravity Engine

| Created by | [Andrew Chen Wang](https://github.com/Andrew-Chen-Wang)
|------------|----------------------
| Created on | 24 June 2020

> Physics engine with 4D graphs that mimics curvatures of space for researching gravitational waves.

This engine's core design lies in simple concepts. If you become too complex,
incorporating new papers and ideas, the engine becomes unusable too quickly.
Making everything simple allows for these old and new principles to be
alive without even coding it: 

This physics engine makes gravity an object (thank you VSauce; something about
people attracting each other) because gravitational waves are "waves..." (maybe).
This may allow for a little better understanding of how gravity affects other "objects".
More on that in the dedicated section.

---
### Introduction

This physics engine was designed to understand gravity and the curvature of space.
It allows for creating objects and viewing a graph at the same time that
the generated object bends space-time. Thus, unlike other physics engines,
**gravity is represented as an invisible object in-game (more representable
as the graph).**

Generating an object begins with a "perfect" sphere with frozen time.
You can also opt to generate an object while time continues. You can think
of it like CAD with layers and objects that can "empty" the insides of a
specified object. An easy option of expanding the initial sphere is to click
the surface area once to create a point which you can expand via
the basic arrow keys. This can also be done with multiple (logical) points
representing a 3D object that can also be used to expand or retract objects
based on your arrow key directions.

The graph that you'll see show how space-time is being bent, assuming we
are using the 4D model of "space." You can enable "random photons" to help
imagine how the curvature works (think of it like a black hole).

To increase the density of an object or access its attributes,
let's say to make a black dwarf, you must click on an object to open
a menu and edit its properties. The option of changing density
supports frozen and unfrozen time. Changing chemical composition
can only be done in frozen time.

This project keeps in mind that plugins can be developed to extend the core
framework. Please view the plugins section for open-source plugins
that have been approved.

Although I'm no astrophysicist, not even an amateur astronomer, I have a general
interest in this field. If you see a conceptual problem, please open a GitHub
issue. Thank you!

---
### Why is gravity an object?

Most physics engines that incorporate gravity make objects "attract" each
other based on their mass and density. This physics engine makes gravity an "object"
because, if we stick with the gravitational wave theory that we have
detected in 2015, then it may allow researchers to better grasp WTF
is going on with stuff.

The reason why this won't break your computer because C++ is mostly OOP
whereas Rust's amethyst goes data-driven, making it a lighter computer
game. There was a great paper from 1958 (?) on rotating objects affecting
the curvature of space that I feel like other engines completely miss.
And that I'll miss too if I don't make gravity an object.

Because its an object, its behavior is based on simple ideas. Making
everything simple is not ground-breaking physics; it's how you will
interact with the engine to make ground-breaking discoveries.

The ideation of gravity being simple came about... 7 hours after creating
this README. The reason is simple: new theories means new code. What if
you didn't NEED any new code because the in-game physics already
represented everything that could ever come about. I hopefully will create
an engine that mimics the natural beauty of gravity so that discovering
properties of gravity does not need to be so difficult.

---
### Beta Usage

I'm a total beginner in Rust, so...

To run the master branch game, run the following command (OS X):

```bash
cargo run --features metal
```

For Windows or Linux:

```bash
cargo run --features vulkan
```

---
### Plugins

If you have a plugin, open a PR with your plugin's name with link (it must be
[open source]((https://youtu.be/95Tc0Rk2cNg?t=260)) to be considered on this list).

---
### License and Contact

The code in this repository is licensed under the
[GNU LESSER GENERAL PUBLIC LICENSE](https://github.com/Andrew-Chen-Wang/curvature-graphing-engine/blob/master/LICENSE).
I'm considering changing this license if more contributors
come and help as I definitely cannot maintain this forever.

This project uses Material Icons: [License](https://github.com/google/material-design-icons/blob/master/LICENSE)

If you'd like to contact me, please email me by pressing this link:

[Andrew Wang at acwangpython@gmail.com](mailto:acwangpython@gmail.com?subject=[GH%20Space%20Curvature])

If you don't use the default email app of your device,
please at least prefix you message with "GH: Space Curvature".
Thank you!
