use std::io;

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn main() {
    println!("Enter numbers separated by spaces (sorted array):");

    let mut numbers_input = String::new();
    io::stdin()
        .read_line(&mut numbers_input)
        .expect("Failed to read line");

    let target: i32 = target_input.trim().parse().expect("Invalid number");

    match binary_search(&numbers, target) {
        Some(index) => println!("Target found at position: {}", index),
        None => println!("Target not found."),
    }
}

#[cfg(test)]
mod tests {
    use supper::*;

    #[test]
    fn test_binary_search_found() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, 5), Some(2));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, 6), None);
    }

    #[test]
    fn test_binary_search_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, 1), None);
    }
}
