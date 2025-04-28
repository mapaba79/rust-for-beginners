use std::io;

fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn main() {
    println!("Enter a word to check if it is a palindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); // Remove newline character

    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_true() {
        assert!(is_palindrome("radar"));
    }

    #[test]
    fn test_palindrome_false() {
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_empty_string() {
        assert!(is_palindrome(""));
    }
}
