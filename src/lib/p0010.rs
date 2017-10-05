// Problem 10
// Summation of primes

extern crate prime;

// Sum ths primes below the bound
fn sum_primes_below(bound: i64) -> i64 {
    let prime_list = prime::sieve_primes(bound);
    prime_list.iter().sum()
}


pub fn solve() -> String {
    sum_primes_below(2000000).to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_small() {
        assert_eq!(super::sum_primes_below(10), 17);
    }
}
