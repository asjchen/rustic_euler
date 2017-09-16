// Problem 3
// Largest prime factor

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

// Finds the largest prime factor of n
fn largest_prime_factor(n: i64) -> i64 {
    let prime_list = sieve_primes((n as f64).sqrt() as i64);
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
