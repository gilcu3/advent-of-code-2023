use std::collections::HashMap;

advent_of_code::solution!(12);

fn rec(
    card: &str,
    pos: &Vec<usize>,
    ii: usize,
    ij: usize,
    mem: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if ii == card.len() {
        if ij == pos.len() {
            return 1;
        } else {
            return 0;
        }
    }
    if mem.contains_key(&(ii, ij)) {
        return *mem.get(&(ii, ij)).unwrap();
    }
    let mut ans = 0;
    let c = card.chars().nth(ii).unwrap();
    match c {
        '#' => {
            if ij == pos.len() {
                return 0;
            }
            for j in 0..pos[ij] + 1 {
                if ii + j >= card.len() {
                    return 0;
                }
                if j == pos[ij] && card.chars().nth(ii + j).unwrap() == '#' {
                    return 0;
                }
                if j < pos[ij] && card.chars().nth(ii + j).unwrap() == '.' {
                    return 0;
                }
            }
            ans = rec(card, pos, ii + pos[ij] + 1, ij + 1, mem);
        }
        '.' => {
            ans = rec(card, pos, ii + 1, ij, mem);
        }
        '?' => {
            if ij != pos.len() {
                let mut poss = true;
                for j in 0..pos[ij] + 1 {
                    if ii + j >= card.len() {
                        poss = false;
                        break;
                    }
                    if j == pos[ij] && card.chars().nth(ii + j).unwrap() == '#' {
                        poss = false;
                        break;
                    }
                    if j < pos[ij] && card.chars().nth(ii + j).unwrap() == '.' {
                        poss = false;
                        break;
                    }
                }
                if poss {
                    ans += rec(card, pos, ii + pos[ij] + 1, ij + 1, mem);
                }
            }
            ans += rec(card, pos, ii + 1, ij, mem);
        }
        _ => {
            panic!("Invalid char");
        }
    }
    mem.insert((ii, ij), ans);
    ans
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ans = 0;
    for line in input.lines() {
        let (card, nums) = line.split_once(' ').unwrap();

        let pos: Vec<usize> = nums
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let mut scard = card.trim().to_string();
        scard += ".";
        let mut mem = HashMap::new();
        let cur = rec(scard.as_str(), &pos, 0, 0, &mut mem);
        ans += cur;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ans = 0;
    for line in input.lines() {
        let (card, nums) = line.split_once(' ').unwrap();

        let pos: Vec<usize> = nums
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let scard = card.trim().to_string();
        let mut ncard = scard.clone();
        let mut npos = pos.clone();
        for _ in 0..4 {
            ncard += "?";
            ncard += card;
            npos.extend(pos.iter());
        }
        ncard += ".";
        let mut mem = HashMap::new();
        let cur = rec(ncard.as_str(), &npos, 0, 0, &mut mem);
        ans += cur;
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
