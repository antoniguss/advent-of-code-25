use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i64 {
    let len = input.lines().count();

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
    let mut beams: HashMap<usize, i64> = HashMap::new();
    beams.entry(first).or_insert(1);

    for i in 1..len / 2 {
        for v in beams.values() {
            print!("{} ", v);
        }
        println!();
        let mut new_beams: HashMap<usize, i64> = HashMap::new();

        for (b, v) in &beams {
            if splits[i].contains(b) {
                new_beams
                    .entry(b - 1)
                    .and_modify(|v2| *v2 += v)
                    .or_insert(*v);

                new_beams
                    .entry(b + 1)
                    .and_modify(|v2| *v2 += v)
                    .or_insert(*v);
            } else {
                new_beams.entry(*b).and_modify(|v2| *v2 += v).or_insert(*v);
            }
        }

        beams = new_beams;
    }
    beams.values().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
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
        assert_eq!(result, 40)
    }
}
