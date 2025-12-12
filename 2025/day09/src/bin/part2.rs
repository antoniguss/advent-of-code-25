fn main() {
    let input = include_str!("input2.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i64 {
    let vertices: Vec<(i64, i64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let nums: Vec<i64> = l.split(',').map(|c| c.trim().parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let edges: Vec<_> = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .collect();

    vertices
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| vertices.iter().skip(i + 1).map(move |t2| (*t1, *t2)))
        .filter(|(t1, t2)| {
            let x_min = t1.0.min(t2.0);
            let x_max = t1.0.max(t2.0);
            let y_min = t1.1.min(t2.1);
            let y_max = t1.1.max(t2.1);

            // 1. Check if any polygon vertex is STRICTLY inside
            let vertex_inside = vertices
                .iter()
                .any(|v| v.0 > x_min && v.0 < x_max && v.1 > y_min && v.1 < y_max);

            if vertex_inside {
                return false;
            }

            // 2. Check if any polygon edge STRICTLY crosses
            let edge_crosses = edges.iter().any(|(p1, p2)| {
                let ex_min = p1.0.min(p2.0);
                let ex_max = p1.0.max(p2.0);
                let ey_min = p1.1.min(p2.1);
                let ey_max = p1.1.max(p2.1);

                // Vertical Edge check
                if p1.0 == p2.0 {
                    let vx = p1.0;
                    if vx > x_min && vx < x_max {
                        // Overlaps vertically, check if overlaps horizontally
                        return ey_min <= y_min && ey_max >= y_max;
                    }
                }

                // Horizontal Edge check
                if p1.1 == p2.1 {
                    let hy = p1.1;
                    if hy > y_min && hy < y_max {
                        // Overlaps vertically, check if overlaps vertically
                        return ex_min <= x_min && ex_max >= x_max;
                    }
                }
                false
            });

            !edge_crosses
        })
        .map(|(t1, t2)| (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1))
        .max()
        .unwrap_or(0) as i64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_example() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part2(input), 24);
    }
}
