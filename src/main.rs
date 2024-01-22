mod tests;

mod problem_1_two_sum;
mod problem_3_longest_substring_without_repeating_characters;
mod problem_198_house_robber;

fn main() -> std::io::Result<()> {
    problem_1_two_sum::solution::tests();
    problem_3_longest_substring_without_repeating_characters::solution::tests();
    problem_198_house_robber::solution::tests();
    Ok(())
}