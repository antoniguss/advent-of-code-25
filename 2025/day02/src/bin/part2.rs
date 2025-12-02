fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i64 {
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

    // Check all possible divisors
    let len = s.len();
    let mut divs = divisors::get_divisors(len);
    divs.push(1);

    divs.into_iter().filter(|&n| n != len).rev().any(|div| {
        let mut v = vec![];
        let mut cur = s.as_str();
        while !cur.is_empty() {
            let (chunk, rest) = cur.split_at(std::cmp::min(div, cur.len()));
            v.push(chunk.to_string());
            cur = rest;
        }

        v.windows(2).all(|w| w[0] == w[1])
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, 4174379265)
    }
}
