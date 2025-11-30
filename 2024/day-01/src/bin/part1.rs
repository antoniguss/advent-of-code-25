fn main() {
    let input = include_str!("./input1.txt");

    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
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

    values1.sort();
    values2.sort();

    let mut sum = 0;
    for i in 0..values1.len() {
        sum += values1[i].abs_diff(values2[i])
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "3   4
4   3
2   5
1   3
3   9
3   3
",
        );
        assert_eq!(result, 11);
    }
}
