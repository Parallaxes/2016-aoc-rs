advent_of_code::solution!(1);

// 1 -> Up
// 2 -> Right
// 3 -> Down
// 4 -> Left
struct Direction {
    direction: u8,
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords: Vec<&str> = input.split_whitespace().collect();
    let mut direction = Direction { direction: 2 };
    for coord in coords {
        if coord.chars().nth(0)? == 'R' {
            direction { direction + 1};
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
