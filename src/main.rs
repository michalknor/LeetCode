mod tests;

mod problem_1_two_sum;
mod problem_3_longest_substring_without_repeating_characters;
mod problem_4_median_of_two_sorted_arrays;
mod problem_198_house_robber;
mod problem_645_set_mismatch;
mod problem_1143_longest_common_subsequence;
mod problem_1239_maximum_length_of_a_concatenated_string_with_unique_characters;
mod problem_1457_pseudo_palindromic_paths_in_a_binary_tree;



fn main() -> std::io::Result<()> {
    problem_1_two_sum::solution::tests();
    problem_3_longest_substring_without_repeating_characters::solution::tests();
    problem_4_median_of_two_sorted_arrays::solution::tests();
    problem_198_house_robber::solution::tests();
    problem_645_set_mismatch::solution::tests();
    problem_1143_longest_common_subsequence::solution::tests();
    problem_1239_maximum_length_of_a_concatenated_string_with_unique_characters::solution::tests();
    problem_1457_pseudo_palindromic_paths_in_a_binary_tree::solution::tests();
    Ok(())
}