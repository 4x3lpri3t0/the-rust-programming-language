# the-rust-programming-language
Code produced while following "The Rust Programming Language" book by Steve Klabnik.

`launch.json`:

```json
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            // To create new projects and debug from current main.rs directly
            "program": "${file}/../../target/debug/the-rust-programming-language",
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

If breakpoints are not being attached, you need to `cargo b`.

When a new subproject (e.g.: `n-exercise`) is created, you still need to `cd n-exercise` and `cargo b` there before being able to debug this new `main.rs` file.
Alternatively: `cargo b --manifest-path n-exercise/Cargo.toml`

- [] Change the approach to use libraries instead