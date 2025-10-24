# Declarative Macros (Pattern Matching)

> How are declarative Macros defined?

```rust
#[macro_export]
macro_rules! vec {
    () => (
        $crate::vec::Vec::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::vec::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        <[_]>::into_vec(
            // Using the intrinsic produces a dramatic improvement in stack usage for
            // unoptimized programs using this code path to construct large Vecs.
            $crate::boxed::box_new([$($x),+])
        )
    );
}
```

- They are powerful, e.g. you can do a [Lisp Parser](https://github.com/corgijan/lispr/blob/master/src/main.rs)
- They can help a lot with boilerplate!
