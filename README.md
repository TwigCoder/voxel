# Rust Voxel Engine

[![Rust](https://github.com/TwigCoder/voxel/actions/workflows/rust.yml/badge.svg)](https://github.com/TwigCoder/voxel/actions/workflows/rust.yml)

## Overview

This is a voxel engine built completely in Rust! This project was inspired by Veloren (an open-source voxel game built in Rust) and was an opportunity for me to learn the language whilst coding something I love!

I plan to use this engine to create my own game in the future.

## Movement

1. WASD for movement in the direction you're looking
2. Space/LShift for up/down movement
3. Right mouse button + mouse movement to look around
4. LControl for sprint (2x movement speed)

## Current Features

Block types are fully implemented, including their unique properties and behaviors (transparency, flammability, and interaction types).

The world is divided into chunks. Visible blocks in these chunks are rendered through mesh generation along with optimizations such as frustum culling and chunk loading/unloading.

The world is now infinite and procedurally generated, consisting of grass, stone, water, and sand.

Light and the day/night cycle has now been introduced to the engine.

## Backlog

- [ ] Procedurally generating structures and biomes.

- [ ] Optimizations including level of detail and multithreading.

- [ ] Developing the physics engine and animations. (fluid logic, etc.)

- [ ] Designing a user interface for the soon-to-be-implemented players.
