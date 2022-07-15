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

## Exercises Workflow

* Ctrl + Shift + B
* F5 (runs existing build)

## TODO

Finish the [book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html):
- [x] 4. Understanding Ownership
- [x] 5. Using Structs to Structure Related Data
- [x] 6. Enums and Pattern Matching
- [X] 7. Managing Growing Projects with Packages, Crates, and Modules
- [ ] 8. Common Collections
- [ ] 9. Error Handling
- [ ] 10. Generic Types, Traits, and Lifetimes
- [ ] 11. Writing Automated Tests
- [ ] 12. An I/O Project: Building a Command Line Program
- [ ] 13. Functional Language Features: Iterators and Closures
- [ ] 14. More about Cargo and Crates.io
- [ ] 15. Smart Pointers
- [ ] 16. Fearless Concurrency
- [ ] 17. Object Oriented Programming Features of Rust
- [ ] 18. Patterns and Matching
- [ ] 19. Advanced Features
- [ ] 20. Final Project: Building a Multithreaded Web Server