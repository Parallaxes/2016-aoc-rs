advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        let dimensions: Vec<i32> = line.trim().split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        let ok = {
            dimensions[0] + dimensions[1] > dimensions[2] && dimensions[0] + dimensions[2] > dimensions[1] && dimensions[1] + dimensions[2] > dimensions[0]
        };

        if ok {
            result += 1;
        }
    }

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        let dimensions: Vec<i32> = line.trim().split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        let ok = {
            dimensions[0] + dimensions[1] > dimensions[2] && dimensions[0] + dimensions[2] > dimensions[1] && dimensions[1] + dimensions[2] > dimensions[0]
        };

        if ok {
            result += 1;
        }
    }

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
