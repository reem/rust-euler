# Project Euler in Rust

## Problems Finished:
1  - 10 ... in progress

## Running
To run tests:
* Make a "tests" directory: `mkdir -p tests`
* Compile using rusts builtin support for tests: `rustc --test --out-dir probNUM.rs`
* Run the test executable: `./test/probNUM`

To run a specific problem:
* Make a "bin": `mkdir -p bin`
* Compile without tests and with optimization: `rustc --opt-level=3 --out-dir probNUM.rs`
* Run the problem: `./bin/probNUM`

## Contributing
* Solve a problem
* Write unit tests
* Write benchmarks
* Describe your approach in comments
* Send a PR

## Guidelines for Contributions
* Your approach should run relatively quickly.
* Use `#[test]` and `#[bench]` for your tests and benchmarks.
* The main function should solve the actual problem and print out only the
  solution.
* Please document your thought process unless painfully obvious.
