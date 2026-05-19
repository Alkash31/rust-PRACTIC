#[allow(dead_code)]
pub fn staircase(n: i32) -> String {
    let mut result = String::new();

    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;

        result.push_str(&" ".repeat(spaces as usize));
        result.push_str(&"#".repeat(hashes as usize));

        if i != n {
            result.push('\n');
        }
    }

    result
}

#[test]
fn test_staircase_size_6() {
    let real = staircase(6);

    let expected = "     #\n    ##\n   ###\n  ####\n #####\n######";

    assert_eq!(real, expected);
}

#[test]
fn test_staircase_size_4() {
    let real = staircase(4);

    let expected = "   #\n  ##\n ###\n####";

    assert_eq!(real, expected);
}