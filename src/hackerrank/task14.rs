#![allow(dead_code)]

pub fn bonappetit(bill: &[i32], k: i32, b: i32) -> String {
    let mut anna_sum = 0;

    for i in 0..bill.len() {
        if i != k as usize {
            anna_sum += bill[i];
        }
    }

    let anna_share = anna_sum / 2;

    if anna_share == b {
        "Bon Appetit".to_string()
    } else {
        (b - anna_share).to_string()
    }
}

#[test]
fn testovercharged() {
    let result = bonappetit(&[3, 10, 2, 9], 1, 12);
    let expected = "5".to_string();

    assert_eq!(result, expected);
}

#[test]
fn testcorrectcharge() {
    let result = bonappetit(&[3, 10, 2, 9], 1, 7);
    let expected = "Bon Appetit".to_string();

    assert_eq!(result, expected);
}

#[test]
fn testanothercase() {
    let result = bonappetit(&[2, 4, 6], 2, 6);
    let expected = "3".to_string();

    assert_eq!(result, expected);
}