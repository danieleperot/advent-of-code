pub fn count_safe_reports(reports: String) -> usize {
    let mut sum = 0;
    for report in reports.lines() {
        let levels = report.split_whitespace();
        let report = levels.map(|level| level.parse().unwrap()).collect();
        if is_safe(&report) {
            sum += 1;
        }
    }

    sum
}

fn is_safe(report: &Vec<usize>) -> bool {
    let mut growing = None;
    for (index, level) in report.iter().skip(1).enumerate() {
        // Index starts from 0, but because we skip 1, 0 = vec[1]
        let previous = report.get(index).unwrap();
        let is_growing = level > previous;
        if growing.is_none() {
            growing = Some(is_growing);
        }

        if is_growing != growing.unwrap() {
            return false;
        }

        let distance = previous.abs_diff(*level);
        if distance == 0 || distance > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_level_is_safe() {
        assert!(is_safe(&vec![]));
    }

    #[test]
    fn single_level_is_safe() {
        assert!(is_safe(&vec![1]));
    }

    #[test]
    fn two_levels_are_not_safe_if_they_are_the_same() {
        assert!(!is_safe(&vec![1, 1]));
    }

    #[test]
    fn two_levels_are_safe_if_they_are_different() {
        assert!(is_safe(&vec![1, 2]));
    }

    #[test]
    fn two_levels_are_not_safe_if_their_distance_is_more_than_3() {
        assert!(!is_safe(&vec![1, 5]));
        assert!(!is_safe(&vec![5, 1]));
    }

    #[test]
    fn three_levels_are_safe_if_direction_does_not_change() {
        assert!(is_safe(&vec![1, 2, 3]));
        assert!(!is_safe(&vec![1, 3, 2]));
    }

    #[test]
    fn solves_example() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1]));
        assert!(!is_safe(&vec![1, 2, 7, 8, 9]));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1]));
        assert!(!is_safe(&vec![1, 3, 2, 4, 5]));
        assert!(!is_safe(&vec![8, 6, 4, 4, 1]));
        assert!(is_safe(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn finds_safe_report_from_string() {
        assert_eq!(1, count_safe_reports("7 6 4 2 1".to_string()));
        assert_eq!(0, count_safe_reports("1 2 7 8 9".to_string()));
        assert_eq!(0, count_safe_reports("9 7 6 2 1".to_string()));
        assert_eq!(0, count_safe_reports("1 3 2 4 5".to_string()));
        assert_eq!(0, count_safe_reports("8 6 4 4 1".to_string()));
        assert_eq!(1, count_safe_reports("1 3 6 7 9".to_string()));
    }

    #[test]
    fn counts_how_many_reports_are_safe() {
        assert_eq!(
            2,
            count_safe_reports(
                "7 6 4 2 1
                 1 2 7 8 9
                 9 7 6 2 1
                 1 3 2 4 5
                 8 6 4 4 1
                 1 3 6 7 9"
                    .to_string()
            )
        );
    }
}
