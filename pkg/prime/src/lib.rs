// Prime Utility Functions

// Runs the Sieve of Eratosthenes to find primes below the bound
pub fn sieve_primes(bound: i64) -> Vec<i64> {
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

// Find the number of divisors of n given a list of primes
pub fn num_divisors_given_primes(n: i64, prime_list: &Vec<i64>) -> i32 {
    let mut count = 1;
    let mut remain = n;
    let mut prime_idx = 0;
    while prime_idx < prime_list.len() && prime_list[prime_idx] * prime_list[prime_idx] <= remain {
        let mut prime_power = 0;
        while remain % prime_list[prime_idx] == 0 {
            prime_power += 1;
            remain /= prime_list[prime_idx];
        }
        count *= prime_power + 1;
        prime_idx += 1;
    }
    if remain > 1 {
        count *= 2;
    }
    count
}
