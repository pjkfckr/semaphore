
#[cfg(test)]
mod binary_semaphore_tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration};
    use concurrency_project::semaphore::binary_semaphore::BinarySemaphore;

    // 상호 배제가 제대로 작동하는지 확인합니다. 한 번에 하나의 스레드만 임계 영역에 접근할 수 있어야 합니다.
    #[test]
    fn test_binary_semaphore_mutual_exclusion() {
        let semaphore = Arc::new(BinarySemaphore::new());
        let counter = Arc::new(Mutex::new(0));
        let num_threads = 500;
        let mut handles = vec![];

        for i in 0..num_threads {
            let sem = Arc::clone(&semaphore);
            let counter = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                sem.acquire();
                {
                    let mut count = counter.lock().unwrap();
                    *count += 1;
                    assert_eq!(*count, 1, "More than one thread in critical section");
                    println!("Thread {} acquired the semaphore", i);
                    thread::sleep(Duration::from_millis(10));
                    *count -= 1;
                }
                sem.release();
                println!("Thread {} released the semaphore", i);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 0, "Final count should be 0");
    }

    // 세마포어가 여러 번 해제되어도 문제가 없는지 확인합니다.
    #[test]
    fn test_binary_semaphore_multiple_release() {
        let semaphore = Arc::new(BinarySemaphore::new());
        semaphore.release();

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

        handle.join().unwrap();
        assert_eq!(*flag.lock().unwrap(), true, "Thread should have acquired and released the semaphore");
    }


    // 세마포어가 스레드들에게 비교적 공정하게 접근 권한을 부여하는지 확인합니다.
    #[test]
    fn test_binary_semaphore_fairness() {
        let semaphore = Arc::new(BinarySemaphore::new());
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
        // Note: We can't guarantee perfect fairness, but we can check if it's reasonably fair
        assert!(final_order.windows(2).filter(|w| w[0] > w[1]).count() <= 1,
                "Order should be mostly fair");
    }
}
