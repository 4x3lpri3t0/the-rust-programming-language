# TEST
# The Rust Programming Language
Code produced while following "The Rust Programming Language" book by Steve Klabnik.

To debug from current open main.rs file directly, use this `launch.json` snippet:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            
            "program": "${file}/../../target/debug/the-rust-programming-language",
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

If breakpoints are not being attached, you are missing a `cargo b`.

When creating a new subproject (e.g.: `n-exercise`), you still need to `cd n-exercise` and `cargo b` there before being able to debug this new `main.rs` file.
Alternatively: `cargo b --manifest-path n-exercise/Cargo.toml`

- [ ] Finish the book
- [ ] Change the approach to use libraries instead