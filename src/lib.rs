// https://adventofcode.com/2024/day/1#part2

// Calculate a similarity score between the two lists
pub fn list_similarity_score(first: Vec<i32>, second: Vec<i32>) -> i32 {
    first.iter()
        .map(|a| a * second.iter().filter(
            |&b| b.eq(a)
        ).collect::<Vec<&i32>>().len() as i32)
        .sum()
}
