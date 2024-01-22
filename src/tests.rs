#[cfg(test)]
mod tests {
    use crate::problem_1_two_sum;
    use crate::problem_3_longest_substring_without_repeating_characters;
    use crate::problem_198_house_robber;

    #[test]
    fn problem_1_two_sum() {
        problem_1_two_sum::solution::tests()
    }

    #[test]
    fn problem_3_longest_substring_without_repeating_characters() {
        problem_3_longest_substring_without_repeating_characters::solution::tests()
    }

    #[test]
    fn problem_198_house_robber() {
        problem_198_house_robber::solution::tests()
    }
}