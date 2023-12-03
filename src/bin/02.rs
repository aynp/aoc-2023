advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    const MAX_ALLOWED_RED: u32 = 12;
    const MAX_ALLOWED_GREEN: u32 = 13;
    const MAX_ALLOWED_BLUE: u32 = 14;

    Some(
        input
            .lines()
            .map(|line| {
                let (game, all_sets) = line.split_once(':').unwrap_or(("", ""));

                let idx = game
                    .trim()
                    .split_once(' ')
                    .unwrap_or(("", ""))
                    .1
                    .parse::<u32>()
                    .unwrap_or(0);

                all_sets
                    .trim()
                    .split(';')
                    .map(|set| {
                        set.split(',')
                            .map(|draw| {
                                let (count, color) =
                                    draw.trim().split_once(' ').unwrap_or(("0", ""));

                                match color {
                                    "red" => (count.parse::<u32>().unwrap_or(0), 0, 0),
                                    "green" => (0, count.parse::<u32>().unwrap_or(0), 0),
                                    "blue" => (0, 0, count.parse::<u32>().unwrap_or(0)),
                                    _ => (0, 0, 0),
                                }
                            })
                            .fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
                                (max_r.max(r), max_g.max(g), max_b.max(b))
                            })
                    })
                    .fold((idx, 0, 0, 0), |(idx, max_r, max_g, max_b), (r, g, b)| {
                        (idx, max_r.max(r), max_g.max(g), max_b.max(b))
                    })
            })
            .fold(0, |count, (idx, r, g, b)| {
                if r <= MAX_ALLOWED_RED && g <= MAX_ALLOWED_GREEN && b <= MAX_ALLOWED_BLUE {
                    return count + idx;
                } else {
                    return count;
                }
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, all_sets) = line.split_once(':').unwrap_or(("", ""));

                all_sets
                    .trim()
                    .split(';')
                    .map(|set| {
                        set.split(',')
                            .map(|draw| {
                                let (count, color) =
                                    draw.trim().split_once(' ').unwrap_or(("0", ""));

                                match color {
                                    "red" => (count.parse::<u32>().unwrap_or(0), 0, 0),
                                    "green" => (0, count.parse::<u32>().unwrap_or(0), 0),
                                    "blue" => (0, 0, count.parse::<u32>().unwrap_or(0)),
                                    _ => (0, 0, 0),
                                }
                            })
                            .fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
                                (max_r.max(r), max_g.max(g), max_b.max(b))
                            })
                    })
                    .fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
                        (max_r.max(r), max_g.max(g), max_b.max(b))
                    })
            })
            .fold(0, | power, (r, g, b)| {
                power + r * g * b
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
