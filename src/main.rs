mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task11::diagonaldifference;

fn main() {
    let matrix = vec![
        vec![11, 2, 4],
        vec![4, 5, 6],
        vec![10, 8, -12],
    ];

    let result = diagonaldifference(&matrix);

    println!("{}", result);
}