pub fn extract_sides(single_box: &str) -> (u32, u32, u32) {
    let mut sides = single_box.split("x");
    let l = sides.next().unwrap_or("0").parse().unwrap_or(0);
    let w = sides.next().unwrap_or("0").parse().unwrap_or(0);
    let h = sides.next().unwrap_or("0").parse().unwrap_or(0);

    (l, w, h)
}
