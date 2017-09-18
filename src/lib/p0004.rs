// Problem 4
// Largest palindrome product

// Checks if n is a palindrome when written with no leading zeros
fn is_palindrome(n: i32) -> bool {
    let mut reverse_n = 0;
    let mut remain_n = n;
    while remain_n > 0 {
        reverse_n = reverse_n * 10 + remain_n % 10;
        remain_n /= 10;
    }
    (reverse_n == n)
}

// Finds the largest palindrome that is the product of 
// two (num_digits)-long numbers
fn palindrome_product(num_digits: u32) -> i32 {
    let base: i32 = 10;
    let lower_bound: i32 = base.pow(num_digits - 1);
    let upper_bound: i32 = base.pow(num_digits);
    let mut largest_product = 0;
    let mut factor1 = upper_bound - 1;
    while factor1 >= lower_bound && factor1 * (upper_bound - 1) > largest_product {
        for factor2 in (lower_bound..upper_bound).rev() {
            let product = factor1 * factor2;
            if is_palindrome(product) {
                if product > largest_product {
                    largest_product = product;
                }
                break;
            }
        }
        factor1 -= 1;
    }
    largest_product
}

pub fn solve() -> String {
    palindrome_product(3).to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_two_digits() {
        assert_eq!(super::palindrome_product(2), 9009);
    }
}
