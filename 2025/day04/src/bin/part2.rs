fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i32 {
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

fn get_accessible(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut accessible: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if !grid[row][col] {
                continue;
            }

            let count = (-1..=1)
                .flat_map(|d_row| {
                    (-1..=1).filter_map(move |d_col| {
                        if d_row == 0 && d_col == 0 {
                            return None;
                        }
                        let new_row = (row as i32 + d_row) as usize;
                        let new_col = (col as i32 + d_col) as usize;
                        grid.get(new_row)
                            .and_then(|r| r.get(new_col))
                            .filter(|&&v| v)
                            .map(|_| 1)
                    })
                })
                .sum::<usize>();

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
