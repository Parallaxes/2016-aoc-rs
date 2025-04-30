advent_of_code::solution!(2);

static KEYPAD: [[i32; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos = (1, 1);
    let mut result = String::new();

    for line in input.lines() {
        let orders = line.trim().chars();
        for order in orders {
            let (mut dx, mut dy) = match order {
                'R' => (0, 1),
                'L' => (0, -1),
                'D' => (1, 0),
                'U' => (-1, 0),
                _ => unreachable!(),
            };
    
            if (pos.0 as i32 + dx) > 2 || (pos.0 as i32 + dx) < 0 || (pos.1 as i32 + dy) > 2 || (pos.1 as i32 + dy) < 0 {
                dx = 0;
                dy = 0;
            }

            pos.0 = (pos.0 as i32 + dx) as usize;
            pos.1 = (pos.1 as i32 + dy) as usize;
        }
        result += &KEYPAD[pos.0 as usize][pos.1 as usize].to_string();
    }
    
    result.parse::<u64>().ok()
}


static KEYPAD_2: &[&[&str]] = &[
    &[" ", " ", "1", " ", " "],
    &[" ", "2", "3", "4", " "],
    &["5", "6", "7", "8", "9"],
    &[" ", "A", "B", "C", " "],
    &[" ", " ", "D", " ", " "],
];

pub fn part_two(input: &str) -> Option<String> {
    let mut pos = (2, 0);
    let mut result = String::new();

    for line in input.lines() {
        let orders = line.trim().chars();
        for order in orders {
            let (mut dx, mut dy) = match order {
                'R' => (0, 1),
                'L' => (0, -1),
                'D' => (1, 0),
                'U' => (-1, 0),
                _ => unreachable!(),
            };

            if (pos.0 as i32 + dx) > 4 || (pos.0 as i32 + dx) < 0 || (pos.1 as i32 + dy) > 4 || (pos.1 as i32 + dy) < 0 || KEYPAD_2[(pos.0 as i32 + dx) as usize][(pos.1 as i32 + dy) as usize] == " " {
                dx = 0;
                dy = 0;
            }

            pos.0 = (pos.0 as i32 + dx) as usize;
            pos.1 = (pos.1 as i32 + dy) as usize;
        }

        result += &KEYPAD_2[pos.0 as usize][pos.1 as usize].to_string();
    }
    
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1985));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5DB3".to_string()));
    }
}
