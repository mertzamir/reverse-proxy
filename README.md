# Simple Reverse Proxy

This project implements a reverse proxy server that forwards the HTTP requests to the
necessary origin server and returns the original response to the user. All the response
data (body) and metadata (status code, headers) are copied over to preserve the full
original response.

It also maintains an in-memory cache with a TTL of 30 seconds, so if the user sends consecutive requests
with the same URL, the cached response is returned if it's within TTL. (Please monitor server logs to see
which responses are fetched from the origin server and which ones are returned directly from the in-memory cache.)


For simplicity, reverse proxy only expects GET requests to the index path ("/"). All the other request
types return to 404 Not Found. If the GET request doesn't have the origin server as the query parameter,
it returns to 400 Bad Request. See the correct query example below:

#### Start Server

```
cargo run
```

#### Example Query
Specify the origin server in the query parameters with a key of `url`
```
curl localhost:8080/?url=https://httpbin.org/get
```

#### Run Tests
```
cargo test
```