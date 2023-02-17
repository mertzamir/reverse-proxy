# Simple Reverse Proxy

This project implements a reverse proxy server that forwards the HTTP requests to the
necessary origin server and returns the original response to the user. It also maintains
an in-memory cache with a TTL of 30 seconds, so if the user sends subsequent same requests
within the TTL, the response from cache is returned.

#### Start Server

```
cargo run
```