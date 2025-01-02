pub fn count_safe_reports(reports: &str, tolerance: bool) -> usize {
    let mut sum = 0;
    for report in reports.lines() {
        let levels = report.split_whitespace();
        let report = levels.map(|level| level.parse().unwrap()).collect();
        if is_safe(&report, tolerance) {
            sum += 1;
        }
    }

    sum
}

fn is_safe(report: &Vec<usize>, tolerance: bool) -> bool {
    let mut growing = None;
    let mut growing_positive: usize = 0;
    let mut growing_negative: usize = 0;
    let mut faulty_index = None;

    for (index, level) in report.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let prev_index = if faulty_index.is_some() && faulty_index.unwrap() == index - 1 {
            index - 2
        } else {
            index - 1
        };

        let previous = report.get(prev_index).unwrap();
        let is_growing = level > previous;
        if growing.is_none() {
            growing = Some(is_growing);
        }

        if is_growing {
            growing_positive += 1
        } else {
            growing_negative += 1
        }

        if !tolerance && is_growing != growing.unwrap() {
            return false;
        }

        let distance = previous.abs_diff(*level);
        if distance == 0 || distance > 3 {
            if !tolerance {
                return false;
            }

            if faulty_index.is_some() {
                if faulty_index.unwrap() == 1 && index == 2 {
                    // Special case! Could be that either index 0 or index 1 are actually wrong, but
                    // we don't have enough context. Right now, we know that index - index0 does not
                    // work, but if index - index1 works, then we could assume that the problem was
                    // actually index0. If that's the case, we set index0 to be the problematic index,
                    // and we move on to the next.
                    let diff_zero = report.get(1).unwrap().abs_diff(*level);
                    if diff_zero == 0 || diff_zero > 3 {
                        return false;
                    } else {
                        faulty_index = Some(0)
                    }
                } else {
                    return false;
                }
            } else {
                faulty_index = Some(index);
            }
        }
    }

    let is_tolerant_for_sign = !(growing_negative > 1 && growing_positive > 1);
    !tolerance || is_tolerant_for_sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_level_is_safe() {
        assert!(is_safe(&vec![], false));
    }

    #[test]
    fn single_level_is_safe() {
        assert!(is_safe(&vec![1], false));
    }

    #[test]
    fn two_levels_are_not_safe_if_they_are_the_same() {
        assert!(!is_safe(&vec![1, 1], false));
    }

    #[test]
    fn two_levels_are_safe_if_they_are_different() {
        assert!(is_safe(&vec![1, 2], false));
    }

    #[test]
    fn two_levels_are_not_safe_if_their_distance_is_more_than_3() {
        assert!(!is_safe(&vec![1, 5], false));
        assert!(!is_safe(&vec![5, 1], false));
    }

    #[test]
    fn three_levels_are_safe_if_direction_does_not_change() {
        assert!(is_safe(&vec![1, 2, 3], false));
        assert!(!is_safe(&vec![1, 3, 2], false));
    }

    #[test]
    fn solves_example() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1], false));
        assert!(!is_safe(&vec![1, 2, 7, 8, 9], false));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1], false));
        assert!(!is_safe(&vec![1, 3, 2, 4, 5], false));
        assert!(!is_safe(&vec![8, 6, 4, 4, 1], false));
        assert!(is_safe(&vec![1, 3, 6, 7, 9], false));
    }

    #[test]
    fn finds_safe_report_from_string() {
        assert_eq!(1, count_safe_reports(&"7 6 4 2 1", false));
        assert_eq!(0, count_safe_reports(&"1 2 7 8 9", false));
        assert_eq!(0, count_safe_reports(&"9 7 6 2 1", false));
        assert_eq!(0, count_safe_reports(&"1 3 2 4 5", false));
        assert_eq!(0, count_safe_reports(&"8 6 4 4 1", false));
        assert_eq!(1, count_safe_reports(&"1 3 6 7 9", false));
    }

    #[test]
    fn counts_how_many_reports_are_safe() {
        assert_eq!(
            2,
            count_safe_reports(
                &"7 6 4 2 1
                 1 2 7 8 9
                 9 7 6 2 1
                 1 3 2 4 5
                 8 6 4 4 1
                 1 3 6 7 9",
                false
            )
        );
    }

    #[test]
    fn tolerance_marks_record_safe_if_only_one_sign_is_different() {
        assert!(is_safe(&vec![1, 4, 2], true));
        assert!(is_safe(&vec![1, 4, 2, 3], true));
        assert!(is_safe(&vec![1, 4, 2, 1], true));
        assert!(is_safe(&vec![1, 4, 2, 3, 5], true));
        assert!(!is_safe(&vec![1, 4, 2, 5, 3], true));
    }

    #[test]
    fn tolerance_marks_record_safe_if_only_one_distance_is_unsafe() {
        assert!(is_safe(&vec![1, 2, 10], true));
        assert!(!is_safe(&vec![1, 2, 10, 20], true));
        assert!(!is_safe(&vec![1, 10, 11, 20], true));
        assert!(is_safe(&vec![1, 6, 7, 8], true));
        assert!(is_safe(&vec![10, 4, 3, 2], true));
        assert!(!is_safe(&vec![1, 4, 8, 9], true));
        assert!(is_safe(&vec![1, 5, 8, 9], true));
        assert!(!is_safe(&vec![1, 3, 4, 8, 9], true));
        assert!(is_safe(&vec![1, 3, 4, 8, 7], true));
        assert!(is_safe(&vec![1, 3, 4, 4, 5], true));
        assert!(!is_safe(&vec![1, 3, 4, 4, 4], true));
        assert!(!is_safe(&vec![1, 4, 8, 9, 20], true));
    }

    #[test]
    fn solves_example_with_tolerance() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1], true));
        assert!(!is_safe(&vec![1, 2, 7, 8, 9], true));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1], true));
        assert!(is_safe(&vec![1, 3, 2, 4, 5], true));
        assert!(is_safe(&vec![8, 6, 4, 4, 1], true));
        assert!(is_safe(&vec![1, 3, 6, 7, 9], true));
    }

    #[test]
    fn counts_how_many_reports_are_safe_with_tolerance() {
        assert_eq!(
            4,
            count_safe_reports(
                &"7 6 4 2 1
                 1 2 7 8 9
                 9 7 6 2 1
                 1 3 2 4 5
                 8 6 4 4 1
                 1 3 6 7 9",
                true
            )
        );
    }
}
