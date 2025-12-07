use std::collections::HashSet;

fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i32 {
    let len = input.lines().count();
    let line_len = input.lines().next().unwrap().len();

    let splits: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^' || *c == 'S')
                .map(|(i, _)| i)
                .collect()
        })
        .collect();

    let first = *splits[0].first().unwrap();
    let mut beams = HashSet::new();
    beams.insert(first);

    let mut n_splits = 0;
    for i in 1..len / 2 {
        let mut new_beams = HashSet::new();

        for b in &beams {
            if splits[i].contains(b) {
                n_splits += 1;
                new_beams.insert(*b - 1);
                new_beams.insert(*b + 1);
            } else {
                new_beams.insert(*b);
            }
        }

        beams = new_beams;
    }

    n_splits
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!(result, 21)
    }
}
