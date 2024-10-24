
#[cfg(test)]
mod strong_semaphore_tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use concurrency_project::semaphore::strong_semaphore::StrongSemaphore;

    #[test]
    fn test_strong_semaphore_concurrency() {
        let semaphore = Arc::new(StrongSemaphore::new(1));
        let counter = Arc::new(Mutex::new(0));
        let num_threads = 500;
        let mut handles = vec![];

        for _ in 0..num_threads {
            let sem = Arc::clone(&semaphore);
            let counter = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                sem.acquire();
                {
                    let mut count = counter.lock().unwrap();
                    *count += 1;
                    assert_eq!(*count, 1, "More than one thread in critical section");
                    thread::sleep(Duration::from_millis(10));
                    *count -= 1;
                }
                sem.release();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 0, "Final count should be 0");
    }

    #[test]
    fn test_strong_semaphore_order() {
        let semaphore = Arc::new(StrongSemaphore::new(1));
        let order = Arc::new(Mutex::new(Vec::new()));
        let num_threads = 5;
        let mut handles = vec![];

        for i in 0..num_threads {
            let sem = Arc::clone(&semaphore);
            let order = Arc::clone(&order);
            handles.push(thread::spawn(move || {
                sem.acquire();
                order.lock().unwrap().push(i);
                thread::sleep(Duration::from_millis(10));
                sem.release();
            }));
            thread::sleep(Duration::from_millis(1));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_order = order.lock().unwrap();
        println!("Execution order: {:?}", *final_order);
        // FIFO 순서보장 테스트
        assert_eq!(*final_order, (0..num_threads).collect::<Vec<_>>());
    }

    #[test]
    fn test_strong_semaphore_multiple_permits() {
        let semaphore = Arc::new(StrongSemaphore::new(3));
        let counter = Arc::new(Mutex::new(0));
        let num_threads = 10;
        let mut handles = vec![];

        for _ in 0..num_threads {
            let sem = Arc::clone(&semaphore);
            let counter = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                sem.acquire();
                {
                    let mut count = counter.lock().unwrap();
                    *count += 1;
                    assert!(*count <= 3, "More than three threads in critical section");
                    thread::sleep(Duration::from_millis(10));
                    *count -= 1;
                }
                sem.release();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 0, "Final count should be 0");
    }

    #[test]
    fn test_strong_semaphore_zero_init() {
        let semaphore = Arc::new(StrongSemaphore::new(0));
        let flag = Arc::new(Mutex::new(false));
        let handle = {
            let sem = Arc::clone(&semaphore);
            let flag = Arc::clone(&flag);
            thread::spawn(move || {
                sem.acquire();
                *flag.lock().unwrap() = true;
                sem.release();
            })
        };

        thread::sleep(Duration::from_millis(100));
        assert_eq!(*flag.lock().unwrap(), false, "Semaphore should not have been acquired");

        semaphore.release();
        handle.join().unwrap();
        assert_eq!(*flag.lock().unwrap(), true, "Semaphore should have been acquired and released");
    }
}

// test_strong_semaphore_concurrency: 동시성 제어가 제대로 작동하는지 확인합니다. 한 번에 하나의 스레드만 임계 영역에 접근할 수 있어야 합니다.
// test_strong_semaphore_order: 강한 세마포어의 FIFO(First-In-First-Out) 특성을 검증합니다. 스레드들이 생성된 순서대로 세마포어를 획득해야 합니다.
// test_strong_semaphore_multiple_permits: 여러 개의 허가(permit)를 가진 세마포어의 동작을 테스트합니다. 이 경우 최대 3개의 스레드가 동시에 임계 영역에 있을 수 있습니다.
// test_strong_semaphore_zero_init: 0으로 초기화된 세마포어의 동작을 테스트합니다. 세마포어가 해제되기 전까지 스레드가 블록되어야 합니다.