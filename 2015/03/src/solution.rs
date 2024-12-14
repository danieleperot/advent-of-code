use std::collections::HashSet;

pub fn visited_houses(movements: &str, santas: usize) -> HashSet<(i32, i32)> {
    let mut houses = HashSet::new();
    let mut santa_positions = Vec::new();
    for _ in 0..santas {
        santa_positions.push((0, 0));
    }
    houses.insert((0, 0));


    for (index, movement) in movements.chars().into_iter().enumerate() {
        match movement {
            '>' => santa_positions[index % santas].0 += 1,
            '<' => santa_positions[index % santas].0 += -1,
            '^' => santa_positions[index % santas].1 += 1,
            'v' => santa_positions[index % santas].1 += -1,
            _ => {}
        }

        houses.insert(santa_positions[index % santas].clone());
    }

    houses
}

#[test]
fn visits_only_house_at_starting_position_with_no_movements() {
    let result = visited_houses("", 1);
    assert_eq!(1, result.len());
    assert_eq!((0, 0), result.into_iter().next().unwrap());
}

#[test]
fn visit_only_one_house_by_going_up() {
    let result = visited_houses("^", 1);
    assert_eq!(2, result.len());
    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(0, 1)));
}

#[test]
fn visit_only_one_house_by_going_down() {
    let result = visited_houses("v", 1);
    assert_eq!(2, result.len());
    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(0, -1)));
}

#[test]
fn visit_only_one_house_by_going_right() {
    let result = visited_houses(">", 1);
    assert_eq!(2, result.len());
    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(1, 0)));
}

#[test]
fn visit_only_one_house_by_going_left() {
    let result = visited_houses("<", 1);
    assert_eq!(2, result.len());
    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(-1, 0)));
}

#[test]
fn visit_multiple_houses() {
    let result = visited_houses("^>v<", 1);

    assert_eq!(4, result.clone().len());

    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(0, 1)));
    assert!(result.contains(&(1, 1)));
    assert!(result.contains(&(1, 0)));
}

#[test]
fn visit_multiple_houses_more_than_once() {
    let result = visited_houses("^v^v^v^v^v", 1);
    assert_eq!(2, result.len());
}

#[test]
fn multiple_santas_visit_instructions_in_turn() {
    let result = visited_houses("^>v<", 2);

    assert_eq!(3, result.len());
    assert!(result.contains(&(0, 0))); // visited by both
    assert!(result.contains(&(0, 1))); // visited by santa 1
    assert!(result.contains(&(1, 0))); // visited by santa 2
}

#[test]
fn multiple_santas_visit_instructions_in_turn_and_one_santa_visiting_other_santa_house_again() {
    let result = visited_houses("^>v<^^", 2);

    assert_eq!(3, result.len());
    assert!(result.contains(&(0, 0))); // visited by both (santa 1 twice + ones by santa 2)
    assert!(result.contains(&(0, 1))); // visited by santa 1 and santa 2
    assert!(result.contains(&(1, 0))); // visited by santa 2
}

#[test]
fn multiple_santas_visit_instructions_as_in_example() {
    let result = visited_houses("^v^v^v^v^v", 2);

    assert_eq!(11, result.len());
    assert!(result.contains(&(0, 0))); // visited by both
    assert!(result.contains(&(0, 1))); // visited by santa 1
    assert!(result.contains(&(0, -1))); // visited by santa 2
    assert!(result.contains(&(0, 2))); // visited by santa 1
    assert!(result.contains(&(0, -2))); // visited by santa 2
    assert!(result.contains(&(0, 3))); // visited by santa 1
    assert!(result.contains(&(0, -3))); // visited by santa 2
    assert!(result.contains(&(0, 4))); // visited by santa 1
    assert!(result.contains(&(0, -4))); // visited by santa 2
    assert!(result.contains(&(0, 5))); // visited by santa 1
    assert!(result.contains(&(0, -5))); // visited by santa 2
}
