// Problem 7
// 10001st prime

extern crate prime;

// Finds the nth prime number
fn nth_prime(n: usize) -> i64 {
    // Theoretical bound is from the Prime Number Theorem, 
    // but we double it to account for the variance of pi(x)
    let theoretical_bound = ((n as f64) * (n as f64).ln()).ceil();
    let prime_list = prime::sieve_primes(2 * (theoretical_bound as i64));
    prime_list[n - 1]
}

pub fn solve() -> String {
    nth_prime(10001).to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_small() {
        assert_eq!(super::nth_prime(6), 13);
    }
}
