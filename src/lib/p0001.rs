// Problem 1
// Multiples of 3 and 5

extern crate series;

// Sums the multiples of 3 or 5 below the bound
fn sum_all_multiples(bound: i32) -> i32 {
    let sum_threes = series::sum_first_integers((bound - 1) / 3) * 3;
    let sum_fives = series::sum_first_integers((bound - 1) / 5) * 5;
    let sum_fifteens = series::sum_first_integers((bound - 1) / 15) * 15;
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
