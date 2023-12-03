use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first_digit = line.chars().find(|c| c.is_numeric()).unwrap_or('0');
                let last_digit = line.chars().rev().find(|c| c.is_numeric()).unwrap_or('0');

                format!("{first_digit}{last_digit}")
                    .parse::<u32>()
                    .unwrap_or(0)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    Some(
        input
            .lines()
            .map(|line| {
                let first_digit_idx = numbers
                    .clone()
                    .into_iter()
                    .filter_map(|(key, value)| line.find(key).map(|index| (index, value)))
                    .min()
                    .unwrap_or((0, 0));

                let last_digit_idx = numbers
                    .clone()
                    .into_iter()
                    .filter_map(|(key, value)| line.rfind(key).map(|index| (index, value)))
                    .max()
                    .unwrap_or((0, 0));

                let first_digit = first_digit_idx.1;
                let last_digit = last_digit_idx.1;

                format!("{first_digit}{last_digit}")
                    .parse::<u32>()
                    .unwrap_or(0)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
