# Example - 1. Function-like Macros

Generates Rust-Code from the Macro Input. Looks like declarative Macro. Embeds JSON as DSL.

```rust
let john = json!({
    "name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
});
```

> Expands to:

```rust
let john = ::serde_json::Value::Object({
    let mut object = ::serde_json::Map::new();
    let _ = object
        .insert(("name").into(), ::serde_json::to_value(&"John Doe").unwrap());
    let _ = object.insert(("age").into(), ::serde_json::to_value(&43).unwrap());
    let _ = object
        .insert(
            ("phones").into(),
            ::serde_json::Value::Array(
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        ::serde_json::to_value(&"+44 1234567").unwrap(),
                        ::serde_json::to_value(&"+44 2345678").unwrap(),
                    ]),
                ),
            ),
        );
    object
});
```
