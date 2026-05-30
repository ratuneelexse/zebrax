use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Mutex,
    time::Instant,
};

struct Window {
    count: u64,
    reset_at: Instant,
}

/// Token-bucket per-IP rate limiter.
pub struct RateLimiter {
    max_requests: u64,
    window: std::time::Duration,
    state: Mutex<HashMap<IpAddr, Window>>,
}

impl RateLimiter {
    pub fn new(max_requests: u64, window: std::time::Duration) -> Self {
        Self { max_requests, window, state: Mutex::new(HashMap::new()) }
    }

    /// Returns `true` if the request should be allowed, `false` if rate-limited.
    pub fn check(&self, ip: IpAddr) -> bool {
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();
        let entry = state.entry(ip).or_insert(Window { count: 0, reset_at: now + self.window });

        if now >= entry.reset_at {
            entry.count = 0;
            entry.reset_at = now + self.window;
        }

        if entry.count >= self.max_requests {
            return false;
        }

        entry.count += 1;
        true
    }

    /// Remove expired windows (call periodically to bound memory).
    pub fn prune(&self) {
        let now = Instant::now();
        let mut state = self.state.lock().unwrap();
        state.retain(|_, w| now < w.reset_at);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn allows_up_to_max_then_blocks() {
        let limiter = RateLimiter::new(3, std::time::Duration::from_secs(60));
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        assert!(limiter.check(ip));
        assert!(limiter.check(ip));
        assert!(limiter.check(ip));
        assert!(!limiter.check(ip)); // 4th request blocked
    }
}
