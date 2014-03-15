#[allow(deprecated_owned_vector)]; // for test crate
extern crate test;

use std::iter::AdditiveIterator;
use std::vec_ng::Vec;

fn prob2(value_cap: u64) -> u64 {
    // Here we can be clever and generate only the values we need because
    // we have a custom Iterator.
    Fibonacci::new()
        .take_while(|&num| num < value_cap)
        .filter(|&num| num % 2 == 0)
        .sum()
}

struct Fibonacci {
    current: u64,
    last: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { current: 1, last: 1 }
    }
}

impl Iterator<u64> for Fibonacci {
    fn next(&mut self) -> Option<u64> {
        let res = self.current;
        self.current += self.last;
        self.last = self.current - self.last;
        Some(res)
    }
}

#[cfg(test)]
fn fibonacci(num_terms: u64) -> Vec<u64> {
    Fibonacci::new().take(num_terms as uint).collect()
}

#[cfg(not(test))]
fn main() {
    println!("{}", prob2(4e6 as u64));
}

#[cfg(test)]
fn check_prob2_impl(prob2_impl : fn(u64) -> u64) {
    check_prob2_regular(prob2_impl);
    check_prob2_zero(prob2_impl);
}

#[cfg(test)]
fn check_prob2_regular(prob2_impl : fn(u64) -> u64) {
    let value_cap = 90;
    assert!(prob2_impl(value_cap) == 44);
}

#[cfg(test)]
fn check_prob2_zero(prob2_impl : fn(u64) -> u64) {
    assert!(prob2_impl(0) == 0);
}

#[test]
fn test_prob2() {
    check_prob2_impl(prob2);
}

#[cfg(test)]
fn check_fibonacci_impl(fibonacci_impl : fn (u64) -> Vec<u64>) {
    check_fibonacci_empty(fibonacci_impl);
    check_fibonacci_regular(fibonacci_impl);
}

#[cfg(test)]
fn check_fibonacci_regular(fibonacci_impl : fn (u64) -> Vec<u64>) {
    assert!(fibonacci_impl(10) == vec!(1, 2, 3, 5, 8, 13, 21, 34, 55, 89));
}

#[cfg(test)]
fn check_fibonacci_empty(fibonacci_impl : fn (u64) -> Vec<u64>) {
    assert!(fibonacci_impl(0).len() == 0);
}

#[test]
fn test_fibonacci() {
    check_fibonacci_impl(fibonacci);
}

#[cfg(test)]
fn bench_fibonacci_impl(fibonacci_impl : fn(u64) -> Vec<u64>) {
    fibonacci_impl(1000);
}

#[bench]
fn bench_fibonacci(b: &mut test::BenchHarness) {
    b.iter(|| bench_fibonacci_impl(fibonacci))
}

#[cfg(test)]
fn bench_prob2_impl(prob2_impl: fn(u64) -> u64) {
    prob2_impl(1e18 as u64);
}

#[bench]
fn bench_prob2(b: &mut test::BenchHarness) {
    b.iter(|| bench_prob2_impl(prob2))
}
