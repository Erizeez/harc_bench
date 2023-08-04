use std::{cell::UnsafeCell, sync::Arc};

pub struct Buffer {
    buffer: Arc<UnsafeCell<Vec<u32>>>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: Arc::new(UnsafeCell::new(vec![0; width * height])),
        }
    }

    pub fn get(&self) -> &mut Vec<u32> {
        unsafe { &mut *self.buffer.get() }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
        }
    }
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}
