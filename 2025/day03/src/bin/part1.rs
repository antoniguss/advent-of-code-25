fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let bank: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            find_joltage(&bank)
        })
        .sum()
}

fn find_joltage(bank: &[u32]) -> u32 {
    let mut max = 0;
    for right in (0..bank.len()).rev() {
        for left in 0..right {
            if bank[left] * 10 + bank[right] > max {
                max = bank[left] * 10 + bank[right];
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_works() {
        let result = part1(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(result, 357);
    }

    #[test]
    fn parts_work() {
        let bank = [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1];
        assert_eq!(find_joltage(&bank.to_vec()), 98);

        let bank = [8, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(find_joltage(&bank.to_vec()), 89);

        let bank = [2, 3, 4, 3, 4, 5, 3, 4, 2, 3, 4, 7, 8];
        assert_eq!(find_joltage(&bank.to_vec()), 78);

        let bank = [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(find_joltage(&bank.to_vec()), 92);
    }
}
