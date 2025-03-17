# HTTP Proxy Server with Caching

This project is a simple HTTP proxy server written in Rust. It forwards requests to a specified origin server and caches responses for future requests. Cached responses are served with an `X-Cache` header indicating whether the response was a `HIT` or `MISS`.

## Features

- Asynchronous handling of requests using `tokio`.
- Simple in-memory caching using `dashmap`.
- Custom header `X-Cache` to indicate cache status.
- Command-line arguments for setting port and origin.

## Dependencies

- `tokio`: Asynchronous runtime for Rust.
- `reqwest`: HTTP client for forwarding requests.
- `dashmap`: Thread-safe concurrent hashmap.
- `clap`: Command-line argument parser.

## Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
dashmap = "4.0"
clap = { version = "4.0", features = ["derive"] }
```

## Usage

Compile the project using:

```bash
cargo build --release
```

Run the server using:

```bash
./target/release/your_project_name --port <PORT> --origin <ORIGIN>
```

Example:

```bash
./target/release/your_project_name --port 8080 --origin http://example.com
```

## How It Works

1. The server listens for incoming HTTP requests on the specified port.
2. It attempts to serve the request from the cache if available.
3. If not cached, it forwards the request to the specified origin server.
4. The response is cached and then sent to the client.

## Custom Headers

- If the response is served from the cache, the header `X-Cache: HIT` is added.
- If the response is fetched from the origin, the header `X-Cache: MISS` is added.

## Example Request

If you run the server as follows:

```bash
./target/release/your_project_name --port 8080 --origin http://example.com
```

And make a request to:

```http
GET http://localhost:8080/path/to/resource
```

The server will forward the request to:

```http
http://example.com/path/to/resource
```

## License

This project is licensed under the MIT License.
