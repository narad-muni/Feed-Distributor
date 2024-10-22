use std::{mem::{self}, ptr};

pub fn bytes_to_struct<T>(s: &[u8]) -> T {
    unsafe {
        let src = s.as_ptr() as *const T;

        std::ptr::read(src)
    }
}

pub fn struct_to_bytes<T>(s: &T, buffer: &mut [u8]) {
    unsafe{
        let mut size = std::mem::size_of::<T>();

        // Ensure the buffer is large enough
        if buffer.len() < size {
            size = buffer.len();
        }

        // Get a pointer to the value
        let ptr = s as *const T as *const u8;

        // Copy the bytes into the buffer
        std::ptr::copy_nonoverlapping(ptr, buffer.as_mut_ptr(), size);
    }
}

pub fn create_empty<T>() -> T {
    unsafe { mem::zeroed() }
}

pub fn bytes_to_partial_struct<T>(s: &mut T, buffer: &[u8]) {
    unsafe {
        // Get unsafe mutable raw pointer
        let struct_ptr = s as *mut T as *mut u8;

        // Copy lowest size
        // This is to prevent overflow
        let mut min_size = mem::size_of::<T>();

        if min_size > buffer.len() {
            min_size = buffer.len();
        }

        // Similar to memcpy
        ptr::copy_nonoverlapping(buffer.as_ptr(), struct_ptr, min_size);
    };
}

pub fn bytes_to_struct_mut<T>(buf: &mut [u8]) -> &mut T {
    unsafe { &mut *(buf.as_mut_ptr() as *mut T) }
}
