pub fn solve(input: &str) -> i32 {
    let mut accumulator = 0;
    for character in input.chars() {
        match character {
            '(' => accumulator += 1,
            ')' => accumulator -= 1,
            _ => {}
        }
    }

    accumulator
}

#[test]
fn empty_data_returns_0() {
    assert_eq!(0, solve(""));
}

#[test]
fn open_parens_returns_1() {
    assert_eq!(1, solve("("));
}

#[test]
fn close_parens_returns_neg_1() {
    assert_eq!(-1, solve(")"));
}

#[test]
fn multiple_open_parens_return_sum_of_open_parens() {
    assert_eq!(3, solve("((("));
}

#[test]
fn multiple_close_parens_return_neg_sum_of_close_parens() {
    assert_eq!(-3, solve(")))"));
}

#[test]
fn other_characters_are_ignored() {
    assert_eq!(-3, solve(")ahd3h  )3y5gef32@!)"));
}

#[test]
fn verify_examples() {
    // (()) and ()() both result in floor 0.
    // ((( and (()(()( both result in floor 3.
    // ))((((( also results in floor 3.
    //     ()) and ))( both result in floor -1 (the first basement level).
    // ))) and )())()) both result in floor -3.
    let assertions = vec![
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3),
    ];

    for (index, (input, expected)) in assertions.into_iter().enumerate() {
        assert_eq!(expected, solve(input), "[#{}] {} should return {}", index, input, expected);
    }
}
