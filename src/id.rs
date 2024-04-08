// snowflake.rs

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

const WORKER_ID_BITS: i64 = 10;
const SEQUENCE_BITS: i64 = 12;

const MAX_WORKER_ID: i64 = (1 << WORKER_ID_BITS) - 1;
const MAX_SEQUENCE: i64 = (1 << SEQUENCE_BITS) - 1;

const TIMESTAMP_SHIFT: i64 = WORKER_ID_BITS + SEQUENCE_BITS;
const WORKER_ID_SHIFT: i64 = SEQUENCE_BITS;

pub struct IdGen {
    // worker_id: i64,
    sequence: AtomicI64,
    // last_timestamp: AtomicI64,
}

impl IdGen {
    const fn new(worker_id: i64) -> Self {
        assert!(worker_id <= MAX_WORKER_ID, "Invalid worker ID");

        Self {
            // worker_id,
            sequence: AtomicI64::new(0),
            // last_timestamp: AtomicI64::new(0),
        }
    }

    pub fn ins() -> &'static Self {
        static mut INSTANCE: *const IdGen = std::ptr::null();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            let mut instance = IdGen::new(0);
            instance.sequence = AtomicI64::new(IdGen::current_timestamp());

            unsafe {
                INSTANCE = std::mem::transmute(Box::new(instance));
            }
        });

        unsafe { &*INSTANCE }
    }

    pub fn gen(&self) -> i64 {
        // let mut timestamp = Self::current_timestamp();
        //
        // loop {
        //     let last_timestamp = self.last_timestamp.load(Ordering::Relaxed);
        //
        //     if timestamp < last_timestamp {
        //         timestamp = last_timestamp;
        //     }
        //
        //     if last_timestamp == self.last_timestamp.compare_exchange(last_timestamp, timestamp, Ordering::Relaxed, Ordering::Relaxed).unwrap_or_else(|x| x) {
        //         break;
        //     }
        // }
        //
        // let sequence = self.sequence.fetch_add(1, Ordering::Relaxed) & MAX_SEQUENCE;
        // let id = (timestamp << TIMESTAMP_SHIFT) | (self.worker_id << WORKER_ID_SHIFT) | sequence;
        self.sequence.fetch_add(1, Ordering::Relaxed)
    }

    pub fn current_timestamp() -> i64 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");

        since_epoch.as_millis() as i64
    }
}