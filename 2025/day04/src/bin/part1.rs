fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i32 {
    println!("{}", input);
    println!();

    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => false,
                })
                .collect()
        })
        .collect();

    //for row in grid {
    //    for cell in row {
    //        match cell {
    //            true => print!("@"),
    //            false => print!("."),
    //        }
    //    }
    //
    //    println!();
    //}

    let mut accessible: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    for row in 0..grid.len() {
        '_cols: for col in 0..grid[row].len() {
            if !grid[row][col] {
                continue;
            }

            let mut count = 0;

            for d_row in -1..=1 {
                for d_col in -1..=1 {
                    if d_row == 0 && d_col == 0 {
                        continue;
                    }

                    let new_row = row as i32 + d_row;
                    let new_col = col as i32 + d_col;

                    if let Some(row_vec) = grid.get(new_row as usize)
                        && let Some(&value) = row_vec.get(new_col as usize)
                        && value
                    {
                        count += 1;
                    }
                }
            }

            accessible[row][col] = count < 4;
        }
    }

    accessible.iter().flatten().filter(|c| **c).count() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert_eq!(result, 13)
    }
}
