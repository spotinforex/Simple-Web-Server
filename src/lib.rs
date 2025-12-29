use std::
{thread, sync::mpsc};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender : mpsc::Sender<Job>,
}

struct Job;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    /// Create A New ThreadPool
    pub fn new(size: usize) -> ThreadPool {
        // Ensuring a Valid Number of Threads 
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create Some Threads and Store In Vector
            workers.push(Worker::new(id));
        }
        ThreadPool { workers, sender }
      }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
        }
    }

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker {id, thread}
    }
}
