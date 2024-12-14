pub fn find_match_for_key(key: &str) -> usize {
    for candidate in 0..usize::MAX {
        let result = md5::compute(format!("{}{}", key, candidate));
        if format!("{:?}", result).starts_with("00000") {
            return candidate;
        }
    }

    0
}

#[test]
fn finds_solution_for_example_one() {
    assert_eq!(609043, find_match_for_key("abcdef"));
    assert_eq!(1048970, find_match_for_key("pqrstuv"));
}