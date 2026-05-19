mod rustpbyractice;
mod hackerrank;

use crate::hackerrank::task04::grading_students;

fn main() {
    let grades = vec![73, 67, 38, 33];
    let result = grading_students(grades);

    println!("{:?}", result);
}