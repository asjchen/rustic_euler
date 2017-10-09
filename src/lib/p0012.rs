// Problem 12
// Highly divisible triangular number

extern crate prime;

// Finds the first triangular number with target_factors distinct factors
fn triangular_num_with_divisors(target_factors: i32) -> i64 {
    let float_target = target_factors as f64;
    let theoretical_bound = (float_target * float_target.ln()).ceil() as i64;
    let prime_list = prime::sieve_primes(theoretical_bound);
    let mut n: i64 = 1;
    let mut num_factors = 1;
    while num_factors <= target_factors {
        n += 1;
        // Note that a triangular number is of the form n(n+1)/2, 
        // and since gcd(n, n+1) = 1, the number of factors is 
        // either the product of the numbers of factors of n/2 and n+1
        // or of n and (n+1)/2
        if n % 2 == 0 {
            let num_comp1 = prime::num_divisors_given_primes(n / 2, &prime_list);
            let num_comp2 = prime::num_divisors_given_primes(n + 1, &prime_list);
            num_factors = num_comp1 * num_comp2;
        }
        else {
            let num_comp1 = prime::num_divisors_given_primes(n, &prime_list);
            let num_comp2 = prime::num_divisors_given_primes((n + 1) / 2, &prime_list);
            num_factors = num_comp1 * num_comp2;
        }
    }
    n * (n + 1) / 2
}

pub fn solve() -> String {
    triangular_num_with_divisors(500).to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_small() {
        assert_eq!(super::triangular_num_with_divisors(5), 28);
    }
}
