#![allow(dead_code)]

pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut counts = [0; 101];

    for &sock in ar {
        counts[sock as usize] += 1;
    }

    counts.iter().map(|&count| count / 2).sum()
}

#[test]
fn test_sock_merchant_sample() {
    let result = sock_merchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]);
    let expected = 3;

    assert_eq!(result, expected);
}

#[test]
fn test_sock_merchant_two_pairs() {
    let result = sock_merchant(7, &[1, 2, 1, 2, 1, 3, 2]);
    let expected = 2;

    assert_eq!(result, expected);
}

#[test]
fn test_sock_merchant_no_pairs() {
    let result = sock_merchant(5, &[1, 2, 3, 4, 5]);
    let expected = 0;

    assert_eq!(result, expected);
}