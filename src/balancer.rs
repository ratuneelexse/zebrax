use std::sync::atomic::{AtomicUsize, Ordering};

/// Round-robin load balancer over a list of backend addresses.
pub struct RoundRobin {
    backends: Vec<String>,
    idx: AtomicUsize,
}

impl RoundRobin {
    pub fn new(backends: Vec<String>) -> Self {
        assert!(!backends.is_empty(), "at least one backend required");
        Self { backends, idx: AtomicUsize::new(0) }
    }

    /// Pick the next backend in rotation.
    pub fn next(&self) -> &str {
        let i = self.idx.fetch_add(1, Ordering::Relaxed) % self.backends.len();
        &self.backends[i]
    }

    pub fn len(&self) -> usize {
        self.backends.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycles_through_backends() {
        let lb = RoundRobin::new(vec!["a".into(), "b".into(), "c".into()]);
        assert_eq!(lb.next(), "a");
        assert_eq!(lb.next(), "b");
        assert_eq!(lb.next(), "c");
        assert_eq!(lb.next(), "a"); // wraps
    }
}
