mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task10::sock_merchant;

fn main() {
    let result = sock_merchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]);

    println!("{}", result);
}