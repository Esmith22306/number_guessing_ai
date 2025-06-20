# Overview

This project was created to improve my understanding of systems programming with Rust. My goal was to build a simple binary search algorithm that implements core Rust concepts like ownership, conditionals, loops, and data structures. Working on this helped me become more familiar with the Rust syntax, error handling, and memory safety features.

The software is a command-line program that plays a number guessing game. The user chooses a number in their mind, and the AI uses a binary search strategy to guess it efficiently. The program accepts feedback from the user—such as "too high" or "too low"—and adjusts its range accordingly until it finds the correct number. It also keeps track of guess history using a vector and uses slicing to display the last three guesses.


[Software Demo Video](https://youtu.be/jRcXGMJnGpI)

# Development Environment

- **IDE/Editor:** Visual Studio Code with Rust Analyzer extension
- **Build Tool:** Cargo (Rust’s built-in build and package manager)
- **Terminal:** Windows Terminal / PowerShell

**Programming Language:**
- Rust

**Libraries/Dependencies:**
- No third-party libraries used; built entirely with the Rust standard library.

# Useful Websites

{Make a list of websites that you found helpful in this project}

 [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- [Stack Overflow](https://stackoverflow.com/)
- [Rust Playground](https://play.rust-lang.org/)
- [Binary Search](https://www.geeksforgeeks.org/dsa/binary-search/)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}

- Allow the computer think of a number and the user guesses it.
- Add file I/O to store guess statistics and user history.
- Add some random variability to the computers guesses.
- Create a web-based version using WebAssembly (WASM) for broader accessibility
