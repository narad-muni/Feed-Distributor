pub mod client_threadpool;

use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crossbeam::queue::SegQueue;
use threadpool::ThreadPool;

pub trait WorkTrait {
    fn do_work(&self);
}

pub struct ThreadPoolMaster<T: WorkTrait + 'static + Send + Sync> {
    pool: ThreadPool,
    tpool_queue: Arc<SegQueue<T>>,
}

unsafe impl<T: WorkTrait + Send + Sync> Send for ThreadPoolMaster<T> {}
unsafe impl<T: WorkTrait + Send + Sync> Sync for ThreadPoolMaster<T> {}

impl<T: WorkTrait + Send + Sync> ThreadPoolMaster<T> {
    pub fn new(num_threads: usize, tpool_queue: Arc<SegQueue<T>>) -> Self {
        let pool = ThreadPool::new(num_threads);

        Self { pool, tpool_queue }
    }

    pub fn start_tpool(&self) -> JoinHandle<()> {
        let tpool_queue = self.tpool_queue.clone();
        let pool = self.pool.clone();

        thread::spawn(move || loop {
            if let Some(work) = tpool_queue.pop() {
                // Run in threadpool
                pool.execute(move || work.do_work());
            }
        })
    }
}
