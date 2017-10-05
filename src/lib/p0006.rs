// Problem 5
// Largest palindrome product

extern crate series;

fn squares_difference(n: i32) -> i32 {
    series::sum_first_integers(n).pow(2) - series::sum_first_squares(n)
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
