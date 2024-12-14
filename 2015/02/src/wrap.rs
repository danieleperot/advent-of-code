pub fn wrap_boxes(boxes: &str) -> u32 {
    fn wrap_boxes(boxes: &str) -> u32 {
        let mut total = 0;

        for single_box in boxes.trim().split("\n") {
            total += wrap_single_box(single_box);
        }

        total
    }
    let mut total = 0;

    for single_box in boxes.trim().split("\n") {
        total += wrap_single_box(single_box);
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

fn wrap_single_box(single_box: &str) -> u32 {
    let mut sides = single_box.split("x");
    let l = sides.next().unwrap_or("0").parse().unwrap_or(0);
    let w = sides.next().unwrap_or("0").parse().unwrap_or(0);
    let h = sides.next().unwrap_or("0").parse().unwrap_or(0);

    wrap_box(l, w, h)
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
