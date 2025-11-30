fn main() {
    let input = include_str!("input1.txt");

    let result = part1(input);

    dbg!(&result);
}

fn part1(input: &str) -> i32 {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    reports.iter().map(|report| report_safe(report)).sum()
}

fn report_safe(report: &[i32]) -> i32 {
    let is_increasing: bool = report[0] < report[1];
    //dbg!(report);

    for i in 0..report.len() - 1 {
        // Check increasing/decreasing
        if is_increasing && report[i] >= report[i + 1] {
            return 0;
        }
        if !is_increasing && report[i] <= report[i + 1] {
            return 0;
        }

        // Check if difference between 1 and 3
        let diff = report[i].abs_diff(report[i + 1]);

        if diff == 0 || diff > 3 {
            return 0;
        }
    }

    1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 2)
    }

    #[test]
    fn report_test() {
        let report1 = [1, 2, 3, 4, 5];

        assert_eq!(1, report_safe(&report1));

        let report1 = [1, 4, 3, 4, 5];

        assert_eq!(0, report_safe(&report1));

        let report1 = [1, 1, 3, 4, 5];

        assert_eq!(0, report_safe(&report1));

        let report1 = [1, 2, 1, 2, 1];

        assert_eq!(0, report_safe(&report1));

        let report1 = [1, 4, 7, 10, 13];

        assert_eq!(1, report_safe(&report1));

        let report1 = [13, 10, 7, 4, 1];

        assert_eq!(1, report_safe(&report1));

        let report1 = [13, 10, 7, 4, 1];

        assert_eq!(1, report_safe(&report1));

        let report1 = [40, 42, 45, 45, 49, 47];

        assert_eq!(0, report_safe(&report1));

        let report1 = [65, 66, 68, 71, 72, 72];

        assert_eq!(0, report_safe(&report1));
    }
}
