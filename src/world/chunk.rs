use super::block::{BlockType, BlockFace};
use crate::engine::renderer::Vertex;
use glam::Vec3;

pub const CHUNK_SIZE: usize = 16;

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
                                position: [x as f32, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            Vertex {
                                position: [x as f32, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            
                            Vertex {
                                position: [x as f32, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::Top),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::Top),
                            },
                        ]);
                    }

                    
                    if y == 0 || self.get_block(x, y - 1, z).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            Vertex {
                                position: [x as f32, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            
                            Vertex {
                                position: [x as f32 + 1.0, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                            Vertex {
                                position: [x as f32, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::Bottom),
                            },
                        ]);
                    }

                    
                    if z == 0 || self.get_block(x, y, z - 1).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::North),
                            },
                            Vertex {
                                position: [x as f32, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::North),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::North),
                            },
                            
                            Vertex {
                                position: [x as f32, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::North),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::North),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::North),
                            },
                        ]);
                    }

                    
                    if z == CHUNK_SIZE - 1 || self.get_block(x, y, z + 1).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::South),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::South),
                            },
                            Vertex {
                                position: [x as f32, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::South),
                            },
                            
                            Vertex {
                                position: [x as f32, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::South),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::South),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::South),
                            },
                        ]);
                    }

                    
                    if x == CHUNK_SIZE - 1 || self.get_block(x + 1, y, z).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32 + 1.0, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::East),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::East),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::East),
                            },
                            
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::East),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::East),
                            },
                            Vertex {
                                position: [x as f32 + 1.0, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::East),
                            },
                        ]);
                    }

                    
                    if x == 0 || self.get_block(x - 1, y, z).is_transparent() {
                        vertices.extend_from_slice(&[
                            
                            Vertex {
                                position: [x as f32, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::West),
                            },
                            Vertex {
                                position: [x as f32, y as f32, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::West),
                            },
                            Vertex {
                                position: [x as f32, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::West),
                            },
                            
                            Vertex {
                                position: [x as f32, y as f32, z as f32],
                                color: block.get_face_color(BlockFace::West),
                            },
                            Vertex {
                                position: [x as f32, y as f32 + 1.0, z as f32 + 1.0],
                                color: block.get_face_color(BlockFace::West),
                            },
                            Vertex {
                                position: [x as f32, y as f32 + 1.0, z as f32],
                                color: block.get_face_color(BlockFace::West),
                            },
                        ]);
                    }
                }
            }
        }

        vertices
    }
}
