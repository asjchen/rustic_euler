# Rustic Euler
## Project Euler Solutions in Rust

This is a personal project to learn Rust by solving Project Euler problems. The problem statements are publicly available on the Project Euler website here: [https://projecteuler.net/archives](https://projecteuler.net/archives).

If you wish to solve the problems on your own, **do not** run this project or look at the source code.  You have been warned.

To run Rustic Euler, first build the project with Cargo:
```
cargo build
```
Then, solve all the problems solved by running:
```
./target/debug/run_problems
```
Many of the problems give small examples to clarify their problem statements (ex. "The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17." when the goal to find the sum of the primes below two million). To run the unit tests that confirm these small examples, run:
```
cargo test
```
