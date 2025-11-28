fn main() {
    let input = include_str!("input2.txt");

    let result = part2(input);

    dbg!(&input);
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2("");
        assert_eq!(result, 0)
    }
}
