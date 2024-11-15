use std::{
    sync::{atomic::Ordering, Arc},
    thread::JoinHandle,
};

use crate::types::work::ClientWork;

use super::ThreadPoolMaster;
use crossbeam::queue::SegQueue;

struct ClientThreadpool {
    tpool_queue: Arc<SegQueue<ClientWork>>,
    tpool: ThreadPoolMaster<ClientWork>,
}

impl ClientThreadpool {
    pub fn new(num_threads: usize) -> Self {
        let tpool_queue = Arc::new(SegQueue::new());

        Self {
            tpool_queue: tpool_queue.clone(),
            tpool: ThreadPoolMaster::new(num_threads, tpool_queue),
        }
    }

    pub fn do_work(work: ClientWork) {
        // Add work to queue
        work.client_profile.work_list.push(work);

        // Acquire lock
        if work
            .client_profile
            .work_lock
            .compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {}
    }

    // Start threadpool
    pub fn start_tpool(&self) -> JoinHandle<()> {
        self.tpool.start_tpool()
    }
}
