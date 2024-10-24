use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Default)]
struct TimingStats {
    wait_times: Vec<Duration>,
    execution_times: Vec<Duration>
}

pub fn test_semaphore_timing<S, A, R> (
    name: &str,
    semaphore: Arc<S>,
    acquire: A,
    release: R,
    num_threads: usize,
    work_duration: Duration
) where
    S: Send + Sync + 'static,
    A: Fn(&S) + Send + Sync + Clone + 'static,
    R: Fn(&S) + Send + Sync + Clone + 'static,
{
    let stats = Arc::new(Mutex::new(TimingStats::default()));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let sem = Arc::clone(&semaphore);
        let stats = Arc::clone(&stats);
        let acquire = acquire.clone();
        let release = release.clone();

        handles.push(thread::spawn(move || {
            let wait_start = Instant::now();
            acquire(&sem);
            let wait_time = wait_start.elapsed();

            let execution_start = Instant::now();
            thread::sleep(work_duration);
            let execution_time = execution_start.elapsed();

            release(&sem);

            let mut stats = stats.lock().unwrap();
            stats.wait_times.push(wait_time);
            stats.execution_times.push(execution_time);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let stats = stats.lock().unwrap();

    let avg_wait_time: Duration = stats.wait_times.iter().sum::<Duration>() /
        stats.wait_times.len() as u32;

    let avg_execution_time: Duration = stats.execution_times.iter().sum::<Duration>() /
        stats.execution_times.len() as u32;

    println!("{} Timing Results:", name);
    println!("Average wait time: {:.2} ms", avg_wait_time.as_millis());
    println!("Average execution time: {:.2} ms", avg_execution_time.as_millis());
}