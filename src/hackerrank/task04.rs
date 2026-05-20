#![allow(dead_code)]

pub fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades
        .into_iter()
        .map(|grade| {
            if grade < 38 {
                grade
            } else {
                let remainder = grade % 5;

                if remainder >= 3 {
                    grade + (5 - remainder)
                } else {
                    grade
                }
            }
        })
        .collect()
}

#[test]
fn test_grading_students_sample() {
    let real = grading_students(vec![73, 67, 38, 33]);
    let expected = vec![75, 67, 40, 33];

    assert_eq!(real, expected);
}

#[test]
fn test_grading_students_without_rounding() {
    let real = grading_students(vec![37, 57, 60]);
    let expected = vec![37, 57, 60];

    assert_eq!(real, expected);
}

#[test]
fn test_grading_students_with_rounding() {
    let real = grading_students(vec![38, 84, 98]);
    let expected = vec![40, 85, 100];

    assert_eq!(real, expected);
}