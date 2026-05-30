# zebrax

A lightweight Rust HTTP reverse proxy with rate-limiting and header rewriting.

## Features
- Round-robin load balancing across N backends
- Per-IP rate limiting (configurable window + max requests)
- Request/response header injection
- `/healthz` health-check endpoint
- Single static binary, no runtime deps

## Install
```bash
cargo build --release
./target/release/zebrax --config zebrax.toml
```

## Configuration (`zebrax.toml`)
```toml
listen   = "0.0.0.0:8080"
backends = ["http://127.0.0.1:3001", "http://127.0.0.1:3002"]

[rate_limit]
max_requests = 100
window_secs  = 60

[headers]
request  = [["X-Forwarded-By", "zebrax"]]
response = [["X-Proxy", "zebrax"]]
```

## Run tests
```bash
cargo test
```
