pub fn find_match_for_key(key: &str, zeros: usize) -> usize {
    let mut start = String::new();
    for _ in 0..zeros {
        start.push('0');
    }

    for candidate in 0..usize::MAX {
        let result = md5::compute(format!("{}{}", key, candidate));
        if format!("{:?}", result).starts_with(start.as_str()) {
            return candidate;
        }
    }

    0
}

#[test]
fn finds_solution_for_example_one() {
    assert_eq!(609043, find_match_for_key("abcdef", 5));
    assert_eq!(1048970, find_match_for_key("pqrstuv", 5));
}
