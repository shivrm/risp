# RISP

**Warning: The language is a work in progress. It's lacks many features and might not be stable.**

**RISP** is an interpreted LISP-like language. It's written in Rust.

# Features
Checked entries have been implemented.
- [x] Integers and Strings
- [x] Floats
- [ ] Better error reporting
- [x] Operators
- [ ] Variables
- [ ] `if`-`else` expressions
- [ ] `while` and `for` loops
- [ ] Functions
- [ ] Macros
- [ ] Classes

# Usage
Rust must be installed for compiling the code. The latest version is recommended. Installing `cargo` is also recommended.

1. Clone the repo
    ```
    git clone https://github.com/shivrm/risp
    ```
    GitHub also lets you download the source code [as a ZIP](https://github.com/shivrm/risp/archive/refs/heads/main.zip).

2. Navigate to project folder
    ```
    cd risp
    ```

3. Build and run with cargo
    - Run without any arguments to start the REPL. This lets you interactively run statements - one at a time.
        ```
        cargo run
        ```


    - To execute code from a file, give the file path as a command-line argument. Replace `<file>` with the path to the file:
        ```
        cargo run <file>
        ```

    - Giving `bench` as a command-line argument will benchmark the lexer. Note that this might be performance intensive.
        ```
        cargo run bench
        ```

# Examples
See `scripts/example.risp` for an example RISP script. Run it with
```
cargo run scripts/example.risp
```

# License
RISP is licensed under the MIT license. See [LICENSE.md](LICENSE.md)