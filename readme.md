# for_match

A macro that simplifies matching on everything in an iterator.

```rust
#![feature(phase)]

#[phase(plugin)]
extern crate for_match;

fn main() {
    for_match!(range(0, 10u)
        0 => println!("none"),
        1 | 2 => println!("small"),
        x if x % 2 == 0 => println!("even"),
        _ => println!("other")
    )
}

```

