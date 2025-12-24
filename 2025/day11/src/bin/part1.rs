use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i32 {
    // We'll use a hashset from input to a set of outputs
    let connections: HashMap<String, HashSet<String>> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(':');
            let input = parts.next()?.to_string();
            let output = parts
                .next()?
                .split_whitespace()
                .map(String::from)
                .collect::<HashSet<String>>();

            Some((input, output))
        })
        .collect(); // Collect into a HashMap

    // Run bfs to find all paths from start to end
    let mut result = 0;
    let mut paths: VecDeque<Vec<&str>> = VecDeque::new();

    paths.push_back(vec!["you"]);

    while !paths.is_empty() {
        let path = paths.pop_front().unwrap();
        let last = path.last().unwrap();
        if *last == "out" {
            result += 1;
        }

        if let Some(outs) = connections.get(*last) {
            for out in outs {
                if !path.contains(&out.as_str()) {
                    let mut new_path = path.clone();
                    new_path.push(out);
                    paths.push_back(new_path);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        );
        assert_eq!(result, 5)
    }
}
