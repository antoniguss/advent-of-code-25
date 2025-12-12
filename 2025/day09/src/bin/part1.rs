fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i64 {
    let mut tiles: Vec<(i64, i64)> = input
        .lines()
        .map(|l| {
            let nums: Vec<i64> = l.split(",").map(|c| c.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    tiles.sort_by_key(|t| t.0);

    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| {
            tiles
                .iter()
                .skip(i)
                //.filter(|t2| t2.0 >= t1.0 && t2.1 >= t1.1)
                .map(move |t2| (*t1, *t2))
        })
        .map(|(t1, t2)| (t2.0.abs_diff(t1.0) + 1) * (t2.1.abs_diff(t1.1) + 1))
        .max()
        .unwrap() as i64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(result, 50)
    }
}
