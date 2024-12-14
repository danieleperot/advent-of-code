use std::collections::HashSet;

pub fn visited_houses(movements: &str) -> HashSet<(i32, i32)> {
    let mut houses = HashSet::new();
    let mut current = (0, 0);

    houses.insert(current.clone());

    for movement in movements.chars() {
        match movement {
            '>' => current.0 += 1,
            '<' => current.0 += -1,
            '^' => current.1 += 1,
            'v' => current.1 += -1,
            _ => {}
        }

        houses.insert(current.clone());
    }

    houses
}

#[test]
fn visits_only_house_at_starting_position_with_no_movements() {
    let result = visited_houses("");
    assert_eq!(1, result.len());
    assert_eq!((0, 0), result.into_iter().next().unwrap());
}

#[test]
fn visit_only_one_house_by_going_up() {
    let result = visited_houses("^");
    assert_eq!(2, result.len());
    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(0, 1)));
}

#[test]
fn visit_only_one_house_by_going_down() {
    let result = visited_houses("v");
    assert_eq!(2, result.len());
    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(0, -1)));
}

#[test]
fn visit_only_one_house_by_going_right() {
    let result = visited_houses(">");
    assert_eq!(2, result.len());
    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(1, 0)));
}

#[test]
fn visit_only_one_house_by_going_left() {
    let result = visited_houses("<");
    assert_eq!(2, result.len());
    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(-1, 0)));
}

#[test]
fn visit_multiple_houses() {
    let result = visited_houses("^>v<");

    assert_eq!(4, result.clone().len());

    assert!(result.contains(&(0, 0)));
    assert!(result.contains(&(0, 1)));
    assert!(result.contains(&(1, 1)));
    assert!(result.contains(&(1, 0)));
}

#[test]
fn visit_multiple_houses_more_than_once() {
    let result = visited_houses("^v^v^v^v^v");
    assert_eq!(2, result.len());
}
