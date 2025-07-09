use crate::server::job::Job;
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

/// [`Worker`] represents a threaded server unit-of-work.
pub struct Worker {
    /// [`Worker::id`] is the identifier of a [`Worker`].
    pub id: usize,
    /// [`Worker::thread`] is a reference to the [`std::thread`]
    /// that a [`Worker`] is responsible for handling.
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// [`Worker::new`] will create a new [`Worker`] that will operate on
    /// the provided [`Job`] in a thread-safe manner.
    /// # Example
    /// [`Worker::new`] can be used to create a new [`Worker`] to operate
    /// on a [`Job`]:
    /// ```rust
    /// use minimal_api::server::{
    ///     job::Job,
    ///     worker::Worker
    /// };
    /// use std::{
    ///     sync::{Arc, Mutex, mpsc}
    /// };
    ///
    /// fn create_worker(
    ///     id: usize,
    ///     receiver: Arc<Mutex<mpsc::Receiver<Job>>>
    /// ) -> Worker {
    ///     Worker::new(id, receiver)
    /// }
    /// ```
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                match receiver.lock().unwrap().recv() {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
