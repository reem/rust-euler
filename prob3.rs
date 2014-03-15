#[allow(deprecated_owned_vector)];
extern crate test;

use std::vec_ng::Vec;
use common::Primes;
mod common;

type Prob3Impl       = fn(u64) -> Option<u64>;
type PrimeFactorizer = fn(u64) -> Option<Vec<u64>>;

#[cfg(not(test))]
fn main() {
    println!("{}", prob3(600851475143).unwrap());
}

fn prob3(val_to_factor: u64) -> Option<u64> {
    prime_factors(val_to_factor).map(|x| *x.get(x.len() - 1))
}

fn unsafe_factor(val_to_factor: u64) -> Vec<u64> {
    // Slow but fast enough for now.
    let mut factors = vec!();
    let mut working_val = val_to_factor;
    loop {
        for prime in Primes::new() {
            if working_val % prime == 0 {
                factors.push(prime);
                working_val = working_val / prime;
                break;
            }
        }
        if working_val == 1 {
            break;
        }
    }
    factors
}

fn prime_factors(val_to_factor: u64) -> Option<Vec<u64>> {
    match val_to_factor {
        // Handle special cases then pass sanitized input.
        0 => None,
        1 => None,
        v => Some(unsafe_factor(v))
    }
}

#[cfg(test)]
fn check_prob3_impl(prob3_impl: Prob3Impl) {
    check_prob3_regular(prob3_impl);
    check_prob3_corner_cases(prob3_impl);
}

#[cfg(test)]
fn check_prob3_regular(prob3_impl: Prob3Impl) {
    assert!(prob3_impl(13195) == Some(29));
}

#[cfg(test)]
fn check_prob3_corner_cases(prob3_impl: Prob3Impl) {
    assert!(prob3_impl(0) == None);
    assert!(prob3_impl(1) == None);
    assert!(prob3_impl(2) == Some(2u64));
}

#[cfg(test)]
fn check_prime_factorizer(prime_factorizer: PrimeFactorizer) {
    check_prime_factorizer_corner_cases(prime_factorizer);
    check_prime_factorizer_large(prime_factorizer);
}

#[cfg(test)]
fn check_prime_factorizer_corner_cases(prime_factorizer: PrimeFactorizer) {
    assert!(prime_factorizer(0) == None);
    assert!(prime_factorizer(1) == None);
    assert!(prime_factorizer(2) == Some(vec!(2u64)));
}

#[cfg(test)]
fn check_prime_factorizer_large(prime_factorizer: PrimeFactorizer) {
    let mut primes = prime_factorizer(1200).unwrap();
    primes.as_mut_slice().sort();
    assert!(primes == vec!(2u64, 2, 2, 2, 3, 5, 5));
}

#[test]
fn test_prime_factorizer() {
    check_prime_factorizer(prime_factors);
}

#[test]
fn test_prob3() {
    check_prob3_impl(prob3);
}

#[bench]
fn bench_prob3(b: &mut test::BenchHarness) {
    b.iter(|| { let _ = prob3(2345678908); }); // Secure random number ;)
}
