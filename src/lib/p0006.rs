// Problem 5
// Largest palindrome product

// TODO: transfer this into a math library to share with p0001
// Sums the first n positive integers
fn sum_first_integers(n: i32) -> i32 {
    n * (n + 1) / 2
}

// Sums the first n positive squares
fn sum_first_squares(n: i32) -> i32 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn squares_difference(n: i32) -> i32 {
    sum_first_integers(n).pow(2) - sum_first_squares(n)
}

pub fn solve() -> String {
    squares_difference(100).to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_ten() {
        assert_eq!(super::squares_difference(10), 2640);
    }
}
