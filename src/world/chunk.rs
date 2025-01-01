use super::block::{BlockType, BlockFace};
use crate::engine::renderer::Vertex;
use glam::Vec3;
use noise::{NoiseFn, Perlin};

pub const CHUNK_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl ChunkPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self{x, y, z}
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
        let max = self.position + Vec3::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32, CHUNK_SIZE as f32);
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
                    
                    let color = block.get_face_color(BlockFace::Top);
                    
                    
                    if y == CHUNK_SIZE - 1 || self.get_block(x, y + 1, z).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::Top),
                            },
                        ]);
                    }

                    
                    if y == 0 || self.get_block(x, y - 1, z).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                        ]);
                    }

                    
                    if z == 0 || self.get_block(x, y, z - 1).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::North),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::North),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::North),
                            },
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::North),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::North),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::North),
                            },
                        ]);
                    }

                    
                    if z == CHUNK_SIZE - 1 || self.get_block(x, y, z + 1).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::South),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::South),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::South),
                            },
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::South),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::South),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::South),
                            },
                        ]);
                    }

                    
                    if x == CHUNK_SIZE - 1 || self.get_block(x + 1, y, z).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::East),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::East),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::East),
                            },
                            
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::East),
                            },
                            Vertex {
                                position: [x as f32 + 1.0 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::East),
                            },
                            Vertex {
                                position: [x as f32 + 1. + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::East),
                            },
                        ]);
                    }

                    
                    if x == 0 || self.get_block(x - 1, y, z).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::West),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::West),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::West),
                            },
                            
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::West),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + 1.0 + self.position.z],
                                color: block.get_face_color(BlockFace::West),
                            },
                            Vertex {
                                position: [x as f32 + self.position.x, y as f32 + 1.0 + self.position.y, z as f32 + self.position.z],
                                color: block.get_face_color(BlockFace::West),
                            },
                        ]);
                    }
                }
            }
        }

        vertices
    }
    
    pub fn generate_terrain(&mut self, world_pos: Vec3) {
        let perlin = Perlin::new(1); // TODO: MAKE RANDOMIZED LATER
        let mountain_noise = Perlin::new(2);
        let biome_noise = Perlin::new(3);
        let cave_noise = Perlin::new(4);
        
        let base_scale = 0.02;
        let mountain_scale = 0.015;
        let biome_scale = 0.007;
        let detail_scale = 0.08;
        let cave_scale = 0.05;
        
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let wx = world_pos.x + x as f32;
                let wz = world_pos.z + z as f32;
                
                let base_height = (perlin.get([
                    wx as f64 * base_scale,
                    wz as f64 * base_scale,
                ]) * 32.0) as i32;
                
                let mountain_influence = (mountain_noise.get([
                    wx as f64 * mountain_scale,
                    wz as f64 * mountain_scale,
                ]) + 1.0) * 48.0;
                
                let biome_value = biome_noise.get([
                    wx as f64 * biome_scale,
                    wz as f64 * biome_scale,
                ]);
                
                let height = base_height + mountain_influence as i32;
                
                for y in 0..CHUNK_SIZE {
                    let wy = world_pos.y + y as f32;
                    let abs_y = y as i32 + (self.position.y as i32 * CHUNK_SIZE as i32);
                    
                    let cave_density = cave_noise.get([
                        wx as f64 * cave_scale,
                        wy as f64 * cave_scale,
                        wz as f64 * cave_scale
                    ]);
                    
                    if abs_y < height as i32 {
                        if cave_density > 0.03 && abs_y < height - 5 {
                            continue;
                        }
                        
                        let block_type = if abs_y > height - 1 {
                            if biome_value > 0.6 {
                                if abs_y > 40 {
                                    BlockType::Snow
                                } else {
                                    BlockType::Stone
                                }
                            } else if biome_value > 0.2 {
                                BlockType::Grass
                            } else if biome_value > -0.2 {
                                BlockType::Sand
                            } else {
                                BlockType::Clay
                            }
                        } else if abs_y > height - 4 {
                            if biome_value > 0.2 {
                                BlockType::Dirt
                            } else {
                                BlockType::Sand
                            }
                        } else {
                            let stone_noise = perlin.get([
                                wx as f64 * detail_scale,
                                abs_y as f64 * detail_scale,
                                wz as f64 * detail_scale,
                            ]);
                            
                            if stone_noise > 0.7 && abs_y < 0 {
                                match (stone_noise * 100.0) as i32 % 4 {
                                    0 => BlockType::IronOre,
                                    1 => BlockType::CoalOre,
                                    2 => BlockType::GoldOre,
                                    _ => BlockType::Stone,
                                }
                            } else {
                                BlockType::Stone
                            }
                        };
                        
                        self.set_block(x, y, z, block_type);
                        
                    } else if abs_y <= 0 {
                        self.set_block(x, y, z, BlockType::Water);
                    }
                }
                
                if let Some(surface_y) = (0..CHUNK_SIZE).rev()
                        .find(|&y| self.get_block(x, y, z) != BlockType::Air
                        && self.get_block(x, y, z) != BlockType::Water) {
                            if self.get_block(x, surface_y, z) == BlockType::Grass {
                                let tree_chance = perlin.get([
                                    wx as f64 * 0.3,
                                    wz as f64 * 0.3,
                                ]);
                                
                                if tree_chance > 0.8 && surface_y + 4 < CHUNK_SIZE {
                                    for ty in 1..4 {
                                        self.set_block(x, surface_y + ty, z, BlockType::Wood);
                                    }
                                
                                for lx in -2..=2 {
                                    for ly in 3..=5 {
                                        for lz in -2..=2 {
                                            let leaf_x = x as i32 + lx;
                                            let leaf_y = surface_y as i32 + ly;
                                            let leaf_z = z as i32 + lz;
                                            
                                            if leaf_x >= 0 && leaf_x < CHUNK_SIZE as i32
                                            && leaf_y >= 0 && leaf_y < CHUNK_SIZE as i32
                                            && leaf_z >= 0 && leaf_z < CHUNK_SIZE as i32 {
                                                if (lx * lx + (ly - 4) * (ly - 4) + lz * lz) as f32 <= 4.0 {
                                                    self.set_block(leaf_x as usize, leaf_y as usize, leaf_z as usize, BlockType::Leaves);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            if self.position.y <= 0.0 {
                for x in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        self.set_block(x, 0, z, BlockType::Bedrock);
                    }
                }
            }
        }
    
    pub fn get_chunk_pos(world_pos: Vec3) -> Vec3 {
        Vec3::new(
            (world_pos.x / CHUNK_SIZE as f32).floor(),
            (world_pos.y / CHUNK_SIZE as f32).floor(),
            (world_pos.z / CHUNK_SIZE as f32).floor()
        )
    }
}