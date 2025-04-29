advent_of_code::solution!(2);

const KEYPAD: [[i32; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];
pub fn part_one(input: &str) -> Option<u64> {
    let orders = input.chars();
    let mut pos = (1, 1);
    let mut result = String::new();

    for line in input.lines() {
        for order in orders.clone() {
            let (mut dx, mut dy) = match order {
                'R' => (0, 1),
                'L' => (1, 0),
                'D' => (0, -1),
                'U' => (-1, 0),
                _ => unreachable!(),
            };
    
            if pos.0 + dx > 2 || pos.0 + dx < 0 || pos.1 + dy > 2 || pos.1 + dy < 0 {
                (dx, dy) = (0, 0);
            }

            pos.0 += dx;
            pos.1 += dy;
        }
        result += &KEYPAD[pos.0 as usize][pos.1 as usize].to_string();
    }
    
    result.parse::<u64>().ok()
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
