# Quote Variable Interpolation

> Why are those Iterators useful?

> Variable Interpolation with `#var`

- `#(#var)*` - no separators.
- `#(#var),*` - character, comma, before the asterisk is used as separator.
- `#(struct #var;)*` - Repetition can contain other tokens too.
