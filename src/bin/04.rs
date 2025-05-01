advent_of_code::solution!(4);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    for line in input.lines() {
        let mut freqs: HashMap<char, i32> = HashMap::new();
        let room: Vec<char> = line.trim().chars().collect();

        let mut idx = 0;
        for i in 0..room.len() {
            if room[i] == '[' {
                break;
            }

            if room[i] != '-' {
                *freqs.entry(room[i]).or_insert(0) += 1;
            }

            idx += 1;
        }

        let mut sorted_freqs: Vec<(&char, &i32)> = freqs.iter().collect();
        sorted_freqs.sort_by_key(|&(key, _)| key);

        let token = &room[idx..room.len() - 1];
        for c in token {
            
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
