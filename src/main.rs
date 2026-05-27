mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task13::divisiblesumpairs;

fn main() {
    let result = divisiblesumpairs(6, 3, &[1, 3, 2, 6, 1, 2]);

    println!("{}", result);
}