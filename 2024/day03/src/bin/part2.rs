use regex::Regex;

fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i32 {
    let input: String = input.lines().collect();

    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let mut is_enabled = true;
    mul_re.captures_iter(&input).for_each(|c| match &c[0] {
        "do()" => {
            is_enabled = true;
        }
        "don't()" => {
            is_enabled = false;
        }
        _ => {
            if is_enabled {
                let a = c[1].parse::<i32>().unwrap();
                let b = c[2].parse::<i32>().unwrap();

                result += a * b;
            }
        }
    });

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48)
    }
}
