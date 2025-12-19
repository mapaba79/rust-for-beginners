use std::io;

fn main() {
    let mut input = String::new();

    // Ask for user input
    println!("Enter a positive integer: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Converts input to an integer
    let n: i32 = input.trim().parse().expect("Please enter a valid number");

    // Calculates the sum of even numbers from 2 to n
    let mut sum = 0;
    for i in (2..=n).step_by(2) {
        sum += i;
    }

    // Print the result
    println!("Sum of even numbers from 2 to {} is {}", n, sum);
}