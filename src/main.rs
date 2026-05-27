mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task14::bonappetit;

fn main() {
    let result = bonappetit(&[3, 10, 2, 9], 1, 12);

    println!("{}", result);
}