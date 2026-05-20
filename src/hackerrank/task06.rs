pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if x1 == x2 {
        return "YES".to_string();
    }

    let distance = x2 - x1;
    let speed_difference = v1 - v2;

    if speed_difference == 0 {
        return "NO".to_string();
    }

    if distance % speed_difference == 0 && distance / speed_difference >= 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[test]
fn test_kangaroo_yes() {
    let result = kangaroo(0, 3, 4, 2);
    let expected = "YES".to_string();

    assert_eq!(result, expected);
}

#[test]
fn test_kangaroo_no() {
    let result = kangaroo(0, 2, 5, 3);
    let expected = "NO".to_string();

    assert_eq!(result, expected);
}

#[test]
fn test_kangaroo_same_speed() {
    let result = kangaroo(1, 2, 3, 2);
    let expected = "NO".to_string();

    assert_eq!(result, expected);
}