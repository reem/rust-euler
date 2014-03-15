extern crate test;

use std::iter;
use std::f64;
use std::vec_ng::Vec;

type PrimeGenerator  = fn(u64) -> Vec<u64>;

pub struct Primes {
    priv primes: Vec<u64>
}

impl Primes {
    pub fn new() -> Primes {
        Primes { primes: vec!(2) }
    }
}

impl Iterator<u64> for Primes {
    fn next(&mut self) -> Option<u64> {
        let &res = self.primes.get(self.primes.len() - 1);
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
fn first_n_primes(n: u64) -> Vec<u64> {
    // It is usually more efficient to use Primes as an iterator instead of
    // getting all the results immediately.
    Primes::new().take(n as uint).collect()
}

#[cfg(test)]
fn check_prime_generator(prime_generator: PrimeGenerator) {
    for num in range(0u, 5) {
        assert!(prime_generator(num as u64).len() == num);
    }
    assert!(prime_generator(5) == vec!(2u64, 3, 5, 7, 11));
}

#[test]
fn test_prime_generator() {
    check_prime_generator(first_n_primes);
}

#[bench]
fn bench_prime_generator(b: &mut test::BenchHarness) {
    b.iter(|| { let _ = first_n_primes(10000); });
}
