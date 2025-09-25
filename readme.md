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

### format
Determines whether to format the final duration string either short, long, or colon separated.\
Example: `1h 2m 3s`\
Type: `OutputFormat` enum\
Default: `Short`
- `Short`: Uses short labels. Example: `1y 17d 5h 10m 48s`
- `Long`: Uses long labels. Example: `1 year 17 days 5 hours 10 minutes 48 seconds`
- `Colon`: Uses no labels but colon separated values. Example: `1:17:05:10:48`

### seconds
Determines whether to display seconds and milliseconds; combine or separate them.\
Example: `1 second 2 milliseconds`\
Type: `SecondsFormat` enum\
Default: `Separate`\
- `Hide`: Hides seconds and milliseconds.
- `Separate`: Separates seconds and milliseconds into two single digits. Example: `1s 2ms`
- `Combine`: Combines seconds and milliseconds into a single float value. Example: `1.2s`
- `CombineWith`: Combines with custom options of:
  - `precision`: the number of digits to show for the milliseconds part. Default: `1`
  - `fixed_width`: whether milliseconds should be displayed with a fixed width. Default: `false`

### dominant_only
Determines whether displays the most dominant part only (the most left part). The `unit_count` flag takes precedence over this setting.\
Example: `1y 2d` -> `1y`\
Type: `bool`\
Default: `false`

### unit_count
Determines the maximum number of units to display in the formatted string (from years towards nanoseconds).
Default is `None`, which means all units will be displayed.
This flag takes precedence over the `dominant_only` setting.\
Example: `1y 2d 3h` -> `1y 2d`\
Type: `Option<usize>`\
Default: `None`

### days_instead_of_years
Determines whether displays time durations in days rather than converting them into years.\
Example: `1y 1d` -> `366d`\
Type: `bool`\
Default: `false`

### format_sub_milliseconds
Determines whether displays and formats microseconds and nanoseconds if present.\
Example: `1s 2ms 3µs 4ns`\
Type: `bool`\
Default: `false`

### separator
Determines the separator between units. The default value is a `Space` but can be overridden, e.g. in colon notation `Default` gets converted to `Colon` but `Space` stands persisted. \
Example: `1y 2d` -> `1y-2d`\
Type: `Separator` enum\
Default: `Separator::Default`
- `Default`: Displays units a space as separator. Example: `1y 2d`
- `None`: Displays units without a separator. Example: `1y2d`
- `Space`: Displays units with a space as separator. Example: `1y 2d`
- `SingleColon`: Displays units with a colon as separator. Example: `1y:2d`
- `Comma`: Displays units with a comma as separator. Example: `1y,2d`
- `Dash`: Displays units with a dash as separator. Example: `1y-2d`
- `Slash`: Displays units with a slash as separator. Example: `1y/2d`
- `Pipe`: Displays units with a pipe as separator. Example: `1y|2d`
- `Underscore`: Displays units with an underscore as separator. Example: `1y_2d`


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
