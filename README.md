# zebrax

A lightweight Rust HTTP reverse proxy with rate-limiting and header rewriting.

## Features
- Round-robin load balancing across backends
- Per-IP rate limiting (configurable window + max requests)
- Request/response header injection
- Health-check endpoint `/healthz`

## Quick start
```bash
cargo run -- --config zebrax.toml
```
