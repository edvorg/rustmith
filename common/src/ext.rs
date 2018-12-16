use std::time::Duration;

pub trait DurationExt {
    fn total_millis(&self) -> u64;
}

impl DurationExt for Duration {
    fn total_millis(&self) -> u64 {
        self.as_secs() * 1000 + u64::from(self.subsec_millis())
    }
}
