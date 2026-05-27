#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn divisiblesumpairs(_n: i32, k: i32, ar: &[i32]) -> i32 {
let mut count = 0;

    for i in 0..ar.len() {
        for j in i + 1..ar.len() {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisiblesumpairs(n, k, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn testpairssample() {
    let result = divisiblesumpairs(6, 3, &[1, 3, 2, 6, 1, 2]);
    let expected = 5;

    assert_eq!(result, expected);
}

#[test]
fn testpairssecondcase() {
    let result = divisiblesumpairs(6, 5, &[1, 2, 3, 4, 5, 6]);
    let expected = 3;

    assert_eq!(result, expected);
}

#[test]
fn testnopairs() {
    let result = divisiblesumpairs(4, 10, &[1, 2, 3, 4]);
    let expected = 0;

    assert_eq!(result, expected);
}