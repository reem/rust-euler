extern mod extra;

#[cfg(test)]
use extra::test;

fn main() {
    println!("{}", prob4(100, 999));
}

fn reverse(num: uint) -> uint {
    let mut reversed = 0;
    let mut n = num;
    while n > 0 {
        reversed = 10*reversed + n % 10;
        n = n / 10;
    }
    reversed
}

fn is_palindrome(num: uint) -> bool {
    reverse(num) == num
}

fn prob4(low_bound: uint, high_bound: uint) -> uint {
    let mut largest_palindrome = 0;
    let mut a = high_bound;
    while a >= low_bound {
        let mut b = high_bound;
        while b >= a {
            if a*b <= largest_palindrome { break; }
            if is_palindrome(a*b) { largest_palindrome = a*b; }

            b -= 1;
        }
        a -= 1;
    }
    largest_palindrome
}

#[test]
fn test_reverse() {
    assert!(reverse(123456) == 654321);
    assert!(reverse(1) == 1);
    assert!(reverse(101) == 101);
}

#[test]
fn test_is_palindrome() {
    assert!(!is_palindrome(10));
    assert!(is_palindrome(1001));
    assert!(is_palindrome(1));
}

#[test]
fn test_prob4() {
    assert!(prob4(10, 99) == 9009);
}

#[bench]
fn bench_is_palindrome_positive(b: &mut test::BenchHarness) {
    b.iter(|| { let _ = is_palindrome(906609); });
}

#[bench]
fn bench_is_palindrome_negative(b: &mut test::BenchHarness) {
    b.iter(|| { let _ = is_palindrome(100000); });
}

#[bench]
fn bench_prob4(b: &mut test::BenchHarness) {
    b.iter(|| { let _ = prob4(100, 999); });
}
