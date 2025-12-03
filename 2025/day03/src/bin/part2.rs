fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let bank: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            find_joltage(&bank)
        })
        .sum()
}

fn find_joltage(bank: &[u32]) -> u64 {
    let mut result: u64 = 0;
    let mut last = 0;

    for rem in (0..12).rev() {
        let max_val = bank[last..bank.len() - rem].iter().max().unwrap();
        let index = bank[last..bank.len() - rem]
            .iter()
            // Returns first index of max_val
            .position(|&x| x == *max_val)
            .unwrap();
        last = last + index + 1;
        result = result * 10 + (*max_val as u64);
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_works() {
        let result = part2(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn parts_work() {
        let bank: Vec<u32> = "987654321111111"
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(find_joltage(&bank), 987654321111);

        let bank: Vec<u32> = "811111111111119"
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(find_joltage(&bank), 811111111119);

        let bank: Vec<u32> = "234234234234278"
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(find_joltage(&bank), 434234234278);

        let bank: Vec<u32> = "818181911112111"
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(find_joltage(&bank), 888911112111);
    }
}
