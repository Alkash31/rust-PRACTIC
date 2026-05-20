#![allow(dead_code)]

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut answer = 1;
    let mut max_count = counts[1];

    for bird_type in 2..=5 {
        if counts[bird_type] > max_count {
            max_count = counts[bird_type];
            answer = bird_type;
        }
    }

    answer as i32
}

#[test]
fn test_migratory_birds_sample() {
    let result = migratory_birds(&[1, 4, 4, 4, 5, 3]);
    let expected = 4;

    assert_eq!(result, expected);
}

#[test]
fn test_migratory_birds_second_sample() {
    let result = migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]);
    let expected = 3;

    assert_eq!(result, expected);
}

#[test]
fn test_migratory_birds_smallest_id_when_equal() {
    let result = migratory_birds(&[1, 1, 2, 2, 3]);
    let expected = 1;

    assert_eq!(result, expected);
}