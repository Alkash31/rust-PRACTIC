mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task09::migratory_birds;

fn main() {
    let result = migratory_birds(&[1, 4, 4, 4, 5, 3]);

    println!("{}", result);
}