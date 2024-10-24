
#[cfg(test)]
mod counting_semaphore_tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use concurrency_project::semaphore::counting_semaphore::CountingSemaphore;

    //동시성 제어가 제대로 작동하는지 확인합니다. 최대 3개의 스레드만 동시에 임계 영역에 접근할 수 있어야 합니다.
    #[test]
    fn test_counting_semaphore_concurrency() {
        let semaphore = Arc::new(CountingSemaphore::new(3));
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

    //  여러 개의 리소스를 관리하는 CountingSemaphore의 능력을 테스트합니다. 이 경우 3개의 리소스를 동시에 사용할 수 있어야 합니다.
    #[test]
    fn test_counting_semaphore_multiple_resources() {
        let semaphore = Arc::new(CountingSemaphore::new(3));
        let resources_in_use = Arc::new(Mutex::new(0));
        let num_threads = 15;
        let mut handles = vec![];

        for _ in 0..num_threads {
            let sem = Arc::clone(&semaphore);
            let resources = Arc::clone(&resources_in_use);
            handles.push(thread::spawn(move || {
                sem.acquire();
                {
                    let mut count = resources.lock().unwrap();
                    *count += 1;
                    assert!(*count <= 3, "More than three resources in use");
                    thread::sleep(Duration::from_millis(10));
                    *count -= 1;
                }
                sem.release();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*resources_in_use.lock().unwrap(), 0, "All resources should be released");
    }

    //0으로 초기화된 세마포어의 동작을 테스트합니다. 세마포어가 해제되기 전까지 스레드가 블록되어야 합니다.
    #[test]
    fn test_counting_semaphore_zero_init() {
        let semaphore = Arc::new(CountingSemaphore::new(0));
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

    // 세마포어가 여러 번 해제될 때의 동작을 테스트합니다. 이는 CountingSemaphore의 카운트 증가 기능을 확인합니다.
    #[test]
    fn test_counting_semaphore_multiple_release() {
        let semaphore = Arc::new(CountingSemaphore::new(1));
        semaphore.release();
        semaphore.release();

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..3 {
            let sem = Arc::clone(&semaphore);
            let counter = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                sem.acquire();
                {
                    let mut count = counter.lock().unwrap();
                    *count += 1;
                }
                thread::sleep(Duration::from_millis(10));
                sem.release();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 3, "All three threads should have run");
    }
}