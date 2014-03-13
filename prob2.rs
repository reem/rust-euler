use std::iter::AdditiveIterator;

fn prob2_simple(value_cap: u64) -> u64 {
    let mut result = 0;
    // We have to arbitrarily pick a very high number
    // but not too high because this is not lazy.
    let fibs = fibonacci_simple(1e6 as u64);
    for num in fibs.iter() {
        if *num > value_cap {
            break;
        }
        if *num % 2 == 0 {
            result += *num;
        }
    }
    result
}

fn fibonacci_simple(num_terms: u64) -> ~[u64] {
    return match num_terms {
        0 => ~[],
        1 => ~[1],
        2 => ~[1, 2],
        _ => { let mut fib = ~[1, 2];
               for i in range(0, num_terms - 2) {
                   fib.push(fib[i+1] + fib[i]);
               }
               fib
              }
    }
}

fn prob2_smart(value_cap: u64) -> u64 {
    // Here we can be clever and generate only the values we need because
    // we have a custom Iterator.
    let f = fibonacci_smart(value_cap);
    f.iter()
        .filter(|&num| num % 2 == 0)
        .map(|&u| u)
        .sum()
}

struct Fibonacci {
    priv current: u64,
    priv last: u64,
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

fn fibonacci_smart(value_cap: u64) -> ~[u64] {
    Fibonacci::new().take_while(|&num| num < value_cap).collect()
}

fn main() {
    println!("{}", prob2_simple(4e6 as u64));
    println!("{}", prob2_smart(4e6 as u64));
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
fn test_prob2_simple() {
    check_prob2_impl(prob2_simple);
}

#[test]
fn test_prob2_smart() {
    check_prob2_impl(prob2_smart);
}

#[cfg(test)]
fn check_fibonacci_impl(fibonacci_impl : fn (u64) -> ~[u64]) {
    check_fibonacci_empty(fibonacci_impl);
    check_fibonacci_regular(fibonacci_impl);
}

#[cfg(test)]
fn check_fibonacci_regular(fibonacci_impl : fn (u64) -> ~[u64]) {
    assert!(fibonacci_impl(10) == ~[1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
}

#[cfg(test)]
fn check_fibonacci_empty(fibonacci_impl : fn (u64) -> ~[u64]) {
    assert!(fibonacci_impl(0).len() == 0);
}

#[test]
fn test_fibonacci_simple() {
    check_fibonacci_impl(fibonacci_simple);
}

#[test]
fn test_fibonacci_smart() {
    assert!(fibonacci_smart(0).len() == 0);
    assert!(fibonacci_smart(2) == ~[1u64]);
    assert!(fibonacci_smart(3) == ~[1, 2]);
    assert!(fibonacci_smart(10) == ~[1, 2, 3, 5, 8u64]);
}
