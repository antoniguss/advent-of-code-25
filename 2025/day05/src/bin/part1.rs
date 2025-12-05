fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i64 {
    let mut lines = input.lines();

    let ranges: Vec<(i64, i64)> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split('-');

            let start = parts.next().unwrap().trim().parse().unwrap();
            let end = parts.next().unwrap().trim().parse().unwrap();

            (start, end)
        })
        .collect();

    // Skip whitespace
    lines.by_ref().next();

    // Continue using iterator
    lines
        .by_ref()
        .filter(|l| {
            let id: i64 = l.parse().unwrap();
            for (start, end) in &ranges {
                if (start..=end).contains(&&id) {
                    return true;
                }
            }
            false
        })
        .count() as i64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(result, 3)
    }
}
