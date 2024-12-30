# Rust-Based Voxel Engine

## Overview

This project is a 3D voxel engine built entirely using Rust. The engine is designed to create a voxel-based environment, allowing for the rendering of various block types and interactions. The architecture is modular.

This project is a work in progress and is not yet ready for production. It is a personal project for me to learn Rust and 3D graphics programming and is inspired by Veloren.

## Movement

1. WASD for movement in the direction you're looking
2. Space/LShift for up/down movement
3. Right mouse button + mouse movement to look around
4. LControl for sprint (2x movement speed)


### Current Features

- **Chunk Management**: The world is divided into chunks, each containing a 3D array of blocks.

- **Block Types**: Various block types are defined, each with unique properties and behaviors. This includes transparency, flammability, and interaction types that will later be used by the upcoming physics engine.

- **Mesh Generation**: The engine generates meshes for visible blocks, optimizing rendering by only creating geometry for blocks that are not surrounded by other blocks.

## Future Goals

- [ ] **Procedural World Generation**: Implement algorithms to generate dynamically generate terrain and structures.

- [ ] **Fluids**: Implement and simulate fluids.

- [ ] **Physics Engine**: Introduce a physics engine to handle collisions, gravity, etc.

- [ ] **Lighting System**: Develop a lighting system, day/night cycles, shadows, and ambient occlusion.

- [ ] **Optimization**: Implement frustum culling, greedy meshing, and LOD (level of detail) for distant chunks to improve performance.

