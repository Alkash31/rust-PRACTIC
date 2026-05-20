pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    let mut count = 0;

    for number in start..=end {
        let first_condition = a.iter().all(|value| number % value == 0);
        let second_condition = b.iter().all(|value| value % number == 0);

        if first_condition && second_condition {
            count += 1;
        }
    }

    count
}

#[test]
fn test_get_total_x_sample() {
    let result = get_total_x(&[2, 4], &[16, 32, 96]);
    let expected = 3;

    assert_eq!(result, expected);
}

#[test]
fn test_get_total_x_second_case() {
    let result = get_total_x(&[2, 6], &[24, 36]);
    let expected = 2;

    assert_eq!(result, expected);
}

#[test]
fn test_get_total_x_no_numbers() {
    let result = get_total_x(&[3, 5], &[16, 32, 96]);
    let expected = 0;

    assert_eq!(result, expected);
}