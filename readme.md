# Millisecond crate
A better way to format and display time, which converts `33023448000ms` to `1y 17d 5h 10m 48s`

### Install
In your Rust project root directory run:

```shell
$ cargo add millisecond
```

### Example
```rust
use millisecond::Millisecond;

fn main() {
    let ms = Millisecond::from_millis(33023448000);

    print!("display: {ms}");
    // display: 1y 17d 5h 10m 48s

    print!("short: {}", ms.to_short_string());
    // short: 1y 17d 5h 10m 48s

    print!("long: {}", ms.to_long_string());
    // long: 1 year 17 days 5 hours 10 minutes 48 seconds

    assert_eq!(ms, Millisecond {
        years: 1,
        days: 17,
        hours: 5,
        minutes: 10,
        seconds: 48,
        millis: 0,
        micros: 0,
        nanos: 0,
    });
}
```

### License
MIT

##### Inspiration
This crate is inspired by `pretty-ms` npm package.