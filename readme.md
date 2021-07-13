Godot-Rust demo for 64-bits Linux OS
===

This repo contains a sample Godot game using GDNative integration with Rust language.

*Warning: This repo is not recommended as boilerplate for start a game: Using the Gridmap Approach for generating a procedural terrain has a lot of issues related, so this code doesn't scale*

# Why

This is my first approach to start a path as #GameDev (The first in a lot of years... I have some background experience making custom Hack-roms for SNES emulator, Old CS 1.6 maps and some experiments with Unreal Engine and Unity). So I decided that I want to make videogames using Open Source Software and also start making actuve use of Rust language.

# Does it make sense make a procedural world with Tilemaps?

It depends:
- If you want to generate a small world: Yes.
- If you want to make a scenario with a few things: Yes.
- If you want to NOT interact with the tiles: Yes.

In the current version of Godot (3.3.x) there is some issues related to physics engines, Raycast and Gridmap... So I don't recommend it for a lot of use cases.

# Do you think that Godot + Rust is good enough for making full games?

Hell yeah.

# How to use it
- Clone this repo
- Setup Rust according your desired method (I prefer rustup)
- Open a terminal in the folder of the repo and use the following commands:
- `cargo clean`, `cargo build --release`
- Once build finishes, open the project in godot (root folder of the repo)
- Press "Play" and is ready :)


_Please if you want to use this code, take in mind GPLV3 license. As well, using this code/assets for make OC of any type MUST include a link to this repo, and mention to myself with my website (https://cristodcgomez.dev)_

Enjoy the learning :)
