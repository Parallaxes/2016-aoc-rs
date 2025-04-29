advent_of_code::solution!(1);

use std::collections::HashSet;

// There's probably a much more elegant solution, but oh well.
pub fn part_one(input: &str) -> Option<u64> {
    let replaced_input = input.replace(",", "");
    let coords: Vec<&str> = replaced_input.split_whitespace().collect();

    let mut pos = Pos { x: 0, y: 0};
    let mut dir = 0;

    for coord in coords {
        let turn = coord.chars().next()?;
        let distance = coord[1..].parse::<i32>().ok()?;

        dir = match turn {
            'R' => (dir + 1) % 4,
            'L' => (dir + 3) % 4,
            _ => panic!("Invalid turn direction"),
        };

        let (dx, dy) = match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => unreachable!(),
        };

        for _ in 0..distance {
            pos.x += dx;
            pos.y += dy;
        }
    }

    Some((pos.x.abs() + pos.y.abs()).try_into().ok()?)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut set: HashSet<Pos> = HashSet::new();
    let replaced_input = input.replace(",", "");
    let coords: Vec<&str> = replaced_input.split_whitespace().collect();

    let mut pos = Pos { x: 0, y: 0};
    let mut dir = 0;

    for coord in coords {
        let turn = coord.chars().next()?;
        let distance = coord[1..].parse::<i32>().ok()?;

        dir = match turn {
            'R' => (dir + 1) % 4,
            'L' => (dir + 3) % 4,
            _ => panic!("Invalid turn direction"),
        };

        let (dx, dy) = match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => unreachable!(),
        };

        for _ in 0..distance {
            pos.x += dx;
            pos.y += dy;

            if set.contains(&pos) {
                return Some((pos.x.abs() + pos.y.abs()).try_into().ok()?);
            }
            set.insert(pos);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        println!("{:?}", result);
        assert_eq!(result, Some(4));
    }
}