use std::io;

fn merge_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = arr.to_vec();

    let (mut i, mut j, mut k) = (0, mid, 0);
    while i < mid && j < n {
        if arr[i] <= arr[j] {
            merged[k] = arr[i];
            i += 1;
        } else {
            merged[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        merged[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j < n {
        merged[k] = arr[j];
        j += 1;
        k += 1;
    }

    arr.copy_from_slice(&merged);
}

fn main() {
    println!("Enter numbers to sort: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    println!("Unsorted array: {:?}", numbers);
    merge_sort(&mut numbers);
    println!("Sorted array: {:?}", numbers);
}
