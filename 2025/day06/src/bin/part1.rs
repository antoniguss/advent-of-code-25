fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i64 {
    let input: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    // Transpose to get problems as rows
    let input: Vec<Vec<&str>> = (0..input[0].len())
        .map(|i| input.iter().map(|inner| inner[i]).collect::<Vec<&str>>())
        .collect();

    let mut result = 0_i64;

    for p in &input {
        let i0 = p[0].parse::<i64>().unwrap();
        let i1 = p[1].parse::<i64>().unwrap();
        let i2 = p[2].parse::<i64>().unwrap();
        let i3 = p[3].parse::<i64>().unwrap();

        println!("{},{},{}, {}", i1, i2, i3, p[4]);

        match p[4] {
            "+" => result += i0 + i1 + i2 + i3,
            "*" => result += i0 * i1 * i2 * i3,
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        //        let result = part1(
        //            "123 328  51 64
        // 45 64  387 23
        //  6 98  215 314
        //*   +   *   +  ",
        //        );
        //        assert_eq!(result, 4277556);
    }
}
