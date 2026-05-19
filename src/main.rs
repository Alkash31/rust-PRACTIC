mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task05::count_apples_and_oranges;

fn main() {
    let result = count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);

    println!("Apples: {}", result.0);
    println!("Oranges: {}", result.1);
}