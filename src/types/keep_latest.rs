use std::{
    mem::zeroed, ptr::{self, null_mut}, sync::{atomic::AtomicPtr, RwLock}
};

pub struct KeepLatest<T> {
    ptr: AtomicPtr<T>,
    last: RwLock<*mut T>,
}

unsafe impl<T> Send for KeepLatest<T> {}
unsafe impl<T> Sync for KeepLatest<T> {}

impl<T> KeepLatest<T> {
    pub const fn new() -> Self {
        Self {
            ptr: AtomicPtr::new(null_mut()),
            last: RwLock::new(null_mut()),
        }
    }

    pub fn init(&self) {
        *self.last.write().unwrap() = Box::into_raw(Box::new(unsafe{zeroed()}));
    }

    pub fn get(&self, data: &mut T) {
        let ptr = self
            .ptr
            .swap(null_mut(), std::sync::atomic::Ordering::SeqCst);

        if ptr.is_null() {
            let last_ptr = *self.last.read().unwrap();
            
            unsafe {
                ptr::copy_nonoverlapping(last_ptr, data, 1);
            }
        } else {
            // memcpy to last ptr
            let ptr_last = self.last.write().unwrap();
            unsafe {
                ptr::copy_nonoverlapping(ptr, *ptr_last, 1);
                drop(ptr_last);

                // Copy from ptr to data
                ptr::copy_nonoverlapping(ptr, data, 1);

                // delete ptr
                let _ = Box::from_raw(ptr);
            }
        }
    }

    pub fn write(&self, data: T) -> bool {
        let mut consumed = false;
        let new_ptr = Box::into_raw(Box::new(data));

        // Assign new_ptr to ptr
        let old_ptr = self.ptr.swap(new_ptr, std::sync::atomic::Ordering::SeqCst);

        // Mark consumed true if old_ptr is null
        if old_ptr.is_null() {
            consumed = true;
        } else {
            unsafe {
                let _ = Box::from_raw(old_ptr);
            }
        }

        consumed
    }
}
