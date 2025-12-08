use std::collections::HashSet;

fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Box {
    fn dist_squared(&self, other: &Box) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn part2(input: &str) -> i64 {
    let boxes: Vec<Box> = input
        .lines()
        .map(|l| {
            let vals: Vec<i64> = l.split(',').map(|n| n.parse().unwrap()).collect();
            Box {
                x: vals[0],
                y: vals[1],
                z: vals[2],
            }
        })
        .collect();

    let mut connections: Vec<HashSet<&Box>> = vec![];
    for b in &boxes {
        let mut set = HashSet::new();
        set.insert(b);
        connections.push(set);
    }

    let mut pairs: Vec<(&Box, &Box)> = vec![];

    for (i, b) in boxes.iter().enumerate() {
        for b2 in boxes[(i + 1)..].iter() {
            pairs.push((b, b2));
        }
    }

    pairs.sort_by_key(|(b1, b2)| b1.dist_squared(b2));

    //pairs
    //    .iter()
    //    .for_each(|(b1, b2)| println!("{:?} - {:?} = {}", b1, b2, b1.dist_squared(b2)));

    let mut pairs = pairs.iter();

    let mut last_b1 = Box { x: 0, y: 0, z: 0 };
    let mut last_b2 = Box { x: 0, y: 0, z: 0 };
    while connections.len() > 1 {
        let (b1, b2) = pairs.next().unwrap();
        last_b1 = (*b1).clone();
        last_b2 = (*b2).clone();

        // Find group that contains b1
        let mut g1_index = connections.iter().position(|c| c.contains(b1)).unwrap();
        let g2_index = connections.iter().position(|c| c.contains(b2)).unwrap();

        // Check if group (so both groups are the same)
        if g1_index == g2_index {
            continue;
        }

        // Remove g2 from connections
        let g2 = connections.remove(g2_index);

        if g2_index < g1_index {
            g1_index -= 1;
        }
        let g1 = &mut connections[g1_index];

        // Add all elements of g2 into g1
        g1.extend(g2.iter());

        //println!(
        //    "Connected\t({0},{1},{2}) with\n\t\t({3},{4},{5})",
        //    b1.x, b1.y, b1.z, b2.x, b2.y, b2.z
        //);
        //
        //println!("Current conn:");
        //g1.iter().for_each(|b| {
        //    println!("\t({}, {}, {})", b.x, b.y, b.z);
        //});
        //}

        //dbg!(&connections.len());
    }

    last_b1.x * last_b2.x
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );
        assert_eq!(result, 25272)
    }
}
