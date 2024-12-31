pub fn distance_from_string(input: &str) -> usize {
    let (first, second) = crate::shared::string_to_vectors(input);

    distance(first, second)
}

fn distance(mut first: Vec<usize>, mut second: Vec<usize>) -> usize {
    first.sort();
    second.sort();

    let mut accumulator = 0;
    for (index, first_value) in first.iter().enumerate() {
        accumulator += first_value.abs_diff(*second.get(index).unwrap_or(&0));
    }

    accumulator
}

#[test]
fn distance_of_nothing_is_zero() {
    assert_eq!(0, distance(vec![], vec![]));
}

#[test]
fn distance_of_vectors_with_one_element_is_difference_of_the_two() {
    assert_eq!(1, distance(vec![2], vec![1]));
}

#[test]
fn distance_of_vectors_is_absolute() {
    assert_eq!(1, distance(vec![1], vec![2]));
}

#[test]
fn distance_of_vectors_with_more_elements_is_sum_of_all_differences() {
    assert_eq!(3 + 5, distance(vec![2, 7], vec![5, 12]));
}

#[test]
fn vectors_are_sorted_to_get_the_distance() {
    assert_eq!(3 + 5, distance(vec![7, 2], vec![5, 12]));
}

#[test]
fn check_examples() {
    let first = vec![3, 4, 2, 1, 3, 3];
    let second = vec![4, 3, 5, 3, 9, 3];

    assert_eq!(11, distance(first, second));
}

#[test]
fn checks_distance_from_strings() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    assert_eq!(11, distance_from_string(input));
}
