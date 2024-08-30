# time_craft

`time_craft` is a Rust library designed for manipulating times without dealing with dates. Whether you need to add or subtract minutes, handle hour and minute rollovers, or compare different times, `time_craft` makes it easy.

## Features

- **Time Manipulation**: Add or subtract minutes from a given time.
- **Rollover Support**: Automatically adjusts hours and minutes when values exceed their limits.
- **Equality Comparison**: Compare two clock instances to check if they represent the same time.
- **String Formatting**: Provides a user-friendly string representation of the time.

## Installation

Add `time_craft` to your `Cargo.toml`:

```toml
[dependencies]
time_craft = "0.1.0"
```

Then, run `cargo build` to download and compile the library.

## Usage

Here’s a simple example of how to use the `time_craft` library:

```rust
use time_craft::Clock;

fn main() {
    let clock = Clock::new(10, 0);
    let updated_clock = clock.add_minutes(125);
    println!("{}", updated_clock); // Outputs "12:05"
}

```

## Running Tests

The project includes a comprehensive set of tests. You can run them with the following command:

```bash
cargo test
```
