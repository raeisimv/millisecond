# Millisecond crate

A better way to format and display duration, which converts `33023448000ms` to `1y 17d 5h 10m 48s` or relatively
timestamp of`about a year ago`.

## Install

In your Rust project's root directory run:

```shell
$ cargo add millisecond
```

## Example

```rust
use millisecond::prelude::*;

fn main() {
    // Obtain a duration instance
    let dur = core::time::Duration::from_millis(33_023_448_000);

    println!("{}", dur.pretty());
    // displays: 1y 17d 5h 10m 48s

    println!("{}", dur.pretty_with(MillisecondOption::long()));
    // displays: 1 year 17 days 5 hours 10 minutes 48 seconds

    println!("{}", dur.relative());
    // displays: about a year ago

    // the previous solution still works
    let ms = Millisecond::from_millis(33_023_448_000);
    println!("{}", ms.pretty());
    // displays: 1y 17d 5h 10m 48s
}
```

## Options
All options are represented by the `MillisecondOption` struct; which has the following fields:

**`dominant_only`**\
Default: `false`\
Example: `1y 2d` -> `1y`\
Displays the most dominant part only (the most left part).

**`days_instead_of_years`**\
Default: `false`\
Example: `1y 1d` -> `366d`\
Displays time durations in days rather than converting them into years.

**`seconds`**\
Default: `Separate`\
Example: `1 second 2 milliseconds` -> `1.2 seconds`\
An enum that determines whether to display seconds and Milliseconds.
  + `Hide`: Hides seconds and milliseconds.
  + `Separate`: Separates seconds and milliseconds into two single digits. Example: `1s 2ms`
  + `Combine`: Combines seconds and milliseconds into a single float value. Example: `1.2s`
  + `CombineWith`: Combines with custom options of:
    + `precision`: the number of digits to show for the milliseconds part. Default: `1`
    + `fixed_width`: whether milliseconds should be displayed with a fixed width. Default: `false`

**`format_sub_milliseconds`**\
Default: `false`\
Example: `1s 2ms` -> `1s 2ms 3µs 4ns`\
Displays and formats microseconds and nanoseconds if present.

**`long`**\
Default: `false`\
Example: `2 years 3 days 1 hour`\
Displays full and descriptive labels for time units, such as `years` instead of abbreviated forms like `y`.

***

### Day of Week
Calculating the weekday could be easy if the duration was calculated from a known epoch. The `weekday` function
is implemented to convert the duration into the proper weekday value (enum).

```rust
use millisecond::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("today: {}", dur.weekday());
}
```

***

### License

[MIT](./LICENSE-MIT)

***

### Inspiration

This crate is inspired by `pretty-ms` npm package.
