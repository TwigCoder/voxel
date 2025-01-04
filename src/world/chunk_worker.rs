use std::sync::Arc;
use crossbeam::channel::{Sender, Receiver, unbounded};
use parking_lot::Mutex;
use rayon::prelude::*;
use num_cpus;
use std::collections::HashMap;
use crate::world::chunk::{Chunk, ChunkPos};

pub enum ChunkTask {
    Generate(ChunkPos),
    Mesh(ChunkPos),
}

pub struct ChunkWorkerPool {
    task_sender: Sender<ChunkTask>,
    task_receiver: Receiver<ChunkTask>,
    chunks: Arc<Mutex<HashMap<ChunkPos, Chunk>>>,
    thread_pool: rayon::ThreadPool,
}

impl ChunkWorkerPool {
    pub fn new(chunks: Arc<Mutex<HashMap<ChunkPos, Chunk>>>) -> Self {
        let (task_sender, task_receiver) = unbounded();
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus::get() - 1)
            .build()
            .unwrap();
        
        Self {
            task_sender,
            task_receiver,
            chunks,
            thread_pool,
        }
    }
    
    pub fn queue_chunk_generation(&self, pos: ChunkPos) {
        self.task_sender.send(ChunkTask::Generate(pos))
            .expect("ERROR | chunk_worker | queue chunk generation");
    }
    
    pub fn queue_chunk_mesh(&self, pos: ChunkPos) {
        self.task_sender.send(ChunkTask::Mesh(pos))
            .expect("ERROR | chunk_worker | queue chunk mesh");
    }
    
    pub fn process_tasks(&self) {
        while let Ok(task) = self.task_receiver.try_recv() {
            let chunks = Arc::clone(&self.chunks);
            
            self.thread_pool.spawn(move || {
                match task {
                    ChunkTask::Generate(pos) => {
                        let mut chunk = Chunk::new(pos.to_world_pos());
                        chunk.generate_terrain(pos.to_world_pos());
                        chunks.lock().insert(pos, chunk);
                    }
                    ChunkTask::Mesh(pos) => {
                        if let Some(chunk) = chunks.lock().get_mut(&pos) {
                            chunk.generate_mesh();
                        }
                    }
                }
            });
        }
    }
}
