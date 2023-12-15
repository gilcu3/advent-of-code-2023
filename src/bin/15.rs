use std::collections::HashMap;

advent_of_code::solution!(15);

fn hash(s: String) -> u32 {
    let mut h: u32 = 0;
    for c in s.bytes() {
        h = (h + c as u32) * 17 % 256;
    }
    h
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;
    for line in input.lines() {
        for s in line.split(',') {
            ans += hash(s.to_string());
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hh: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    let mut hmap: HashMap<&str, isize> = HashMap::new();
    let mut ans = 0;
    for line in input.lines() {
        for s in line.split(',') {
            if s.contains('=') {
                let (s0, v0) = s.split_once('=').unwrap();
                let v = v0.parse::<usize>().unwrap();
                let h = hash(s0.to_string()) as usize;
                if hmap.contains_key(s0) && hmap[s0] >= 0 {
                    let j = hmap[s0] as usize;
                    hh[h][j].1 = v;
                } else {
                    hh[h].push((s0, v));
                    hmap.insert(s0, (hh[h].len() - 1) as isize);
                }
            } else {
                let (s0, _) = s.split_once('-').unwrap();
                hmap.insert(s0, -1);
            }
        }
    }
    for (i, hhi) in hh.iter().enumerate() {
        let mut slot = 0;
        for (j, (s, v)) in hhi.iter().enumerate() {
            if hmap[*s] == j as isize {
                ans += (i + 1) as u32 * (*v as u32) * (slot + 1);
                slot += 1;
            }
        }
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
