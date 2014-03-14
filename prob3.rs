extern mod extra;

use std::iter;
use std::f64;

#[cfg(test)]
use extra::test;

type prob3_implT       = fn(u64) -> Option<u64>;
type prime_factorizerT = fn(u64) -> Option<~[u64]>;
type prime_generatorT  = fn(u64) -> ~[u64];

fn main() {
    println!("{}", prob3(600851475143).unwrap());
}

fn prob3(val_to_factor: u64) -> Option<u64> {
    prime_factors(val_to_factor).map(|x| *x.last())
}

fn unsafe_factor(val_to_factor: u64) -> ~[u64] {
    // Slow but fast enough for now.
    let mut factors = ~[];
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

fn prime_factors(val_to_factor: u64) -> Option<~[u64]> {
    match val_to_factor {
        // Handle special cases then pass sanitized input.
        0 => None,
        1 => None,
        v => Some(unsafe_factor(v))
    }
}

struct Primes {
    primes: ~[u64]
}

impl Primes {
    fn new() -> Primes {
        Primes { primes: ~[2] }
    }
}

impl Iterator<u64> for Primes {
    fn next(&mut self) -> Option<u64> {
        let res = self.primes[self.primes.len() - 1];
        let next_prime = iter::count(res+1, 1u64)
            .filter(|&x| {
                let x_sqrt = f64::sqrt(x as f64) as u64 + 1;
                self.primes.iter()
                    .take_while(|&p| p < &x_sqrt)
                    .all(|&p| x % p != 0) })
            .next().unwrap();
        self.primes.push(next_prime);
        Some(res)
    }
}

#[cfg(test)] // This function is only used in the test suite for this problem.
fn first_n_primes(n: u64) -> ~[u64] {
    // It is usually more efficient to use Primes as an iterator instead of
    // getting all the results immediately.
    Primes::new().take(n as uint).collect()
}

#[cfg(test)]
fn check_prob3_impl(prob3_impl: prob3_implT) {
    check_prob3_regular(prob3_impl);
    check_prob3_corner_cases(prob3_impl);
}

#[cfg(test)]
fn check_prob3_regular(prob3_impl: prob3_implT) {
    assert!(prob3_impl(13195) == Some(29));
}

#[cfg(test)]
fn check_prob3_corner_cases(prob3_impl: prob3_implT) {
    assert!(prob3_impl(0) == None);
    assert!(prob3_impl(1) == None);
    assert!(prob3_impl(2) == Some(2u64));
}

#[cfg(test)]
fn check_prime_generator(prime_generator: prime_generatorT) {
    for num in range(0u, 5) {
        assert!(prime_generator(num as u64).len() == num);
    }
    assert!(prime_generator(5) == ~[2u64, 3, 5, 7, 11]);
}

#[cfg(test)]
fn check_prime_factorizer(prime_factorizer: prime_factorizerT) {
    check_prime_factorizer_corner_cases(prime_factorizer);
    check_prime_factorizer_large(prime_factorizer);
}

#[cfg(test)]
fn check_prime_factorizer_corner_cases(prime_factorizer: prime_factorizerT) {
    assert!(prime_factorizer(0) == None);
    assert!(prime_factorizer(1) == None);
    assert!(prime_factorizer(2) == Some(~[2u64]));
}

#[cfg(test)]
fn check_prime_factorizer_large(prime_factorizer: prime_factorizerT) {
    let mut primes = prime_factorizer(1200).unwrap();
    primes.sort();
    assert!(primes == ~[2u64, 2, 2, 2, 3, 5, 5]);
}

#[test]
fn test_prime_generator() {
    check_prime_generator(first_n_primes);
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
fn bench_prime_generator(b: &mut test::BenchHarness) {
    b.iter(|| { let _ = first_n_primes(10000); });
}

#[bench]
fn bench_prob3(b: &mut test::BenchHarness) {
    b.iter(|| { let _ = prob3(2345678908); }); // Secure random number ;)
}
