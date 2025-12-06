fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i64 {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // Transpose to get problems as rows
    let input: Vec<Vec<char>> = (0..input[0].len())
        .map(|i| input.iter().map(|inner| inner[i]).collect::<Vec<char>>())
        .collect();

    let mut result = 0;
    // Split by empty lines and process each section
    input
        .split(|line| line.iter().collect::<String>().trim().is_empty())
        .for_each(|chunk| {
            let mut chunks = chunk.iter();
            let first_line = chunks.next().unwrap();
            let op = first_line.last().unwrap();

            let mut vals: Vec<i64> = vec![];
            vals.push(
                first_line
                    .iter()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            );

            chunks
                .map(|l| l.iter().collect::<String>().trim().parse::<i64>().unwrap())
                .for_each(|v| vals.push(v));

            vals.iter().for_each(|val| println!("{}", val));

            match op {
                '+' => result += vals.iter().sum::<i64>(),
                '*' => result += vals.iter().product::<i64>(),
                _ => {}
            }

            println!("----------");
        });
    println!("----------");

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(result, 3263827);
    }
}
