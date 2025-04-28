use std::io;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("Enter the position (n) to find the fibonacci number:");

    let mut n_input = String::new();
    io::stdin()
        .read_line(&mut n_input)
        .expect("Failed to read line");

    let n: u32 = n_input.trim().parse().expect("Invalid number");

    println!("The Fibonacci number at position {} is {}", n, fibonacci(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_know_values() {
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(10), 55);
    }
}
