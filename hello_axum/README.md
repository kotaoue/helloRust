# hello_axum

This is a simple Hello World web application built with the [Axum](https://github.com/tokio-rs/axum) framework.

## Usage

```sh
cd hello_axum
cargo run
```

Once the server is running, open your browser or use `curl` to access it:

```sh
curl http://127.0.0.1:8080/
```

```sh
curl http://127.0.0.1:8080/sing
```

```sh
curl http://127.0.0.1:8080/laugh
```

```sh
curl http://127.0.0.1:8080/hello
```

```sh
curl http://127.0.0.1:8080/hello/Alice
```

```sh
curl http://127.0.0.1:8080/fail
```

```sh
curl http://127.0.0.1:8080/json
```

```sh
curl "http://127.0.0.1:8080/json?message=axum%20style"
```

## References

* [Axum](https://github.com/tokio-rs/axum)
* [axum - crates.io](https://crates.io/crates/axum)
