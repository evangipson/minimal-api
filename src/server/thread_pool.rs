use crate::server::{job::Job, worker::Worker};
use std::sync::{Arc, Mutex, mpsc};

/// [`ThreadPool`] orchestrates one or many [`Worker`] to a [`Job`].
pub struct ThreadPool {
    /// [`ThreadPool::workers`] is a collection of [`Worker`].
    pub workers: Vec<Worker>,
    /// [`ThreadPool::sender`] is the [Sender](mpsc::Sender) of a [`Job`].
    pub sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// [`ThreadPool::new`] will create a new [`ThreadPool`], with capacity
    /// for the provided `size` amount of [`Worker`] threads.
    /// # Example
    /// [`ThreadPool::new`] can be used to create a collection of 10
    /// [`Worker`] threads:
    /// ```rust
    /// use minimal_api::server::thread_pool::ThreadPool;
    ///
    /// fn create_worker_threads() -> ThreadPool {
    ///     ThreadPool::new(10)
    /// }
    /// ```
    /// # Panics
    /// [`ThreadPool::new`] will [`panic`] if the `size` is `0`.
    pub fn new(size: usize) -> ThreadPool {
        // panic if the size is 0 or less
        assert!(size > 0);

        // create the worker threads
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // return the thread pool with the worker threads and a reference to the sender
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// [`ThreadPool::execute`] will send a [`Job`] to a [`Worker`] thread.
    /// # Example
    /// [`ThreadPool::execute`] can be used to say "hello!" many times in a
    /// thread-safe manner:
    /// ```rust
    /// use minimal_api::server::thread_pool::ThreadPool;
    ///
    /// fn say_hello() -> String {
    ///     "hello!".to_owned()
    /// }
    ///
    /// fn start_threaded_work(thread_pool: ThreadPool) {
    ///     thread_pool.execute(|| {
    ///         say_hello();
    ///     });
    /// }
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

/// Implement [`Drop`] for [`ThreadPool`].
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
