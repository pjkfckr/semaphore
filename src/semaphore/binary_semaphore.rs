use std::sync::{Condvar, Mutex};

pub struct BinarySemaphore {
    flag: Mutex<bool>,
    cond: Condvar,
}

impl BinarySemaphore {
    pub fn new() -> Self {
        Self {
            flag: Mutex::new(true),
            cond: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut flag = self.flag.lock().unwrap();

        while !*flag {
            flag = self.cond.wait(flag).unwrap();
        }

        *flag = false;
    }

    pub fn release(&self) {
        let mut flag = self.flag.lock().unwrap();
        *flag = true;
        self.cond.notify_one();
    }
}