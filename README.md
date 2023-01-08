# Japanese Date Parser

A minimal rust library for converting Japanese date strings into parsed dates.
The library returns `chrono::NaiveDate`s.

The library takes all existing Japanese eras into account, and handles the
following date formats by design:

- 2012年03月12日
- 2012年03月12日 (月)
- 2012年03月12日 (月曜日)
- 12/03/12
- 12-03-12
- 12.02.12
- 平成24年3月12日
- H24.03.12
- 平24.03.12

# Example

```rust
let result = parse("2012年03月12日 (月曜日)")
assert_eq!(
    result,
    Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
);
```
