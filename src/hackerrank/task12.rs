#![allow(dead_code)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn birthdaycakecandles(candles: &[i32]) -> i32 {
    let max_height = *candles.iter().max().unwrap();

    candles
        .iter()
        .filter(|&&candle| candle == max_height)
        .count() as i32
}
   pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdaycakecandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}


#[test]
fn testbirthdaycakecandlesample() {
    let result = birthdaycakecandles(&[3, 2, 1, 3]);
    let expected = 2;

    assert_eq!(result, expected);
}

#[test]
fn testbirthdaycakecandlesallequal() {
    let result = birthdaycakecandles(&[4, 4, 4, 4]);
    let expected = 4;

    assert_eq!(result, expected);
}

#[test]
fn testbirthdaycakecandlesonetallest() {
    let result = birthdaycakecandles(&[1, 2, 3, 4]);
    let expected = 1;

    assert_eq!(result, expected);
}