// Problem 5
// Smallest multiple

use std::cmp;

// Finds the greatest common factor of nonnegative integers a and b
fn gcd(a: i64, b: i64) -> i64 {
    let mut max_num = cmp::max(a, b);
    let mut min_num = cmp::min(a, b);
    if min_num == 0 {
        return max_num;
    }
    let mut remainder = max_num % min_num;
    while remainder > 0 {
        max_num = min_num;
        min_num = remainder;
        remainder = max_num % min_num;
    }
    min_num
}

// Finds the smallest number divisible by the integers 1..n
fn divisible_multiple(n: i64) -> i64 {
    let mut multiple: i64 = 1;
    for factor in 1..(n + 1) {
        let current_gcd = gcd(factor, multiple);
        // a * b = gcd(a, b) * lcm(a, b)
        multiple *= factor / current_gcd;
    }
    multiple
}

pub fn solve() -> String {
    divisible_multiple(20).to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_ten() {
        assert_eq!(super::divisible_multiple(10), 2520);
    }
}
