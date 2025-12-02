fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i64 {
    input
        .split(',')
        .flat_map(|s| {
            let mut parts = s.trim().split('-');
            let start = parts.next().unwrap().trim().parse().unwrap();
            let end = parts.next().unwrap().trim().parse().unwrap();
            (start..=end).filter(has_digit_match)
        })
        .sum()
}

fn has_digit_match(n: &i64) -> bool {
    let s = n.to_string();

    s.len().is_multiple_of(2) && s[0..s.len() / 2] == s[s.len() / 2..]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, 1227775554);
    }
}
