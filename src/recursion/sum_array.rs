use std::io;

fn sum_array(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        0
    } else {
        arr[0] + sum_array(&arr[1..])
    }
}

fn main() {
    println!("Enter number separated by spaces:");

    let mut numbers_input = String::new();
    io::stdin()
        .read_line(&mut numbers_input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = numbers_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    let total = sum_array(&numbers);

    println!("The sum of the array elements is: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_array_normal() {
        let arr = [1, 2, 3, 4, 5];
        asser_eq!(sum_array(&arr), 15);
    }

    #[test]
    fn test_sum_array_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(sum_array(&arr), 0);
    }
}
