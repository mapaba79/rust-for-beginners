use std::io;

fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break; // If there were no swaps, the array is already sorted
        }
    }
}

fn main() {
    println!("Enter numbers to sort, separated by spaces: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    println!("Unsorted array: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("Sorted array: {:?}", numbers);
}

