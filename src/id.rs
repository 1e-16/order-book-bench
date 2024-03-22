// snowflake.rs

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

const WORKER_ID_BITS: u64 = 10;
const SEQUENCE_BITS: u64 = 12;

const MAX_WORKER_ID: u64 = (1 << WORKER_ID_BITS) - 1;
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

const TIMESTAMP_SHIFT: u64 = WORKER_ID_BITS + SEQUENCE_BITS;
const WORKER_ID_SHIFT: u64 = SEQUENCE_BITS;

pub struct IdGen {
    worker_id: u64,
    sequence: AtomicU64,
    last_timestamp: AtomicU64,
}

impl IdGen {
    const fn new(worker_id: u64) -> Self {
        assert!(worker_id <= MAX_WORKER_ID, "Invalid worker ID");

        Self {
            worker_id,
            sequence: AtomicU64::new(0),
            last_timestamp: AtomicU64::new(0),
        }
    }

    pub fn ins() -> &'static Self {
        static mut INSTANCE: *const IdGen = std::ptr::null();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            let instance = IdGen::new(0);
            unsafe {
                INSTANCE = std::mem::transmute(Box::new(instance));
            }
        });

        unsafe { &*INSTANCE }
    }

    pub fn gen(&self) -> u64 {
        let mut timestamp = Self::current_timestamp();

        loop {
            let last_timestamp = self.last_timestamp.load(Ordering::Relaxed);

            if timestamp < last_timestamp {
                timestamp = last_timestamp;
            }

            if last_timestamp == self.last_timestamp.compare_exchange(last_timestamp, timestamp, Ordering::Relaxed, Ordering::Relaxed).unwrap_or_else(|x| x) {
                break;
            }
        }

        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed) & MAX_SEQUENCE;
        let id = (timestamp << TIMESTAMP_SHIFT) | (self.worker_id << WORKER_ID_SHIFT) | sequence;

        id
    }

    pub fn current_timestamp() -> u64 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");

        since_epoch.as_millis() as u64
    }
}