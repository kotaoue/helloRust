# helloRust
The study of Rust

## Unit Tests

The project includes basic unit tests for `is_odd` and `is_even` functions.

To run the tests, navigate to the `cargo` directory and use `cargo test`:

```bash
cd cargo
cargo test
```

Expected output:

```
running 4 tests
test tests::test_is_even_with_odd_number ... ok
test tests::test_is_even_with_even_number ... ok
test tests::test_is_odd_with_even_number ... ok
test tests::test_is_odd_with_odd_number ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## References
* [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)
