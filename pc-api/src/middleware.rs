//! Middleware for API

use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use std::time::Instant;
use tower_http::limit::RequestBodyLimitLayer;
use tracing::info;

/// Logging middleware
pub async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    info!(
        method = %method,
        uri = %uri,
        status = %status,
        duration_ms = %duration.as_millis(),
        "Request completed"
    );

    response
}

/// Request body size limit (10 MB)
pub fn body_limit_layer() -> RequestBodyLimitLayer {
    RequestBodyLimitLayer::new(10 * 1024 * 1024)
}

/// Simple rate limiting (in-memory, per IP)
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub fn check_rate_limit(&self, ip: &str) -> bool {
        let mut requests = self.requests.write().unwrap();
        let now = Instant::now();

        let ip_requests = requests.entry(ip.to_string()).or_insert_with(Vec::new);

        // Remove old requests outside the window
        ip_requests.retain(|&time| now.duration_since(time) < self.window);

        if ip_requests.len() >= self.max_requests {
            return false; // Rate limit exceeded
        }

        ip_requests.push(now);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(3, Duration::from_secs(1));

        // First 3 requests should pass
        assert!(limiter.check_rate_limit("127.0.0.1"));
        assert!(limiter.check_rate_limit("127.0.0.1"));
        assert!(limiter.check_rate_limit("127.0.0.1"));

        // 4th request should be blocked
        assert!(!limiter.check_rate_limit("127.0.0.1"));

        // Different IP should work
        assert!(limiter.check_rate_limit("192.168.1.1"));
    }
}
