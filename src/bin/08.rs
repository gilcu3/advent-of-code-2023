use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut moves: Vec<u8> = Vec::new();
    let mut mapl = HashMap::new();
    let mut mapr = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        if !line.trim().is_empty() {
            if i == 0 {
                for m in line.trim().as_bytes() {
                    if *m == b'L' {
                        moves.push(0);
                    } else {
                        moves.push(1);
                    }
                }
            } else {
                let mut v = "";
                let mut vl = "";
                let mut vr = "";
                for (j, c) in line.split_whitespace().enumerate() {
                    if j == 0 {
                        v = c;
                    } else if j == 2 {
                        vl = &c[1..c.len() - 1];
                    } else if j == 3 {
                        vr = &c[0..c.len() - 1];
                    }
                }
                mapl.insert(v, vl);
                mapr.insert(v, vr);
            }
        }
    }
    let mut cur = "AAA";
    let n = moves.len();
    let mut i: u32 = 0;
    while cur != "ZZZ" {
        let c = moves[i as usize % n];
        if c == 0 {
            cur = mapl.get(cur).unwrap();
        } else {
            cur = mapr.get(cur).unwrap();
        }
        i += 1;
    }
    Some(i)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut moves: Vec<u8> = Vec::new();
    let mut mapl = HashMap::new();
    let mut mapr = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        if !line.trim().is_empty() {
            if i == 0 {
                for m in line.trim().as_bytes() {
                    if *m == b'L' {
                        moves.push(0);
                    } else {
                        moves.push(1);
                    }
                }
            } else {
                let mut v = "";
                let mut vl = "";
                let mut vr = "";
                for (j, c) in line.split_whitespace().enumerate() {
                    if j == 0 {
                        v = c;
                    } else if j == 2 {
                        vl = &c[1..c.len() - 1];
                    } else if j == 3 {
                        vr = &c[0..c.len() - 1];
                    }
                }
                mapl.insert(v, vl);
                mapr.insert(v, vr);
            }
        }
    }
    let cur = mapl
        .keys()
        .filter(|&x| x.ends_with('A'))
        .collect::<Vec<&&str>>();
    let n = moves.len();
    let mut map2 = HashMap::new();
    let mut vals = Vec::new();
    for curi in cur {
        let mut t: u64 = 0;
        let mut ncur = curi;
        let mut ocur = curi;
        while !map2.contains_key(&(t % n as u64, ocur)) {
            let ot = t;

            while t == ot || !ncur.ends_with('Z') {
                if moves[(t % n as u64) as usize] == 0 {
                    ncur = mapl.get(ncur).unwrap();
                } else {
                    ncur = mapr.get(ncur).unwrap();
                }
                t += 1;
            }
            // why does this happen? -> very special graph lol
            //assert!((t - ot) % n as u64 == 0);
            map2.insert((ot % n as u64, ocur), (t - ot, ncur));
            vals.push(t - ot);
            ocur = ncur;
        }
    }
    // println!("{:?}", map2);
    let mut ans = 1;
    for v in vals {
        ans = num::integer::lcm(ans, v);
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
