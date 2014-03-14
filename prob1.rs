extern mod extra;

use std::iter::Iterator;
use std::iter::AdditiveIterator;

#[cfg(test)]
use extra::test;

fn simple_prob1(limit: int) -> int {
    // An imperative implementation.
    let mut sum = 0;
    for i in range(1, limit) {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn higher_order_prob1(limit: int) -> int {
    // A functional implementation.
    range(0, limit)
        .filter(|&n: &int| n % 3 == 0 || n % 5 == 0)
        .sum()
}

fn main() {
    println!("{}", simple_prob1(1000));
    println!("{}", higher_order_prob1(1000));
}

#[cfg(test)]
fn check_prob1(prob1_impl: fn(int) -> int) {
    check_prob1_regular(prob1_impl);
    check_prob1_empty(prob1_impl);
}

#[cfg(test)]
fn check_prob1_regular(prob1_impl : fn(int) -> int) {
    let test_limit = 10;
    let test_sum = 23;
    assert!(prob1_impl(test_limit) == test_sum);
}

#[cfg(test)]
fn check_prob1_empty(prob1_impl : fn(int) -> int) {
    assert!(prob1_impl(0) == 0)
}

#[test]
fn test_simple_prob1() {
    check_prob1(simple_prob1);
}

#[test]
fn test_higher_order_prob1() {
    check_prob1(higher_order_prob1);
}

#[cfg(test)]
fn bench_prob1_impl(prob1_impl : fn(int) -> int) {
    prob1_impl(100000);
}

#[bench]
fn bench_prob1_smart(b: &mut test::BenchHarness) {
    b.iter(|| bench_prob1_impl(higher_order_prob1));
}

#[bench]
fn bench_prob1_simple(b: &mut test::BenchHarness) {
    b.iter(|| bench_prob1_impl(simple_prob1));
}
