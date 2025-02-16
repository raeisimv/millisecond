# Millisecond crate
A better way to format and display time, which converts `33023448000ms` to `1y 17d 5h 10m 48s`

## Install
In your Rust project root directory run:

```shell
$ cargo add millisecond
```

## Example
```rust
use millisecond::prelude::*;

fn main() {
    // Obtain a duration instance
    let dur = core::time::Duration::from_millis(33_023_448_000);

    println!("pretty: {}", dur.pretty());
    // pretty: 1y 17d 5h 10m 48s

    println!("pretty_with: {}", dur.pretty_with(&MillisecondOption::long()));
    // pretty_with: 1 year 17 days 5 hours 10 minutes 48 seconds

    // the previous solution still works
    let ms = Millisecond::from_millis(33_023_448_000);
    println!("ms: {}", ms.pretty());
    // dur: 1y 17d 5h 10m 48s
}
```

## Options
Customize the parser and the output format using the `MillisecondOption` struct.

| Option | Description | Example |
| :--- | :--- | :--- |
| `long` | uses full and descriptive labels for time units, such as `years` instead of abbreviated forms like `y`. | `2y` -> `2 years` |
| `dominant_only` | displays the most dominant part only (the most left part). | `1y 2d` -> `1y` |
| `days_instead_of_years` | displays time durations in days rather than converting them into years. | `1y 1d` -> `366d` |

*All options have deafult value unless specified*

### Options shorthand
In order to easily create a `MillisecondOption` instance, you can use the `MillisecondOption::default()` method:
```rust
let option = MillisecondOption{
    days_instead_of_years: true,
    ..MillisecondOption::default()
};
```
___
### License
MIT

##### Inspiration
This crate is inspired by `pretty-ms` npm package.
