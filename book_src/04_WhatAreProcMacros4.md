# Example - 3. Attribute Macros

```rust
#[tokio::main]
async fn main() {
    println!("hello world");
}
```

> Expands to

```rust
fn main() {
    let body = async {
        {
            ::std::io::_print(format_args!("hello world\n"));
        };
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
```