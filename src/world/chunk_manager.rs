use crate::world::chunk::{Chunk, ChunkPos, CHUNK_SIZE};
use crate::world::chunk_worker::ChunkWorkerPool;
use glam::Vec3;
use parking_lot::Mutex;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct ChunkManager {
    chunks: Arc<Mutex<HashMap<ChunkPos, Chunk>>>,
    chunk_cache: HashMap<ChunkPos, Chunk>,
    active_chunks: HashSet<ChunkPos>,
    worker_pool: ChunkWorkerPool,
    cache_size: usize,
}

impl ChunkManager {
    pub fn new() -> Self {
        let chunks = Arc::new(Mutex::new(HashMap::new()));
        let worker_pool = ChunkWorkerPool::new(Arc::clone(&chunks));

        Self {
            chunks,
            chunk_cache: HashMap::with_capacity(64),
            active_chunks: HashSet::new(),
            worker_pool,
            cache_size: 64,
        }
    }

    pub fn update(&mut self, camera_pos: Vec3, render_distance: i32) {
        let camera_chunk = ChunkPos::from_world_pos(camera_pos);
        let mut new_active = HashSet::new();
        let mut to_load = Vec::new();

        for y in -2..=2 {
            for x in -render_distance..=render_distance {
                for z in -render_distance..=render_distance {
                    let pos =
                        ChunkPos::new(camera_chunk.x + x, camera_chunk.y + y, camera_chunk.z + z);

                    let dist_sq = x * x + y * y * 4 + z * z;
                    if dist_sq <= render_distance * render_distance {
                        new_active.insert(pos);
                        if !self.chunks.lock().contains_key(&pos) {
                            to_load.push(pos);
                        }
                    }
                }
            }
        }

        let to_unload: Vec<ChunkPos> = self
            .active_chunks
            .difference(&new_active)
            .copied()
            .collect();

        for pos in to_unload {
            if let Some(chunk) = self.chunks.lock().remove(&pos) {
                if self.chunk_cache.len() >= self.cache_size {
                    self.chunk_cache
                        .remove(&self.chunk_cache.keys().next().copied().unwrap());
                }
                self.chunk_cache.insert(pos, chunk);
            }
        }

        to_load.sort_by_key(|pos| {
            let dx = pos.x - camera_chunk.x;
            let dy = pos.y - camera_chunk.y;
            let dz = pos.z - camera_chunk.z;
            dx * dx + dy * dy + dz * dz
        });

        for pos in to_load {
            if let Some(chunk) = self.chunk_cache.remove(&pos) {
                self.chunks.lock().insert(pos, chunk);
            } else {
                self.worker_pool.queue_chunk_generation(pos);
            }
        }

        self.active_chunks = new_active;
        self.worker_pool.process_tasks();
    }

    pub fn get_chunks(&self) -> Arc<Mutex<HashMap<ChunkPos, Chunk>>> {
        Arc::clone(&self.chunks)
    }
}
