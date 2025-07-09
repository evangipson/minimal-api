/// [`Job`] represents an action for a threaded worker.
pub type Job = Box<dyn FnOnce() + Send + 'static>;
