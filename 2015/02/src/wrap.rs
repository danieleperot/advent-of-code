use crate::shared::extract_sides;

pub fn wrap_boxes(boxes: &str) -> u32 {
    let mut total = 0;

    for single_box in boxes.trim().split("\n") {
        let (l, w, h) = extract_sides(single_box);
        total += wrap_box(l, w, h);
    }

    total
}

fn wrap_box(l: u32, w: u32, h: u32) -> u32 {
    let sides = [l * w, w * h, h * l];
    let mut area = 0;
    let mut smaller_side = sides[0];

    for side in sides {
        if side < smaller_side {
            smaller_side = side;
        }

        area += side * 2;
    }

    area + smaller_side
}

#[test]
fn if_height_is_missing_then_only_length_times_width_is_returned() {
    assert_eq!(2 * (2 * 3), wrap_box(2, 3, 0));
}

#[test]
fn if_length_is_missing_then_only_width_times_height_is_returned() {
    assert_eq!(2 * (2 * 3), wrap_box(0, 2, 3));
}

#[test]
fn if_width_is_missing_then_only_length_times_height_is_returned() {
    assert_eq!(2 * (2 * 3), wrap_box(2, 0, 3));
}

#[test]
fn returns_total_area_plus_extra_equal_to_small_side() {
    assert_eq!(52 + 6, wrap_box(2, 3, 4));
    assert_eq!(42 + 1, wrap_box(1, 1, 10));
}

#[test]
fn sums_no_boxes_to_zero() {
    assert_eq!(0, wrap_boxes(""))
}

#[test]
fn sums_all_boxes() {
    let boxes = "2x3x4\n1x1x10";
    assert_eq!(58 + 43, wrap_boxes(boxes));
}
