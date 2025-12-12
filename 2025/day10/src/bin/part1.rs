use std::fmt;

use bit_field::BitField;
use itertools::Itertools;
fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltage: u32,
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Machine")
            .field("lights", &format!("{:b}", self.lights))
            .field(
                "buttons",
                &self
                    .buttons
                    .iter()
                    .map(|b| format!("{:b}", b))
                    .collect::<Vec<_>>(),
            )
            .field("joltage", &format!("{:b}", self.joltage))
            .finish()
    }
}

fn part1(input: &str) -> i32 {
    let machines: Vec<_> = input
        .lines()
        .map(|l| {
            let mut splits = l.split_whitespace();

            let lights: String = splits
                .next()
                .unwrap()
                .chars()
                .rev()
                .filter_map(|c| match c {
                    '[' | ']' => None, // skip first and last
                    '.' => Some('0'),
                    '#' => Some('1'),
                    _ => panic!("Unexpected char in lights: {c}"),
                })
                .collect();

            let lights = u32::from_str_radix(lights.as_str(), 2).unwrap();

            let buttons: Vec<u32> = splits
                .take_while(|s| s.starts_with('('))
                .map(|b_str| {
                    b_str
                        .chars()
                        .filter_map(|c| match c {
                            c if c.is_ascii_digit() => {
                                Some(2_i32.pow(c.to_digit(10).unwrap()) as u32)
                            }
                            '(' | ')' | ',' => None,
                            _ => panic!("Unexpected char at buttons: {c}"),
                        })
                        .sum()
                })
                .collect();
            Machine {
                lights,
                buttons,
                joltage: 0,
            }
        })
        .collect();

    machines
        .iter()
        .map(|m| {
            dbg!(m);
            (0..=m.buttons.len())
                .flat_map(|count| m.buttons.iter().combinations(count))
                .filter(|comb| m.lights == comb.iter().fold(0, |prev, &b| prev ^ b))
                //.for_each(|comb| {
                //    comb.iter().for_each(|b| {
                //        dbg!("{:b}\t", b);
                //    });
                //    dbg!();
                //})
                .map(|b| b.len())
                .min()
                .unwrap() as i32
            //dbg!("---");
        })
        .sum::<i32>()

    //machines.len() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
             [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
             [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(result, 7)
    }
}
