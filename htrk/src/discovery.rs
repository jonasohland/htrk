use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::ops::Deref;

pub struct DiscoveryContext {
    discovery_worker: std::thread::JoinHandle<()>,
    sync: Arc<(Mutex<bool>, Condvar)>,
}

struct DiscoveryThreadContext {
    sync: Arc<(Mutex<bool>, Condvar)>,
}

impl DiscoveryContext {
    pub fn stop(self: &mut DiscoveryContext) {}
}

fn discovery_run(ctx: DiscoveryThreadContext) {
    let (mutex, c_var) = &*ctx.sync;
    let mut started = mutex.lock().unwrap();
    while !*started {
        started = c_var.wait(started).unwrap();
    }
}



impl DiscoveryContext {
    pub fn new() -> DiscoveryContext {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let thread_ctx = DiscoveryThreadContext {
            sync: pair.clone(),
        };
        DiscoveryContext {
            sync: pair,
            discovery_worker: thread::spawn(|| discovery_run(thread_ctx)),
        }
    }
}