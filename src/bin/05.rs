use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut cur = HashSet::<u64>::new();
    let mut nx = HashSet::<u64>::new();

    for line in input.lines() {
        if line.starts_with("seeds") {
            line.split_whitespace().skip(1).for_each(|x| {
                nx.insert(x.parse::<u64>().unwrap());
            });
        } else if line.trim().is_empty() {
            for n in cur {
                nx.insert(n);
            }
            cur = nx.clone();
            nx = HashSet::<u64>::new();
        } else if line.contains(':') {
        } else {
            let conv = line
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let ocur = cur.clone().into_iter().collect::<Vec<u64>>();
            for n in ocur {
                if n >= conv[1] && n < conv[1] + conv[2] {
                    nx.insert(n - conv[1] + conv[0]);
                    cur.remove(&n);
                }
            }
        }
    }
    for n in cur {
        nx.insert(n);
    }
    let ans: u64 = *nx.iter().min().unwrap();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cur = HashSet::<(u64, u64)>::new();
    let mut nx = HashSet::<(u64, u64)>::new();

    for line in input.lines() {
        if line.starts_with("seeds") {
            let mut vv = Vec::<u64>::new();
            line.split_whitespace().skip(1).for_each(|x| {
                vv.push(x.parse::<u64>().unwrap());
            });
            for i in (0..vv.len()).step_by(2) {
                nx.insert((vv[i], vv[i + 1]));
            }
        } else if line.trim().is_empty() {
            for n in cur {
                nx.insert(n);
            }
            cur = nx.clone();
            nx = HashSet::<(u64, u64)>::new();
        } else if line.contains(':') {
        } else {
            let conv = line
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let ocur = cur.clone().into_iter().collect::<Vec<(u64, u64)>>();
            for (n, jn) in ocur {
                if n + jn > conv[1] && n < conv[1] + conv[2] {
                    let (b, e) = (
                        std::cmp::max(n, conv[1]),
                        std::cmp::min(n + jn, conv[1] + conv[2]),
                    );
                    nx.insert((b - conv[1] + conv[0], e - b));
                    cur.remove(&(n, jn));
                    if e < n + jn {
                        cur.insert((e, n + jn - e));
                    }
                    if b > n {
                        cur.insert((n, b - n));
                    }
                }
            }
        }
    }
    for n in cur {
        nx.insert(n);
    }
    let ans: u64 = *nx.iter().map(|(a, _b)| a).min().unwrap();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
