#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

pub fn diagonaldifference(arr: &[Vec<i32>]) -> i32 {
    let mut leftdiagonal = 0;
    let mut rightdiagonal = 0;
    
    for i in 0..arr.len() {
        leftdiagonal += arr[i][i];
        rightdiagonal += arr[i][arr.len() - 1 - i];
    }
    (leftdiagonal - rightdiagonal).abs()
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonaldifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn testdiagonaldifferencesample() {
    let matrix = vec![
        vec![11, 2, 4],
        vec![4, 5, 6],
        vec![10, 8, -12],
    ];

    let result = diagonaldifference(&matrix);
    let expected = 15;

    assert_eq!(result, expected);
}

#[test]
fn testdiagonaldifferencesecondcase() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![9, 8, 9],
    ];

    let result = diagonaldifference(&matrix);
    let expected = 2;

    assert_eq!(result, expected);
}

#[test]
fn testdiagonaldifferenceoneelement() {
    let matrix = vec![vec![5]];

    let result = diagonaldifference(&matrix);
    let expected = 0;

    assert_eq!(result, expected);
}