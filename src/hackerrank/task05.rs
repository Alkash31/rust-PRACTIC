#![allow(dead_code)]

pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (usize, usize) {
    let apple_count = apples
        .iter()
        .filter(|&&apple| {
            let position = a + apple;
            position >= s && position <= t
        })
        .count();

    let orange_count = oranges
        .iter()
        .filter(|&&orange| {
            let position = b + orange;
            position >= s && position <= t
        })
        .count();

    (apple_count, orange_count)
}

#[test]
fn test_count_apples_and_oranges_sample() {
    let result = count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
    let expected = (1, 1);

    assert_eq!(result, expected);
}

#[test]
fn test_count_apples_and_oranges_second_case() {
    let result = count_apples_and_oranges(7, 10, 4, 12, &[2, 3, -4], &[3, -2, -4]);
    let expected = (1, 2);

    assert_eq!(result, expected);
}

#[test]
fn test_count_apples_and_oranges_no_fruits_on_house() {
    let result = count_apples_and_oranges(10, 20, 5, 25, &[-2, -1], &[1, 2]);
    let expected = (0, 0);

    assert_eq!(result, expected);
}