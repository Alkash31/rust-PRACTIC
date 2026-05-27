#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn pagecount(n: i32, p: i32) -> i32 {
    let turns_from_start = p / 2;

    let total_turns = n / 2;
    let page_turn = p / 2;
    let turns_from_end = total_turns - page_turn;

    std::cmp::min(turns_from_start, turns_from_end)
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pagecount(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn testcountsampleone() {
    let result = pagecount(6, 2);
    let expected = 1;

    assert_eq!(result, expected);
}

#[test]
fn testcountsampletwo() {
    let result = pagecount(5, 4);
    let expected = 0;

    assert_eq!(result, expected);
}

#[test]
fn testcountmiddlepage() {
    let result = pagecount(10, 7);
    let expected = 2;

    assert_eq!(result, expected);
}