mod common;

#[cfg(test)]
mod semaphore_tests {
    use std::sync::Arc;
    use std::time::Duration;
    use concurrency_project::semaphore::{
        weak_semaphore::WeakSemaphore,
        strong_semaphore::StrongSemaphore,
        counting_semaphore::CountingSemaphore,
        binary_semaphore::BinarySemaphore,
    };
    use crate::common::semaphore_timing::test_semaphore_timing;

    #[test]
    fn test_weak_semaphore_timing() {
        let semaphore = Arc::new(WeakSemaphore::new(10));
        test_semaphore_timing(
            "WeakSemaphore",
            semaphore,
            |s| s.acquire(),
            |s| s.release(),
            100,
            Duration::from_millis(10),
        );
    }

    #[test]
    fn test_strong_semaphore_timing() {
        let semaphore = Arc::new(StrongSemaphore::new(10));
        test_semaphore_timing(
            "StrongSemaphore",
            semaphore,
            |s| s.acquire(),
            |s| s.release(),
            100,
            Duration::from_millis(10),
        );
    }

    #[test]
    fn test_counting_semaphore_timing() {
        let semaphore = Arc::new(CountingSemaphore::new(10));
        test_semaphore_timing(
            "CountingSemaphore",
            semaphore,
            |s| s.acquire(),
            |s| s.release(),
            100,
            Duration::from_millis(10),
        );
    }

    #[test]
    fn test_binary_semaphore_timing() {
        let semaphore = Arc::new(BinarySemaphore::new());
        test_semaphore_timing(
            "BinarySemaphore",
            semaphore,
            |s| s.acquire(),
            |s| s.release(),
            100,
            Duration::from_millis(10),
        );
    }
}