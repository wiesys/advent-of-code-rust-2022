pub fn part_one(input: &str) -> Option<u32> {
    let mut max_calories = 0;
    let mut calories = 0;

    for (i, line) in input.lines().enumerate() {
        calories += line.parse::<u32>().unwrap_or_default();
        if !line.is_empty() && i != input.lines().count() - 1 {
            continue;
        }
        if max_calories < calories {
            max_calories = calories;
        }
        calories = 0;
    }

    Some(max_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut top_calories = [0, 0, 0];
    let mut calories = 0;

    for (i, line) in input.lines().enumerate() {
        calories += line.parse::<u32>().unwrap_or_default();
        if !line.is_empty() && i != input.lines().count() - 1 {
            continue;
        }
        if top_calories[0] < calories {
            top_calories[0] = calories;
            top_calories.sort();
        }
        calories = 0;
    }

    Some(top_calories.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
