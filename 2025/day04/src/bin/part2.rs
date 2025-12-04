fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i32 {
    println!("{}", input);
    println!();

    let mut grid: Vec<Vec<bool>> = input
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

    let mut result = 0;
    loop {
        let accessible = get_accessible(&grid);

        let count = accessible.iter().flatten().filter(|c| **c).count();
        result += count;
        if count == 0 {
            break;
        }

        // Remove papers from grid if accesible
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if accessible[row][col] {
                    grid[row][col] = false;
                }
            }
        }
    }

    result as i32
}

fn get_accessible(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
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
    accessible
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
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
        assert_eq!(result, 43)
    }
}
