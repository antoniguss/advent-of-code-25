fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i32 {
    let rotations: Vec<i32> = input
        .lines()
        .map(|rotation| {
            let mut chars = rotation.chars();
            let direction = chars.next().unwrap();
            let amount: String = chars.collect();

            match direction {
                'L' => -amount.parse::<i32>().unwrap(),
                'R' => amount.parse::<i32>().unwrap(),
                _ => {
                    panic!("Incorrect direction")
                }
            }
        })
        .collect();

    let mut position = 50;
    rotations
        .iter()
        .filter(|amount| {
            position += *amount;
            dbg!(position);
            position % 100 == 0
        })
        .count() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(result, 3)
    }
}
