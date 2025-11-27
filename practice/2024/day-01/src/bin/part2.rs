use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");

    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut values1: Vec<i32> = lines.iter().map(|nums| nums[0]).collect();
    let mut values2: Vec<i32> = lines.iter().map(|nums| nums[1]).collect();

    let mut counts: HashMap<i32, i32> = HashMap::new();

    for i in 0..values2.len() {
        *counts.entry(values2[i]).or_insert(0) += 1;
    }
    let mut sum = 0;

    for i in 0..values1.len() {
        sum += values1[i] * *counts.entry(values1[i]).or_default()
    }

    sum as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "3   4
4   3
2   5
1   3
3   9
3   3
",
        );
        assert_eq!(result, 31);
    }
}
