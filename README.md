# Caching Proxy Server

A simple caching proxy server written in Rust that forwards incoming requests to an origin server, caches the response, and returns the cached response if the same request is made again.

## Project Page
[GitHub Repository](https://github.com/AgentUnicorn/caching-proxy-server)

## Features
- Caches responses based on URL paths.
- Adds `X-Cache: HIT` or `X-Cache: MISS` header to indicate whether the response was served from cache.
- Uses asynchronous programming with `tokio` for concurrency.
- Command-line arguments for setting the port and origin server.

## Requirements
- Rust (latest stable version)
- tokio
- reqwest
- clap
- dashmap

## Installation
```bash
$ git clone https://github.com/AgentUnicorn/caching-proxy-server
$ cd caching-proxy-server
$ cargo build --release
```

## Usage
```bash
$ ./caching-proxy-server --port <PORT> --origin <ORIGIN_URL>
```

Example:
```bash
$ ./caching-proxy-server --port 8080 --origin http://example.com
```

## License
This project is licensed under the MIT License.

