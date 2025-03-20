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
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", numbers);

    bubble_sort(&mut numbers);
    println!("Sorted array: {:?}", numbers);
}