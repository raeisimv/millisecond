# Millisecond crate
A better way to format and display duration, which converts `33023448000ms` to `1y 17d 5h 10m 48s` or relatively timestamp of`about a year ago`.

## Install
In your Rust project's root directory run:

```shell
$ cargo add millisecond
```

## Example
```rust
// Activate and bring the crate into scope
use millisecond::prelude::*;

fn main() {
    // Obtain a duration instance
    let dur = core::time::Duration::from_millis(33_023_448_000);

    println!("{}", dur.pretty());
    // displays: 1y 17d 5h 10m 48s

    println!("{}", dur.pretty_with(&MillisecondOption::long()));
    // displays: 1 year 17 days 5 hours 10 minutes 48 seconds

    println!("{} ago", dur.relative());
    // displays: about a year ago

    // the previous solution still works
    let ms = Millisecond::from_millis(33_023_448_000);
    println!("{}", ms.pretty());
    // displays: 1y 17d 5h 10m 48s
}
```

## Options
Customize the parser and the output format using the `MillisecondOption` struct.

| Option                  | Description                                                                                             | Example           |
|:------------------------|:--------------------------------------------------------------------------------------------------------|:------------------|
| `long`                  | uses full and descriptive labels for time units, such as `years` instead of abbreviated forms like `y`. | `2y` -> `2 years` |
| `dominant_only`         | displays the most dominant part only (the most left part).                                              | `1y 2d` -> `1y`   |
| `days_instead_of_years` | displays time durations in days rather than converting them into years.                                 | `1y 1d` -> `366d` |

*All options have default value unless specified*

### Options shorthand
In order to easily create a `MillisecondOption` instance, you can use the `MillisecondOption::default()` method:
```rust
let option = MillisecondOption {
    days_instead_of_years: true,
    ..MillisecondOption::default()
};
```
___
### License
MIT

##### Inspiration
This crate is inspired by `pretty-ms` npm package.
