[![Actions Status](https://github.com/m00s/mastering-rust-tutorial/workflows/Rust/badge.svg)](https://github.com/m00s/mastering-rust-tutorial/actions) [![Build Status](https://travis-ci.com/m00s/mastering-rust-tutorial.svg?branch=master)](https://travis-ci.com/m00s/mastering-rust-tutorial)

# Mastering Rust tutorial

This is a project to learn the Rust programming language

## Specifications

We obviously need to have a clue about what we're trying to build.
Here's the mile-high view of the project:

- Real-time city-building/strategy game
- Client/server architecture
- The game area is a 2D grid of squares, with each square having the following:
    - A mandatory terrain ground
    - An optional terrain block
    - Objects
    - Beings

The terrain ground can be one of *soil* or *stone*. This unsurprisingly refers to the ground or floor.

The terrain block can be *soil*, *stone*, or *tree*. This refers to a non-passable block that can be left as a wall, or be mined or felled away.

Beings are living creatures, and each square may have one of them.

We'll want to make it possible to move *Being* in any direction of *Grid* with the following cases being errors:

- There is no *Being* in *Square*
- *Being* tries to fall off from the edge of *Grid*
- *Being* tries to move into *Square* where there is already *Being*
- *Being* tries to move to *Terrain* which is *Stone*
