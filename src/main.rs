mod tests;

mod problem_1_two_sum;
mod problem_3_longest_substring_without_repeating_characters;
mod problem_4_median_of_two_sorted_arrays;
mod problem_198_house_robber;
mod problem_645_set_mismatch;

fn main() -> std::io::Result<()> {
    problem_1_two_sum::solution::tests();
    problem_3_longest_substring_without_repeating_characters::solution::tests();
    problem_4_median_of_two_sorted_arrays::solution::tests();
    problem_198_house_robber::solution::tests();
    problem_645_set_mismatch::solution::tests();
    Ok(())
}