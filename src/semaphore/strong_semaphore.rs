use std::collections::VecDeque;
use std::sync::{Condvar, Mutex, Arc};

pub struct StrongSemaphore {
    count: Mutex<usize>,
    queue: Mutex<VecDeque<Arc<Condvar>>>,
}

impl StrongSemaphore {
    pub fn new(count: usize) -> Self {
        Self {
            count: Mutex::new(count),
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        if *count == 0 {
            let cvar = Arc::new(Condvar::new());
            self.queue.lock().unwrap().push_back(cvar.clone());
            while *count == 0 {
                count = cvar.wait(count).unwrap();
            }
        }
    }

    pub fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        if let Some(cvar) = self.queue.lock().unwrap().pop_front() {
            cvar.notify_one();
        }
    }
}