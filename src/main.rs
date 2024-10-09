mod bubble_sort;
mod quick_sort;
mod random_string;

use random_string::generate_random_string;
use std::time::Instant;

fn main() {
    // Generierung eines zufälligen Strings
    let random_str_length = 150;
    let random_str = generate_random_string(random_str_length);
    println!("Generierter zufälliger String: {}", random_str);

    let mut chars: Vec<char> = random_str.chars().collect();

    // Bubble Sort
    let start = Instant::now();
    bubble_sort::bubble_sort(&mut chars);
    let duration = start.elapsed();
    let sorted_str: String = chars.iter().collect();
    println!("Bubble Sort dauerte: {:.2} ms", duration.as_secs_f64() * 1000.0);
    println!("Bubble Sort Ergebnis: {}", sorted_str);

    // Quick Sort
    let random_str = generate_random_string(random_str_length);
    let mut chars: Vec<char> = random_str.chars().collect();
    let start = Instant::now();
    quick_sort::quick_sort(&mut chars);
    let duration = start.elapsed();
    let sorted_str: String = chars.iter().collect();
    println!("Quick Sort dauerte: {:.2} ms", duration.as_secs_f64() * 1000.0);
    println!("Quick Sort Ergebnis: {}", sorted_str);
}