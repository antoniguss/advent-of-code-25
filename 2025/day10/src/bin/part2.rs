use good_lp::*;
use itertools::Itertools;

fn main() {
    let input = include_str!("input1.txt");
    let result = part2(input);
    dbg!(&result);
}

#[derive(Debug)]
struct Machine {
    lights: u32,
    buttons: Vec<Vec<u32>>,
    joltage: Vec<u32>,
}

fn part2(input: &str) -> i64 {
    let machines: Vec<_> = input
        .lines()
        .map(|l| {
            let mut splits = l.split_whitespace();

            splits.next();

            let buttons: Vec<Vec<u32>> = splits
                .take_while_ref(|&s| s.starts_with('('))
                .map(|b_str| {
                    b_str
                        .chars()
                        .filter_map(|c| match c {
                            c if c.is_ascii_digit() => Some(c.to_digit(10).unwrap()),
                            '(' | ')' | ',' => None,
                            _ => panic!("Unexpected char at buttons: {c}"),
                        })
                        .collect()
                })
                .collect();

            let joltage: Vec<u32> = splits
                .next()
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();

            Machine {
                lights: 0,
                buttons,
                joltage,
            }
        })
        .collect();

    machines.iter().map(find_min_pressess).sum()
}

fn find_min_pressess(machine: &Machine) -> i64 {
    let n = machine.buttons.len();

    let mut vars = ProblemVariables::new();
    let button_presses: Vec<_> = (0..n)
        .map(|_| vars.add(variable().min(0).integer())) // Now .integer() will be respected by coin_cbc
        .collect();

    let mut problem = vars
        .minimise(button_presses.iter().sum::<Expression>())
        .using(default_solver); // default_solver will now pick coin_cbc

    for (counter_idx, &target) in machine.joltage.iter().enumerate() {
        let constraint_expr = button_presses
            .iter()
            .enumerate()
            .map(|(button_idx, &press_var)| {
                let affects_counter = machine.buttons[button_idx].contains(&(counter_idx as u32));

                if affects_counter {
                    1 * press_var
                } else {
                    0 * press_var
                }
            })
            .sum::<Expression>();

        problem.add_constraint(constraint_expr.eq(target as i32));
    }

    let solution = problem.solve().unwrap();

    button_presses
        .iter()
        .map(|&var| solution.value(var).round() as i64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
             [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
             [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(result, 33)
    }
}

