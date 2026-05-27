mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task12::birthdaycakecandles;

fn main() {
    let result = birthdaycakecandles(&[3, 2, 1, 3]);

    println!("{}", result);
}