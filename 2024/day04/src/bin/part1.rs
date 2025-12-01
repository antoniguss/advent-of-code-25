fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> u32 {
    let mut cols: u32 = 0;
    let mut rows: u32 = 0;
    let char_matrix: Vec<char> = input
        .lines()
        .flat_map(|l| {
            rows += 1;
            cols = l.len() as u32;
            let chars = l.chars();
            chars.collect::<Vec<char>>()
        })
        .collect();

    // Check horizontal matches
    for i in 0..rows {
        for j in 0..cols - 3 {
            println!(
                "{},{},{},{}",
                &char_matrix[(i * cols + j) as usize],
                &char_matrix[(i * cols + j + 1) as usize],
                &char_matrix[(i * cols + j + 2) as usize],
                &char_matrix[(i * cols + j + 3) as usize]
            )
        }
        println!();
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "MMMSXXMASM
MSAMXMSMSV
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 18)
    }
}
