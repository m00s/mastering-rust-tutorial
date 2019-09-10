[![Build Status](https://travis-ci.org/m00s/mastering-rust-tutorial.svg?branch=master)](https://travis-ci.org/m00s/mastering-rust-tutorial)

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