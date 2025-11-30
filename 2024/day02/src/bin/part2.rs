fn main() {
    let input = include_str!("input1.txt");

    let result = part2(input);

    dbg!(&result);
}

fn part2(input: &str) -> i32 {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    dbg!(&reports);

    // Naive approach: try removing each of the elements and checking if safe
    let safe_reports: Vec<Vec<i32>> = reports
        .iter()
        .filter_map(|report| {
            if report_safe(report) {
                return Some(report.clone());
            }

            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if report_safe(&new_report) {
                    return Some(report.clone());
                }
            }

            None
        })
        .collect();

    safe_reports.len() as i32
}
fn report_safe(report: &[i32]) -> bool {
    let is_increasing: bool = report[0] < report[1];
    //dbg!(report);

    for i in 0..report.len() - 1 {
        // Check increasing/decreasing
        if is_increasing && report[i] >= report[i + 1] {
            return false;
        }
        if !is_increasing && report[i] <= report[i + 1] {
            return false;
        }

        // Check if difference between 1 and 3
        let diff = report[i].abs_diff(report[i + 1]);

        if diff == 0 || diff > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 4);
    }
}
