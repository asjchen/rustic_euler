// Problem 7
// 10001st prime

// TODO: move into a common math library
// Runs the Sieve of Eratosthenes to find primes below the bound
fn sieve_primes(bound: i64) -> Vec<i64> {
    let mut prime_list: Vec<i64> = Vec::new();
    let index_bound = (bound + 1) as usize;
    let mut is_prime: Vec<bool> = vec![true; index_bound];
    for num in 2..index_bound {
        if is_prime[num] {
            prime_list.push(num as i64);
            let multiplier_bound = (index_bound - 1) / num + 1;
            for multiplier in 2..multiplier_bound {
                is_prime[multiplier * num] = false;
            }
        }
    }
    prime_list
}

// Finds the nth prime number
fn nth_prime(n: usize) -> i64 {
    // Theoretical bound is from the Prime Number Theorem, 
    // but we double it to account for the variance of pi(x)
    let theoretical_bound = ((n as f64) * (n as f64).ln()).ceil();
    let prime_list = sieve_primes(2 * (theoretical_bound as i64));
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
