# Compare C, C++ and Rust!

| Category             | C (Preprocessor Macros)          | C++ (Templates)                          | Rust (Proc Macros)                                |
|----------------------|----------------------------------|-------------------------------------------|---------------------------------------------------|
| When they run        | Before compilation (preprocessor)| During compilation (template instantiation)| During compilation (syntax transformation)       |
| What they see        | Raw text / tokens                | Types, templates, concepts                | Token stream / AST-like structured syntax         |
| Safety & Ergonomics  | No type safety, brittle, unclear errors | Type-checked but complex errors      | Type-checked after expansion, good tooling/errors |
