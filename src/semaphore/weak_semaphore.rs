use std::sync::{Condvar, Mutex};

pub struct WeakSemaphore {
    count: Mutex<usize>,
    cond: Condvar,
}

impl WeakSemaphore {
    pub fn new(count: usize) -> Self {
        Self {
            count: Mutex::new(count),
            cond: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.cond.notify_all();
    }
}