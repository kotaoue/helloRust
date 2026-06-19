# hello_rocket

This is a simple Hello World web application built with the [Rocket](https://rocket.rs/) framework.

## Usage

```sh
cd hello_rocket
cargo run
```

Once the server is running, open your browser or use `curl` to access it:

```sh
curl http://127.0.0.1:8000/
```

```sh
curl http://127.0.0.1:8000/sing
```

```sh
curl http://127.0.0.1:8000/laugh
```

```sh
curl http://127.0.0.1:8000/hello
```

```sh
curl http://127.0.0.1:8000/hello/Alice
```

```sh
curl http://127.0.0.1:8000/fail
```

```sh
curl http://127.0.0.1:8000/json
```

```sh
curl "http://127.0.0.1:8000/json?message=warp%20style"
```

## References

* [Rocket](https://rocket.rs/)
* [rocket - crates.io](https://crates.io/crates/rocket)
