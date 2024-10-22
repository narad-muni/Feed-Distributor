use crossbeam::queue::SegQueue;

#[derive(Debug)]
pub struct ReuseArr<T> {
    data: Vec<Option<T>>,
    free_queue: SegQueue<usize>,
}

impl <T> ReuseArr<T> {
    pub fn new() -> Self {
        let free_queue = SegQueue::new();
        let data = Vec::new();

        for i in 0..data.capacity() {
            free_queue.push(i);
        }

        Self {
            data,
            free_queue,
        }
    }

    pub fn insert(&mut self, data: T) -> usize {
        if let Some(idx) = self.free_queue.pop() {

            if idx < self.data.len() {
                self.data[idx] = Some(data);
            } else {
                self.data.push(Some(data));
            }

            idx
        } else {
            // No free index, allocate a new one
            self.data.push(Some(data));
            let idx = self.data.len();

            let size = self.data.capacity();

            // Push to free queue
            for i in idx..size {
                self.free_queue.push(i);
            }

            idx
        }
    }

    pub fn get(&self, idx: usize) -> &Option<T> {
        &self.data[idx]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut Option<T> {
        &mut self.data[idx]
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        self.free_queue.push(idx);

        self.data[idx].take()
    }
}