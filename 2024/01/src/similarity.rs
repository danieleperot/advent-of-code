use std::collections::HashMap;

pub fn similarity_from_string(input: &str) -> usize {
    let (left, right) = crate::shared::string_to_vectors(input);

    similarity(left, right)
}

fn similarity(left: Vec<usize>, right: Vec<usize>) -> usize {
    // one digit similarity = [value of digit] * [occurrences on left] * [occurrences of right]
    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();

    for (index, left) in left.iter().enumerate() {
        let right = right
            .iter()
            .nth(index)
            .expect("Left and right are not the same size.");

        let mut matches_left = map.get(left).unwrap_or(&(0, 0)).clone();
        matches_left.0 += 1;
        map.insert(*left, matches_left);

        let mut matches_right = map.get(right).unwrap_or(&(0, 0)).clone();
        matches_right.1 += 1;
        map.insert(*right, matches_right);
    }

    let mut sum = 0;
    for (key, (left, right)) in map.iter() {
        sum += key * left * right;
    }

    sum
}

#[test]
fn similarity_of_empty_vectors_is_zero() {
    assert_eq!(0, similarity(vec![], vec![]));
}

#[test]
fn similarity_of_two_different_vectors_is_zero() {
    assert_eq!(0, similarity(vec![1], vec![2]));
}

#[test]
fn similarity_of_two_vectors_of_one_number_with_same_number_is_1() {
    assert_eq!(1, similarity(vec![1], vec![1]));
}

#[test]
fn similarity_of_two_vectors_with_second_repeating_one_number_as_first_is_1() {
    assert_eq!(1, similarity(vec![1, 3], vec![2, 1]));
}

#[test]
fn similarity_of_two_vectors_with_first_repeating_2_is_4() {
    assert_eq!(4, similarity(vec![2, 2], vec![2, 1]));
}

#[test]
fn similarity_of_two_vectors_with_first_and_second_repeating_2_is_4() {
    assert_eq!(8, similarity(vec![2, 2], vec![2, 2]));
}

#[test]
fn sums_similar_numbers() {
    assert_eq!(
        (2 * 2 * 2) + (1 * 1 * 0) + (3 * 1 * 1) + (6 * 1 * 0),
        similarity(vec![2, 2, 1, 3, 6], vec![2, 2, 3, 12, 33])
    );
}

#[test]
fn solves_example() {
    assert_eq!(
        31,
        similarity(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    )
}

#[test]
fn solves_example_using_string() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
    assert_eq!(31, similarity_from_string(input));
}
