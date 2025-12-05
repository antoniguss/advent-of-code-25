fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();

    let mut ranges: Vec<(i64, i64)> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split('-');

            let start = parts.next().unwrap().trim().parse().unwrap();
            let end = parts.next().unwrap().trim().parse().unwrap();

            (start, end)
        })
        .collect();
    let mut result = 0;

    ranges.sort_unstable_by(|(a_1, _), (a_2, _)| a_1.cmp(a_2));

    ranges.iter().for_each(|(a, b)| println!("{},{}", a, b));

    let mut prev = 0;
    ranges.iter().for_each(|(a, b)| {
        if prev <= *b {
            let start = a.max(&prev);

            println!("{}", b - start + 1);
            result += b - start + 1;
            prev = *b + 1;
        } else {
            println!("Skipping {}, {}", a, b);
        }
    });

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "3-5
        10-14
        16-20
        12-18
        12-14
        13-13

        1
        5
        8
        11
        17
        32",
        );
        assert_eq!(result, 14);

        let result = part2(
            "200-300
100-101
1-1
2-2
3-3
1-3
1-3
2-2
50-70
10-10
98-99
99-99
99-99
99-100
1-1
2-1
100-100
100-100
100-101
200-300
201-300
202-300
250-251
98-99
100-100
100-101
1-101",
        );

        assert_eq!(result, 202);

        let result = part2(
            "1-3
3-5",
        );

        assert_eq!(result, 5);

        let result = part2(
            "200-300
100-101
1-1
2-2
3-3
1-3
1-3
2-2
50-70
10-10
98-99
99-99
99-99
99-100
1-1
2-1
100-100
100-100
100-101
200-300
201-300
202-300
250-251
98-99
100-100
100-101
1-101",
        );

        assert_eq!(result, 202)
    }

    #[test]
    fn part2_contiguous() {
        let result = part2("3-5\n6-6");
        assert_eq!(result, 4);

        let result = part2("0-0");
        assert_eq!(result, 1);
    }
}
