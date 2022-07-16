# The Rust Programming Language
Code produced while following "The Rust Programming Language" book by Steve Klabnik.

The following `launch.json` snippet has been tweaked to make it possible to debug `main.rs` independently of where you are in the project. It `cargo build`s before launching the debugger.

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug",
      "program": "${workspaceFolder}/target/debug/the-rust-programming-language",
      "args": [],
      "cwd": "${workspaceFolder}",
      "preLaunchTask": "cargo build"
    }
  ]
}
```

## Run

All these approaches should work similarly:
* Ctrl + Shift + B (if using VSCode on Ubuntu or Windows)
* F5
* Running the commands `$ cargo build` and `$ cargo run`

## TODO

[Book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) chapters:
- [x] 4. Understanding Ownership
- [x] 5. Using Structs to Structure Related Data
- [x] 6. Enums and Pattern Matching
- [X] 7. Managing Growing Projects with Packages, Crates, and Modules
- [X] 8. Common Collections
- [X] 9. Error Handling
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