#![allow(dead_code)]

pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut best_score = scores[0];
    let mut worst_score = scores[0];

    let mut best_count = 0;
    let mut worst_count = 0;

    for &score in scores.iter().skip(1) {
        if score > best_score {
            best_score = score;
            best_count += 1;
        } else if score < worst_score {
            worst_score = score;
            worst_count += 1;
        }
    }

    vec![best_count, worst_count]
}

#[test]
fn test_breaking_records_sample() {
    let result = breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1]);
    let expected = vec![2, 4];

    assert_eq!(result, expected);
}

#[test]
fn test_breaking_records_second_sample() {
    let result = breaking_records(&[3, 4, 21, 36, 10, 28, 35, 5, 24, 42]);
    let expected = vec![4, 0];

    assert_eq!(result, expected);
}

#[test]
fn test_breaking_records_no_changes() {
    let result = breaking_records(&[10, 10, 10, 10]);
    let expected = vec![0, 0];

    assert_eq!(result, expected);
}