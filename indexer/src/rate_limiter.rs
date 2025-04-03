use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct RateLimiter {
    pub max_requests: usize,
    pub time_window: Duration,
    requests: Mutex<Vec<Instant>>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, time_window: Duration) -> Self {
        RateLimiter {
            max_requests,
            time_window,
            requests: Mutex::new(Vec::with_capacity(max_requests)),
        }
    }

    pub async fn acquire(&self) {
        loop {
            let can_proceed = self.try_acquire().await;
            if can_proceed {
                return;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn try_acquire(&self) -> bool {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();

        let cutoff = now - self.time_window;
        requests.retain(|&timestamp| timestamp > cutoff);

        if requests.len() < self.max_requests {
            requests.push(now);
            true
        } else {
            false
        }
    }
}
