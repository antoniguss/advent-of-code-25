fn main() {
    let input = include_str!("input2.txt");

    let result = part2(input);

    dbg!(&result);
    //    let result = part2(
    //        "L47
    //R26
    //L18
    //R18
    //R26
    //R15
    //L33
    //R8
    //L26
    //L25
    //R6
    //L47
    //L7
    //R40
    //L1
    //L4
    //R43
    //L30
    //R3
    //L1
    //R8
    //L18
    //L25
    //",
    //    );
    //
    //    dbg!(&result);
    //
    //    println!("{}", 100_i32.rem_euclid(100));
    println!("{}", -100_i32.div_euclid(100));
}

fn part2(input: &str) -> i32 {
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

    let mut start_pos = 50;
    let mut pos: i32 = 50;
    let mut result = 0;
    rotations.iter().for_each(|amount| {
        //curr_position = (prev_position + *amount) % 100;
        //dbg!(curr_position);
        //if (curr_position * prev_position).signum() == -1 {
        //    result += 1;
        //}
        //prev_position = curr_position;

        // Check how many times it will go over 0, might be more than 1
        dbg!(pos);
        pos += *amount;
        dbg!(amount);
        dbg!(pos);

        if start_pos == 0 && pos < 0 {
            result += (-pos).div_euclid(100);
        } else if start_pos != 0 && pos <= 0 {
            result += 1;
            result += (-pos).div_euclid(100);
        } else {
            result += pos.div_euclid(100);
        }
        pos = pos.rem_euclid(100);
        dbg!(pos);
        start_pos = pos;
    });

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        //        let result = part2(
        //            "L68
        //L30
        //R48
        //L5
        //R60
        //L55
        //L1
        //L99
        //R14
        //L82
        //R200
        //R200
        //L200
        //L68",
        //        );
        //        assert_eq!(result, 13);
        //
        //        let result = part2("R1000");
        //        assert_eq!(result, 10);
        //
        let result = part2(
            "L70
R650
L123
R23
R51
L82
L69
L814
R630
L79",
        );

        assert_eq!(result, 25);

        let result = part2(
            "L50
L200
",
        );

        assert_eq!(result, 3);
    }
}
