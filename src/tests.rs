#[cfg(test)]
mod tests {
    use crate::problem_1_two_sum;
    use crate::problem_3_longest_substring_without_repeating_characters;
    use crate::problem_4_median_of_two_sorted_arrays;
    use crate::problem_198_house_robber;
    use crate::problem_645_set_mismatch;

    #[test]
    fn problem_1_two_sum() {
        problem_1_two_sum::solution::tests()
    }

    #[test]
    fn problem_3_longest_substring_without_repeating_characters() {
        problem_3_longest_substring_without_repeating_characters::solution::tests()
    }

    #[test]
    fn problem_4_median_of_two_sorted_arrays() {
        problem_4_median_of_two_sorted_arrays::solution::tests()
    }

    #[test]
    fn problem_198_house_robber() {
        problem_198_house_robber::solution::tests()
    }

    #[test]
    fn problem_645_set_mismatch() {
        problem_645_set_mismatch::solution::tests()
    }
}