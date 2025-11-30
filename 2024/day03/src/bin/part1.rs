use regex::Regex;

fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i32 {
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    mul_re
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result =
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161)
    }
}
