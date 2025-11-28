fn main() {
    let input = include_str!("input1.txt");
    let result = part1(input);

    dbg!(result);
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let ints: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        dbg!(&ints);
        sum += 10 * ints[0] + ints.last().unwrap()
    });

    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
