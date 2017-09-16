// Problem 1
// Multiples of 3 and 5

// Sums the first n positive integers
fn sum_first_integers(n: i32) -> i32 {
    n * (n + 1) / 2
}

// Sums the multiples of 3 or 5 below the bound
fn sum_all_multiples(bound: i32) -> i32 {
    let sum_threes = sum_first_integers((bound - 1) / 3) * 3;
    let sum_fives = sum_first_integers((bound - 1) / 5) * 5;
    let sum_fifteens = sum_first_integers((bound - 1) / 15) * 15;
    sum_threes + sum_fives - sum_fifteens
}

pub fn solve() -> String {
    sum_all_multiples(1000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_10() {
    	assert_eq!(super::sum_all_multiples(10), 23);
    }
}
