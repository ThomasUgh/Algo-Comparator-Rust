mod bubble_sort;
mod quick_sort;

use std::time::Instant;

fn main() {
    let mut arr1 = vec![64, 34, 25, 12, 22, 11, 90, 33, 11, 12, 13, 243, 454, 33, 332, 443, 322, 777, 776, 567, 876, 865];
    let mut arr2 = arr1.clone();

    // Bubble Sort
    let start = Instant::now();
    bubble_sort::bubble_sort(&mut arr1);
    let duration = start.elapsed();
    println!("Bubble Sort dauerte: {:.2} ms", duration.as_secs_f64() * 1000.0);

    // Quick Sort
    let start = Instant::now();
    quick_sort::quick_sort(&mut arr2);
    let duration = start.elapsed();
    println!("Quick Sort dauerte: {:.2} ms", duration.as_secs_f64() * 1000.0);
}