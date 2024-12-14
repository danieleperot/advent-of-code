use crate::shared::extract_sides;

pub fn ribbon_for_all_boxes(boxes: &str) -> u32 {
    let mut total = 0;

    for single_box in boxes.trim().split("\n") {
        let (l, w, h) = extract_sides(single_box);
        total += ribbon_for_box(l, w, h);
    }

    total
}

fn smallest_perimeter(l: u32, w: u32, h: u32) -> u32 {
    let mut half_perimeters = vec![l + w, w + h, l + h];
    half_perimeters.sort();

    half_perimeters.into_iter().next().unwrap_or(0) * 2
}

fn ribbon_for_box(l: u32, w: u32, h: u32) -> u32 {
    let volume = l * w * h;
    let perimeter = smallest_perimeter(l, w, h);

    volume + perimeter
}

#[test]
fn finds_smallest_perimeter() {
    assert_eq!(2 * (1 + 0), smallest_perimeter(0, 1, 2));
    assert_eq!(2 * (2 + 3), smallest_perimeter(2, 3, 4));
    assert_eq!(2 * (1 + 1), smallest_perimeter(1, 1, 10));
}

#[test]
fn calculates_ribbon_size_per_box() {
    assert_eq!(2 * (2 + 3) + 2 * 3 * 4, ribbon_for_box(2, 3, 4));
    assert_eq!(2 * (1 + 1) + 1 * 10, ribbon_for_box(1, 1, 10));
}

#[test]
fn calculates_ribbon_needed_for_all_boxes() {
    let boxes = "2x3x4\n1x1x10";
    assert_eq!(48, ribbon_for_all_boxes(boxes));
}
