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
        let scale = 0.04;
        let height_scale = 32.0;
        let water_level = 1;
        
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let wx = world_pos.x + x as f32;
                let wz = world_pos.z + z as f32;
                
                let height = (perlin.get([
                    wx as f64 * scale,
                    wz as f64 * scale,
                ]) * height_scale) as i32;
                
                for y in 0..CHUNK_SIZE {
                    let abs_y = y as i32 + (self.position.y as i32 * CHUNK_SIZE as i32);
                    
                    if abs_y < height as i32 {
                        let block_type = if abs_y < -4 {
                            BlockType::Stone
                        } else if abs_y < -1 {
                            BlockType::Dirt
                        } else {
                            BlockType::Grass
                        };
                        
                        self.set_block(x, y, z, block_type);
                    } else if abs_y <= water_level {
                        self.set_block(x, y, z, BlockType::Water);
                    }
                }
                
                if self.position.y < 0.0 {
                    self.set_block(x, 0, z, BlockType::Stone);
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
