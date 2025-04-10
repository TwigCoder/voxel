use crate::world::chunk::{Chunk, ChunkPos};
use parking_lot::Mutex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

pub enum ChunkTask {
    Generate(ChunkPos),
}

pub struct ChunkWorkerPool {
    tasks: Arc<Mutex<VecDeque<ChunkTask>>>,
    chunks: Arc<Mutex<HashMap<ChunkPos, Chunk>>>,
    processing: Arc<Mutex<HashSet<ChunkPos>>>,
    thread_pool: rayon::ThreadPool,
}

impl ChunkWorkerPool {
    pub fn new(chunks: Arc<Mutex<HashMap<ChunkPos, Chunk>>>) -> Self {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();

        Self {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            chunks,
            processing: Arc::new(Mutex::new(HashSet::new())),
            thread_pool,
        }
    }

    pub fn queue_chunk_generation(&self, pos: ChunkPos) {
        let mut processing = self.processing.lock();
        if processing.contains(&pos) || self.chunks.lock().contains_key(&pos) {
            return;
        }
        processing.insert(pos);
        self.tasks.lock().push_back(ChunkTask::Generate(pos));
    }

    pub fn process_tasks(&self) {
        let max_tasks = 4;
        let mut tasks_to_process = Vec::new();

        {
            let mut task_queue = self.tasks.lock();
            while tasks_to_process.len() < max_tasks && !task_queue.is_empty() {
                if let Some(task) = task_queue.pop_front() {
                    tasks_to_process.push(task);
                }
            }
        }

        for task in tasks_to_process {
            let chunks = Arc::clone(&self.chunks);
            let processing = Arc::clone(&self.processing);

            self.thread_pool.spawn(move || match task {
                ChunkTask::Generate(pos) => {
                    let mut chunk = Chunk::new(pos.to_world_pos());
                    chunk.generate_terrain(pos.to_world_pos());
                    chunks.lock().insert(pos, chunk);
                    processing.lock().remove(&pos);
                }
            });
        }
    }
}
