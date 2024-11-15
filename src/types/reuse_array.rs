use crossbeam::queue::SegQueue;
use std::{cell::UnsafeCell, sync::RwLock};

#[derive(Debug)]
pub struct ReuseArr<T> {
    arr: RwLock<Vec<UnsafeCell<Option<T>>>>,
    free_queue: SegQueue<usize>,
}

// Assuming safety because, Vector size can only be changed with a write lock.
// Modifying inner elements take read lock, because one thread in threadpool only modifies one index.
// Multiple threads are not allowed to modify inner elements.

// If we require multiple threads to access same index, we will need to place rwlock inside option i.e. unsafecell<option<rwlock>>

unsafe impl<T> Sync for ReuseArr<T> {}
unsafe impl<T> Send for ReuseArr<T> {}

impl<T> ReuseArr<T> {
    pub fn new() -> Self {
        let free_queue = SegQueue::new();
        let arr = RwLock::new(Vec::new());

        // Push initial indexes
        {
            let arr = arr.read().unwrap();
            for i in 0..arr.capacity() {
                free_queue.push(i);
            }
        }

        Self { arr, free_queue }
    }

    pub fn reserve(&self) -> usize {
        if let Some(idx) = self.free_queue.pop() {
            let arr = self.arr.read().unwrap();

            // If index is already allocated
            if idx < arr.len() {
                let element = arr[idx].get();

                unsafe {
                    *element = None;
                }
            } else {
                // Drop read lock
                drop(arr);

                // Get write lock
                let mut arr = self.arr.write().unwrap();

                arr.push(UnsafeCell::new(None));
            }

            idx
        } else {
            let mut arr = self.arr.write().unwrap();
            // No free index, allocate a new one
            let idx = arr.len();
            arr.push(UnsafeCell::new(None));

            let size = arr.capacity();

            // Push to free queue
            for i in idx + 1..size {
                self.free_queue.push(i);
            }

            idx
        }
    }

    pub fn insert(&self, data: T) -> usize {
        if let Some(idx) = self.free_queue.pop() {
            let arr = self.arr.read().unwrap();

            if idx < arr.len() {
                let element = arr[idx].get();

                unsafe {
                    *element = Some(data);
                }
            } else {
                // Drop read lock
                drop(arr);

                // Get write lock
                let mut arr = self.arr.write().unwrap();

                arr.push(UnsafeCell::new(Some(data)));
            }

            idx
        } else {
            let mut arr = self.arr.write().unwrap();

            // No free index, allocate a new one
            arr.push(UnsafeCell::new(Some(data)));
            let idx = arr.len();

            let size = arr.capacity();

            // Push to free queue
            for i in idx..size {
                self.free_queue.push(i);
            }

            idx
        }
    }

    pub fn insert_at(&self, data: T, idx: usize) -> usize {
        let arr = self.arr.read().unwrap();

        if idx < arr.len() {
            let element = arr[idx].get();

            unsafe {
                *element = Some(data);
            }
        } else {
            drop(arr);

            let mut arr = self.arr.write().unwrap();
            arr.push(UnsafeCell::new(Some(data)));
        }

        idx
    }

    pub fn get(&self, idx: usize) -> &Option<T> {
        let arr = self.arr.read().unwrap();

        let element = arr[idx].get();

        unsafe { &*element }
    }

    pub fn get_mut(&self, idx: usize) -> &mut Option<T> {
        let arr = self.arr.read().unwrap();

        let element = arr[idx].get();

        unsafe { &mut *element }
    }

    pub fn remove(&self, idx: usize) -> Option<T> {
        self.free_queue.push(idx);

        let arr = self.arr.read().unwrap();

        unsafe { (*arr[idx].get()).take() }
    }
}
