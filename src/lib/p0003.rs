// Problem 3
// Largest prime factor

extern crate prime;

// Finds the largest prime factor of n
fn largest_prime_factor(n: i64) -> i64 {
    let prime_list = prime::sieve_primes((n as f64).sqrt() as i64);
    let mut remain_num = n;
    let mut largest_prime: i64 = 1;

    for prime in prime_list {
        while remain_num % prime == 0 {
            if largest_prime < prime {
                largest_prime = prime;
            }
            remain_num /= prime;
        }
    }
    if remain_num > largest_prime {
        largest_prime = remain_num;
    }
    largest_prime
}


pub fn solve() -> String {
    largest_prime_factor(600851475143).to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_small() {
        assert_eq!(super::largest_prime_factor(13195), 29);
    }
}
