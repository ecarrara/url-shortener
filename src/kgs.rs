use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

const TIME_BITS: usize = 41;
const SEQUENCE_BITS: usize = 10;
const NODE_BITS: usize = 63 - TIME_BITS - SEQUENCE_BITS;

pub struct KeyGenerationService {
    node_id: u16,
    custom_epoch: SystemTime,
    previous_time: AtomicU64,
    counter: AtomicU64,
}

impl KeyGenerationService {
    pub fn new(node_id: u16) -> Self {
        // 2020-01-01 00:00:00 (UTC)
        let custom_epoch = SystemTime::UNIX_EPOCH + Duration::from_secs(1577836800);
        Self {
            node_id,
            custom_epoch,
            previous_time: AtomicU64::new(
                SystemTime::now()
                    .duration_since(custom_epoch)
                    .unwrap()
                    .as_millis() as u64,
            ),
            counter: AtomicU64::new(0),
        }
    }

    pub fn get(&self) -> u64 {
        match SystemTime::now().duration_since(self.custom_epoch) {
            Ok(elapsed) => {
                let sequence = self.counter.fetch_add(1, Ordering::AcqRel);
                let time = elapsed.as_millis() as u64;

                if time != self.previous_time.load(Ordering::SeqCst) {
                    self.previous_time.store(time, Ordering::SeqCst);
                    self.counter.store(0, Ordering::SeqCst);
                }

                (time << (SEQUENCE_BITS + NODE_BITS))
                    | (sequence << NODE_BITS)
                    | self.node_id as u64
            }
            Err(_) => panic!(),
        }
    }
}
