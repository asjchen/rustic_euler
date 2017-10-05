// Problem 9
// Special Pythagorean triplet

// Finds a Pythagorean triple whose sum is equal to the parameter
fn find_pythagorean_sum(sum: i32) -> i32 {
    // Each triplet is of the form (m^2 - n^2, 2mn, m^2 + n^2),
    // which means the sum is of the form 2m(m + n) where m > n
    assert!(sum % 2 == 0);
    let min_m = ((sum / 4) as f64).sqrt() as i32;
    let max_m = ((sum / 2) as f64).sqrt() as i32;
    for m in min_m..(max_m + 1) {
        if (sum / 2) % m == 0 {
            let n = sum / (2 * m) - m;
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            return a * b * c;
        }
    }
    panic!("No Pythagorean triplets with sum {}", sum);
}


pub fn solve() -> String {
    find_pythagorean_sum(1000).to_string()
}
