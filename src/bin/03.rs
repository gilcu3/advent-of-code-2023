use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let (n, m) = (lines.len(), lines[0].len());

    let moves = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut ans: u32 = 0;
    for i in 0..n {
        let mut j = 0;
        while j < m {
            let j0 = j;
            while j < m && lines[i].as_bytes()[j].is_ascii_digit() {
                j += 1;
            }
            if j > j0 {
                let mut possible = false;
                let mut cur: u32 = 0;
                for j1 in j0..j {
                    cur = 10 * cur + (lines[i].as_bytes()[j1] - b'0') as u32;
                    for d in moves {
                        let (i1, j1) = (i as i32 + d.0, j1 as i32 + d.1);
                        if i1 >= 0 && i1 < n as i32 && j1 >= 0 && j1 < m as i32 {
                            let c = lines[i1 as usize].as_bytes()[j1 as usize];
                            if !c.is_ascii_digit() && c != b'.' {
                                possible = true;
                            }
                        }
                    }
                }
                if possible {
                    ans += cur;
                }
            }

            j += 1;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let (n, m) = (lines.len(), lines[0].len());

    let moves = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut ans: u32 = 0;
    let mut sol = HashMap::new();
    for i in 0..n {
        let mut j = 0;
        while j < m {
            let j0 = j;
            while j < m && lines[i].as_bytes()[j].is_ascii_digit() {
                j += 1;
            }
            if j > j0 {
                let mut cur: u32 = 0;
                for j1 in j0..j {
                    cur = 10 * cur + (lines[i].as_bytes()[j1] - b'0') as u32;
                }
                let mut seen = HashSet::new();
                for j1 in j0..j {
                    for d in moves {
                        let (i1, j1) = (i as i32 + d.0, j1 as i32 + d.1);
                        if i1 >= 0
                            && i1 < n as i32
                            && j1 >= 0
                            && j1 < m as i32
                            && !seen.contains(&(i1, j1))
                        {
                            seen.insert((i1, j1));
                            let c = lines[i1 as usize].as_bytes()[j1 as usize];
                            if c == b'*' {
                                let it = sol.entry((i1, j1)).or_insert((0, 1));
                                if it.0 <= 2 {
                                    *it = (it.0 + 1, it.1 * cur);
                                }
                            }
                        }
                    }
                }
            }

            j += 1;
        }
    }
    for (_gear, (val, prod)) in sol {
        if val == 2 {
            ans += prod;
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
