use super::{GRAVITY, TERMINAL_VELOCITY, aabb::AABB, motion::Motion, collision};
use crate::world::chunk::{Chunk, ChunkPos};
use glam::Vec3;
use std::collections::HashMap;

const MAX_STEP_HEIGHT: f32 = 0.6;
const JUMP_FORCE: f32 = 8.0;
const MOVEMENT_SPEED: f32 = 4.5;
const AIR_CONTROL: f32 = 0.3;

pub struct PhysicsSystem {
    pub gravity_enabled: bool,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            gravity_enabled: true,
        }
    }
    
    pub fn update(&self, position: &mut Vec3, motion: &mut Motion, collider: &mut AABB, chunks: &HashMap<ChunkPos, Chunk>, dt: f32) {
        if self.gravity_enabled && !motion.on_ground {
            motion.acceleration.y = GRAVITY;
        }
        
        motion.velocity += motion.acceleration * dt;
        self.move_with_collision(position, motion, collider, chunks, motion.velocity * dt);
        motion.acceleration = Vec3::ZERO;
    }
    
    fn move_with_collision(&self, position: &mut Vec3, motion: &mut Motion, collider: &mut AABB, chunks: &HashMap<ChunkPos, Chunk>, movement: Vec3) {
        for axis in 0..3 {
            let mut axis_movement = Vec3::ZERO;
            axis_movement[axis] = movement[axis];
            collider.translate(axis_movement);
            
            let mut had_collision = false;
            let check_radius = 2;
            let pos = *position + axis_movement;
            
            for x in -check_radius..=check_radius {
                for y in -check_radius..=check_radius {
                    for z in -check_radius..=check_radius {
                        let block_pos = pos.floor() + Vec3::new(x as f32, y as f32, z as f32);
                        
                        if let Some(collision) = collision::check_block_collision(collider, chunks, block_pos) {
                            had_collision = true;
                            if axis == 1 && collision.normal.y > 0.0 {
                                motion.on_ground = true;
                            }
                            
                            let resolution = collision.normal * collision.penetration[axis].abs();
                            collider.translate(resolution);
                            axis_movement += resolution;
                            
                            motion.velocity[axis] = 0.0;
                        }
                    }
                }
            }
            
            if !had_collision {
                *position +=  axis_movement;
            }
        }
        
        *position = collider.min;
    }
}
