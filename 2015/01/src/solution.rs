pub enum StopAt {
    None,
    Floor(i32),
}

pub fn solve(input: &str, stop_at: StopAt) -> i32 {
    let mut accumulator = 0;
    for (index, character) in input.chars().enumerate() {
        match character {
            '(' => accumulator += 1,
            ')' => accumulator -= 1,
            _ => {}
        }

        match stop_at {
            StopAt::None => {}
            StopAt::Floor(floor) => {
                if accumulator == floor {
                    return (index + 1) as i32;
                }
            }
        }
    }

    accumulator
}

#[test]
fn empty_data_returns_0() {
    assert_eq!(0, solve("", StopAt::None));
}

#[test]
fn open_parens_returns_1() {
    assert_eq!(1, solve("(", StopAt::None));
}

#[test]
fn close_parens_returns_neg_1() {
    assert_eq!(-1, solve(")", StopAt::None));
}

#[test]
fn multiple_open_parens_return_sum_of_open_parens() {
    assert_eq!(3, solve("(((", StopAt::None));
}

#[test]
fn multiple_close_parens_return_neg_sum_of_close_parens() {
    assert_eq!(-3, solve(")))", StopAt::None));
}

#[test]
fn other_characters_are_ignored() {
    assert_eq!(-3, solve(")ahd3h  )3y5gef32@!)", StopAt::None));
}

#[test]
fn returns_position_of_first_time_a_floor_is_reached_if_floor_is_specified() {
    assert_eq!(4, solve("((((", StopAt::Floor(4)));
    assert_eq!(6, solve("((()(()()()", StopAt::Floor(4)));
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
        assert_eq!(
            expected,
            solve(input, StopAt::None),
            "[#{}] {} should return {}",
            index,
            input,
            expected
        );
    }
}
