// Problem 1
// Multiples of 3 and 5

fn sum_first_integers(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn sum_multiples(bound: i32) -> i32 {
    let sum_threes = sum_first_integers((bound - 1) / 3) * 3;
    let sum_fives = sum_first_integers((bound - 1) / 5) * 5;
    let sum_fifteens = sum_first_integers((bound - 1) / 15) * 15;
    sum_threes + sum_fives - sum_fifteens
}

pub fn solve() -> String {
    sum_multiples(1000).to_string()
}
