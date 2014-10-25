# for_match

### A macro that simplifies matching on everything in an iterator.

Due to rustc bug [8372](https://github.com/rust-lang/rust/issues/8372),
code that used to look like:

```rust
for e in event_iter {
    match e {
        Render(args) => app.render(event_iter.window, &args),
        Update(args) => app.update(event_iter.window, &args),
        _ => { }
    }
}
```

became as ugly as

```rust
loop {
    let e = match event_iter.next() {
        Some(e) => e,
        None => { break; }
    };

    match e {
        Render(_args) => app.render(event_iter.window, &_args),
        Update(_args) => app.update(event_iter.window, &_args),
        _ => {  }
    }
}
```

This macro generates the inner match and marries it with the explicit match
for cases where the user simply wants to match on the elements in an iterator.

## Example
^code(examples/basic.rs)

