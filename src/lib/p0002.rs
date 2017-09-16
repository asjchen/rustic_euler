// Problem 2
// Even Fibonacci numbers

// Sums the even Fibonacci numbers not exceeding the bound
fn sum_even_fib(bound: i64) -> i64 {
	// Every third Fibonacci number is even
	let mut small_odd: i64;
	let mut large_odd: i64 = 1;
	let mut even: i64 = 2;
	let mut sum: i64 = 0;
	while even <= bound {
		sum += even;
		small_odd = large_odd + even;
		large_odd = even + small_odd;
		even = small_odd + large_odd;
	}
	sum
}

pub fn solve() -> String {
    sum_even_fib(4000000).to_string()
}
