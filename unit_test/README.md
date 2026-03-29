# unit_test

`is_odd` と `is_even` 関数を使ったRustのユニットテストの基本サンプルです。

## テストの実行方法

`unit_test` ディレクトリに移動して `cargo test` を実行します。

```bash
cd unit_test
cargo test
```

実行結果の例:

```
running 4 tests
test tests::test_is_even_with_even_number ... ok
test tests::test_is_even_with_odd_number ... ok
test tests::test_is_odd_with_even_number ... ok
test tests::test_is_odd_with_odd_number ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## テスト内容

| 関数 | 説明 |
|------|------|
| `is_odd(n: i32) -> bool` | `n` が奇数なら `true` を返す |
| `is_even(n: i32) -> bool` | `n` が偶数なら `true` を返す |

## References

* [Testing - The Rust Programming Language](https://doc.rust-lang.org/book/ch11-00-testing.html)
* [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch11-00-testing.html)
