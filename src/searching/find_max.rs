use std::io;

fn find_maximum(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut maximum = arr[0];
    for &item in arr.iter() {
        if item > maximum {
            maximum = item;
        }
    }
    Some(maximum)
}

fn main() {
    println!("Enter numbers separated by spaces:");

    let mut numbers_input = String::new();
    io::stdin()
        .read_line(&mut numbers_input)
        .expect("Failed to real line");

    let numbers: Vec<i32> = numbers_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    match find_maximum(&numbers) {
        Some(max) => println!("The maximum number is: {}", max),
        None => println!("Array is empty."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_normal() {
        let arr = [4, 8, 1, 9, 2];
        assert_eq!(find_maximum(&arr), Some(9));
    }

    #[test]
    fn test_find_maximum_single_element() {
        let arr = [7];
        assert_eq!(find_maximum(&arr), Some(7));
    }

    #[test]
    fn test_find_maximum_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(find_maximum(&arr), None);
    }
}
