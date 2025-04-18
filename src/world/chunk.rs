use super::block::{BlockFace, BlockType};
use crate::engine::renderer::Vertex;
use glam::Vec3;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand::Rng;

pub const CHUNK_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn from_world_pos(pos: Vec3) -> Self {
        Self {
            x: (pos.x / CHUNK_SIZE as f32).floor() as i32,
            y: (pos.y / CHUNK_SIZE as f32).floor() as i32,
            z: (pos.z / CHUNK_SIZE as f32).floor() as i32,
        }
    }

    pub fn to_world_pos(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * CHUNK_SIZE as f32,
            self.y as f32 * CHUNK_SIZE as f32,
            self.z as f32 * CHUNK_SIZE as f32,
        )
    }
}

#[derive(Clone)]
pub struct Chunk {
    pub position: Vec3,
    blocks: [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            blocks: [[[BlockType::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            self.blocks[x][y][z] = block;
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockType {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            self.blocks[x][y][z]
        } else {
            BlockType::Air
        }
    }

    pub fn get_bounds(&self) -> (Vec3, Vec3) {
        let min = self.position;
        let max =
            self.position + Vec3::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32, CHUNK_SIZE as f32);
        (min, max)
    }

    pub fn generate_mesh(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block = self.get_block(x, y, z);
                    if block == BlockType::Air {
                        continue;
                    }

                    if y == CHUNK_SIZE - 1 || self.get_block(x, y + 1, z).is_transparent() {
                        let normal = [0.0, 1.0, 0.0];

                        vertices.extend_from_slice(&[
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Top),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Top),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Top),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Top),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Top),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Top),
                                normal,
                            },
                        ]);
                    }

                    if y == 0 || self.get_block(x, y - 1, z).is_transparent() {
                        let normal = [0.0, -1.0, 0.0];

                        vertices.extend_from_slice(&[
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Bottom),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Bottom),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Bottom),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Bottom),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Bottom),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::Bottom),
                                normal,
                            },
                        ]);
                    }

                    if z == 0 || self.get_block(x, y, z - 1).is_transparent() {
                        let normal = [0.0, 0.0, -1.0];

                        vertices.extend_from_slice(&[
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::North),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::North),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::North),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::North),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::North),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::North),
                                normal,
                            },
                        ]);
                    }

                    if z == CHUNK_SIZE - 1 || self.get_block(x, y, z + 1).is_transparent() {
                        let normal = [0.0, 0.0, 1.0];

                        vertices.extend_from_slice(&[
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::South),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::South),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::South),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::South),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::South),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::South),
                                normal,
                            },
                        ]);
                    }

                    if x == CHUNK_SIZE - 1 || self.get_block(x + 1, y, z).is_transparent() {
                        let normal = [1.0, 0.0, 0.0];

                        vertices.extend_from_slice(&[
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::East),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::East),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::East),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::East),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1.0 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::East),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + 1. + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::East),
                                normal,
                            },
                        ]);
                    }

                    if x == 0 || self.get_block(x - 1, y, z).is_transparent() {
                        let normal = [-1.0, 0.0, 0.0];

                        vertices.extend_from_slice(&[
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::West),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::West),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::West),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::West),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + 1.0 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::West),
                                normal,
                            },
                            Vertex {
                                position: [
                                    x as f32 + self.position.x,
                                    y as f32 + 1.0 + self.position.y,
                                    z as f32 + self.position.z,
                                ],
                                color: block.get_face_color(BlockFace::West),
                                normal,
                            },
                        ]);
                    }
                }
            }
        }

        vertices
    }

    fn generate_tree(&mut self, x: usize, y: usize, z: usize) {
        let height = rand::thread_rng().gen_range(4..7);

        if y + height + 2 >= CHUNK_SIZE {
            return;
        }

        if x < 2 || x >= CHUNK_SIZE - 2 || z < 2 || z >= CHUNK_SIZE - 2 {
            return;
        }

        for dy in 0..height {
            self.set_block(x, y + dy, z, BlockType::Wood);
        }

        let leave_start = y + height - 2;
        let leaf_height = 4;

        for dy in 0..4 {
            let radius = if dy == 0 || dy == leaf_height - 1 {
                1
            } else {
                2
            };

            for dx in -radius..=radius {
                for dz in -radius..radius {
                    let nx = x as i32 + dx;
                    let ny = (leave_start + dy) as i32;
                    let nz = z as i32 + dz;

                    if nx >= 0
                        && nx < CHUNK_SIZE as i32
                        && ny >= 0
                        && ny < CHUNK_SIZE as i32
                        && nz >= 0
                        && nz < CHUNK_SIZE as i32
                    {
                        if !(dx == 1 && dz == 0 && dy < height) {
                            self.set_block(
                                nx as usize,
                                ny as usize,
                                nz as usize,
                                BlockType::Leaves,
                            );
                        }
                    }
                }
            }
        }
    }

    fn generate_features(&mut self) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    if self.get_block(x, y, z) == BlockType::Grass {
                        let mut rng = thread_rng();

                        if rng.gen::<f32>() < 0.01 {
                            self.generate_tree(x, y + 1, z);
                        }
                    }
                }
            }
        }
    }

    pub fn generate_terrain(&mut self, world_pos: Vec3) {
        let perlin = Perlin::new(1234); // TODO: MAKE RANDOMIZED LATER

        let continent_scale = 0.002;
        let hills_scale = 0.02;
        let roughness_scale = 0.1;

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let wx = world_pos.x + x as f32;
                let wz = world_pos.z + z as f32;

                let continent =
                    (perlin.get([wx as f64 * continent_scale, wz as f64 * continent_scale]) + 1.0)
                        * 32.0;

                let hills = (perlin.get([
                    wx as f64 * hills_scale + 1000.0,
                    wz as f64 * hills_scale + 1000.0,
                ]) + 1.0)
                    * 16.0;

                let roughness = perlin.get([
                    wx as f64 * roughness_scale + 2000.0,
                    wz as f64 * roughness_scale + 2000.0,
                ]) * 4.0;

                let height = (continent + hills + roughness) as i32;
                let base_height = 64;
                let total_height = base_height + height;

                for y in 0..CHUNK_SIZE {
                    let abs_y = y as i32 + (self.position.y as i32 + CHUNK_SIZE as i32);

                    if abs_y < total_height {
                        let block_type = if abs_y == total_height - 1 && abs_y > base_height {
                            BlockType::Grass
                        } else if abs_y <= base_height {
                            BlockType::Dirt
                        } else if abs_y <= base_height {
                            if abs_y > base_height - 5 {
                                BlockType::Sand
                            } else {
                                let ore_noise = perlin.get([
                                    (wx as f64 * 0.5) + abs_y as f64 * 0.1,
                                    (wz as f64 * 0.5) + abs_y as f64 * 0.1,
                                ]);

                                if ore_noise > 0.8 && abs_y < 20 {
                                    BlockType::DiamondOre
                                } else if ore_noise > 0.7 && abs_y < 40 {
                                    BlockType::IronOre
                                } else if ore_noise > 0.6 {
                                    BlockType::CoalOre
                                } else {
                                    BlockType::Stone
                                }
                            }
                        } else {
                            BlockType::Stone
                        };

                        self.set_block(x, y, z, block_type)
                    } else if abs_y <= base_height {
                        self.set_block(x, y, z, BlockType::Water);
                    } else {
                        self.set_block(x, y, z, BlockType::Air);
                    }
                }

                if self.position.y == 0.0 {
                    self.set_block(x, 0, z, BlockType::Bedrock);
                }
            }
        }

        self.generate_features();
    }

    pub fn get_chunk_pos(world_pos: Vec3) -> Vec3 {
        Vec3::new(
            (world_pos.x / CHUNK_SIZE as f32).floor(),
            (world_pos.y / CHUNK_SIZE as f32).floor(),
            (world_pos.z / CHUNK_SIZE as f32).floor(),
        )
    }
}
