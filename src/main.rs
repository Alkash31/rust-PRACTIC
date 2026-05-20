mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task08::breaking_records;

fn main() {
    let result = breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1]);

    println!("{:?}", result);
}