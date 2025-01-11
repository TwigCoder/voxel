use super::aabb::AABB;
use crate::world::block::BlockType;
use crate::world::chunk::Chunk;
use glam::Vec3;
use std::collections::HashMap;
use crate::world::chunk::ChunkPos;

#[derive(Debug)]
pub struct CollisionResult {
    pub collision: bool,
    pub penetration: Vec3,
    pub normal: Vec3,
}

pub fn check_block_collision(
    aabb: &AABB,
    chunks: &HashMap<ChunkPos, Chunk>,
    position: Vec3,
) -> Option<CollisionResult> {
    let block_min = Vec3::new(
        position.x.floor(),
        position.y.floor(),
        position.z.floor(),
    );
    let block_max = block_min + Vec3::ONE;
    let block_aabb = AABB::new(block_min, block_max);
    
    if !aabb.intersects(&block_aabb) {
        return None;
    }
    
    let chunk_pos = ChunkPos::from_world_pos(position);
    if let Some(chunk) = chunks.get(&chunk_pos) {
        let local_pos = (position - chunk.position).floor();
        let block = chunk.get_block(
            local_pos.x as usize,
            local_pos.y as usize,
            local_pos.z as usize,
        );
        
        if block != BlockType::Air && !block.is_fluid() {
            let penetration = Vec3::new(
                (block_max.x -aabb.min.x).min(aabb.max.x - block_min.x),
                (block_max.y -aabb.min.y).min(aabb.max.y - block_min.y),
                (block_max.z -aabb.min.z).min(aabb.max.z - block_min.z),
            );
            
            let normal = if penetration.x < penetration.y && penetration.x < penetration.z {
                Vec3::new(if aabb.min.x < block_min.x {-1.0} else {1.0}, 0.0, 0.0)
            } else if penetration.y < penetration.z {
                Vec3::new(0.0, if aabb.min.y < block_min.y {-1.0} else {1.0}, 0.0)
            } else {
                Vec3::new(0.0, 0.0, if aabb.min.z < block_min.z {-1.0} else {1.0})
            };
            
            return Some(CollisionResult {
                collision: true,
                penetration,
                normal
            });
        }
    }
    
    None
}
