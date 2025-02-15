# Millisecond crate
A better way to format and display time, which converts `33023448000ms` to `1y 17d 5h 10m 48s`

### Install
In your Rust project root directory run:

```shell
$ cargo add millisecond
```

### Example
```rust
use millisecond::prelude::*;

fn main() {
    let ms = Millisecond::from_millis(33_023_448_000);

    println!("display: {ms}");
    // display: 1y 17d 5h 10m 48s

    println!("pretty: {}", ms.pretty());
    // pretty: 1y 17d 5h 10m 48s

    println!("pretty_with: {}", ms.pretty_with(&MillisecondOption::long()));
    // pretty_with: 1 year 17 days 5 hours 10 minutes 48 seconds

    // Also works on Duration instance directly
    let dur = core::time::Duration::from_millis(33_023_448_000);
    println!("dur: {}", dur.pretty());
    // dur: 1y 17d 5h 10m 48s
}

```

### License
MIT

##### Inspiration
This crate is inspired by `pretty-ms` npm package.
