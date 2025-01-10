use std::{collections::HashMap, sync::{Arc, Mutex}};

pub trait Worker: Send + Sync {
    fn abort(&self) -> ();
    fn is_finished(&self) -> bool;
}

pub struct Schedule {
    pool: Arc<Mutex<HashMap<u32, Box<dyn Worker>>>>
}

impl Schedule {
    fn new() -> Self {
        Self { pool: Arc::new(Mutex::new(HashMap::new())) }
    }

    fn insert(&self, id: u32, worker: impl Worker + 'static) {
        let mut pool = self.pool.lock().unwrap();
        pool.insert(id, Box::new(worker));
    }

    fn remove(&mut self, i: u32) -> Option<Box<dyn Worker>> {
        let mut pool = self.pool.lock().unwrap();
        pool.remove(&i)
    }
}

mod impl_tokio {
    use tokio::task::JoinHandle;

    use super::Worker;

    impl<T> Worker for JoinHandle<T>
    where T: Send {
        fn abort(&self) -> () {
            self.abort()
        }
    
        fn is_finished(&self) -> bool {
            self.is_finished()
        }
    }
}
