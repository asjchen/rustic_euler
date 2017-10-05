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
