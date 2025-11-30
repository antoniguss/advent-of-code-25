use regex::{Captures, Regex};
fn main() {
    let input = include_str!("input1.txt");
    let result = part2(input);

    dbg!(result);
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let regex_rev = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();

    input.lines().for_each(|line| {
        println!("{}", &line);
        let line_rev: String = line.chars().rev().collect();

        let captures: Vec<Captures> = regex.captures_iter(line).collect();
        let first_match = &captures[0][1];

        let captures_rev: Vec<Captures> = regex_rev.captures_iter(&line_rev).collect();
        let last_match = &captures_rev[0][1].chars().rev().collect::<String>();

        dbg!(first_match);
        dbg!(last_match);

        fn word_to_digit(s: &str) -> i32 {
            match s {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => s.parse::<i32>().unwrap(),
            }
        }

        let first_n = word_to_digit(first_match);
        let last_n = word_to_digit(last_match);

        let calibration_values = first_n * 10 + last_n;
        println!("{}", calibration_values);

        sum += calibration_values;
    });

    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
        let result = part2("twone");
        assert_eq!(result, 21);
    }
}
