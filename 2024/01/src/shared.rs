pub fn string_to_vectors(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut first: Vec<usize> = vec![];
    let mut second: Vec<usize> = vec![];

    for line in input.lines() {
        let numbers = line.split_once(" ").unwrap_or(("0", "0"));
        first.push(numbers.0.parse().unwrap_or(0));
        second.push(numbers.1.trim().parse().unwrap_or(0));
    }
    (first, second)
}
