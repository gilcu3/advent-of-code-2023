use std::collections::{HashMap, HashSet};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut work = HashMap::new();
    let mut ans = 0;
    for line in input.lines() {
        if !line.trim().is_empty() {
            if !line.starts_with('{') {
                let p = line.find('{').unwrap();
                let suf = &line[p + 1..line.len() - 1];
                let pref = &line[0..p];
                let mut rules = vec![];
                for r in suf.split(',') {
                    //println!("{}", r);
                    let c = r.chars().next().unwrap();

                    if r.find(':').is_none() {
                        rules.push((c, -1, false, r));
                    } else {
                        let pp = r.find(':').unwrap();
                        rules.push((
                            c,
                            r[2..pp].parse::<i32>().unwrap(),
                            r[1..2] == *">",
                            &r[pp + 1..r.len()],
                        ));
                    }
                }
                //println!("{} {}", pref, suf);
                work.insert(pref, rules);
                //work.insert(pref, suf);
            } else {
                let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);
                for c in line[1..line.len() - 1].split(',') {
                    let c0 = c.chars().next().unwrap();
                    match c0 {
                        'x' => x = c[2..c.len()].parse::<i32>().unwrap(),
                        'a' => a = c[2..c.len()].parse::<i32>().unwrap(),
                        'm' => m = c[2..c.len()].parse::<i32>().unwrap(),
                        's' => s = c[2..c.len()].parse::<i32>().unwrap(),
                        _ => panic!(),
                    }
                }
                let xmas = x + m + a + s;
                let mut cur = "in";
                let mut seen = HashSet::new();
                while !seen.contains(cur) {
                    seen.insert(cur);
                    if cur == "R" {
                        break;
                    }
                    if cur == "A" {
                        ans += xmas;
                        break;
                    }

                    let rules = &work[cur];
                    for rule in rules {
                        if rule.1 == -1 {
                            cur = rule.3;
                        } else {
                            let pass = match rule.0 {
                                'x' => {
                                    if rule.2 {
                                        x > rule.1
                                    } else {
                                        x < rule.1
                                    }
                                }
                                'm' => {
                                    if rule.2 {
                                        m > rule.1
                                    } else {
                                        m < rule.1
                                    }
                                }
                                'a' => {
                                    if rule.2 {
                                        a > rule.1
                                    } else {
                                        a < rule.1
                                    }
                                }
                                's' => {
                                    if rule.2 {
                                        s > rule.1
                                    } else {
                                        s < rule.1
                                    }
                                }
                                _ => panic!(),
                            };
                            if pass {
                                cur = rule.3;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut work = HashMap::new();
    let mut mm = HashMap::from([("R", 0), ("A", 1), ("in", 2)]);
    let mut ans = 0;
    let mut idx = 3;
    for line in input.lines() {
        if !line.trim().is_empty() && !line.starts_with('{') {
            let p = line.find('{').unwrap();
            let suf = &line[p + 1..line.len() - 1];
            let pref = &line[0..p];
            let mut rules = vec![];
            for r in suf.split(',') {
                let c = r.chars().next().unwrap();

                if r.find(':').is_none() {
                    if !mm.contains_key(r) {
                        mm.insert(r, idx);
                        idx += 1;
                    }
                    rules.push((c, -1, false, mm[r]));
                } else {
                    let pp = r.find(':').unwrap();
                    let rr = &r[pp + 1..r.len()];
                    if !mm.contains_key(rr) {
                        mm.insert(rr, idx);
                        idx += 1;
                    }
                    rules.push((c, r[2..pp].parse::<i32>().unwrap(), r[1..2] == *">", mm[rr]));
                }
            }
            if !mm.contains_key(pref) {
                mm.insert(pref, idx);
                idx += 1;
            }
            work.insert(mm[pref], rules);
        }
    }
    let mut que = vec![];
    let x0: (u32, u32) = (1, 4000);
    let (m0, a0, s0) = (x0, x0, x0);
    que.push(((x0, m0, a0, s0), 2));
    let mut front = 0;
    while front < que.len() {
        let ((mut x, mut m, mut a, mut s), c) = que[front];
        front += 1;
        let mut done = false;
        if c == 0 {
            continue;
        } else if c == 1 {
            ans += (x.1 - x.0 + 1) as u64
                * (m.1 - m.0 + 1) as u64
                * (a.1 - a.0 + 1) as u64
                * (s.1 - s.0 + 1) as u64;
            continue;
        }
        for rule in &work[&c] {
            if done {
                break;
            }
            let (ch, n, greater, nc) = *rule;
            if n == -1 {
                match nc {
                    1 => {
                        ans += (x.1 - x.0 + 1) as u64
                            * (m.1 - m.0 + 1) as u64
                            * (a.1 - a.0 + 1) as u64
                            * (s.1 - s.0 + 1) as u64
                    }
                    0 => {}
                    _ => que.push(((x, m, a, s), nc)),
                };
            } else {
                match ch {
                    'x' => {
                        if greater {
                            if x.0 > n as u32 {
                                que.push(((x, m, a, s), nc));
                                done = true;
                            } else if x.1 <= n as u32 {
                            } else {
                                que.push((((n as u32 + 1, x.1), m, a, s), nc));
                                x.1 = n as u32;
                            }
                        } else if x.0 >= n as u32 {
                        } else if x.1 < n as u32 {
                            que.push(((x, m, a, s), nc));
                            done = true;
                        } else {
                            que.push((((x.0, n as u32 - 1), m, a, s), nc));
                            x.0 = n as u32;
                        }
                    }
                    'm' => {
                        if greater {
                            if m.0 > n as u32 {
                                que.push(((x, m, a, s), nc));
                                done = true;
                            } else if m.1 <= n as u32 {
                            } else {
                                que.push(((x, (n as u32 + 1, m.1), a, s), nc));
                                m.1 = n as u32;
                            }
                        } else if m.0 >= n as u32 {
                        } else if m.1 < n as u32 {
                            que.push(((x, m, a, s), nc));
                            done = true;
                        } else {
                            que.push(((x, (m.0, n as u32 - 1), a, s), nc));
                            m.0 = n as u32;
                        }
                    }
                    'a' => {
                        if greater {
                            if a.0 > n as u32 {
                                que.push(((x, m, a, s), nc));
                                done = true;
                            } else if a.1 <= n as u32 {
                            } else {
                                que.push(((x, m, (n as u32 + 1, a.1), s), nc));
                                a.1 = n as u32;
                            }
                        } else if a.0 >= n as u32 {
                        } else if a.1 < n as u32 {
                            que.push(((x, m, a, s), nc));
                            done = true;
                        } else {
                            que.push(((x, m, (a.0, n as u32 - 1), s), nc));
                            a.0 = n as u32;
                        }
                    }
                    's' => {
                        if greater {
                            if s.0 > n as u32 {
                                que.push(((x, m, a, s), nc));
                                done = true;
                            } else if s.1 <= n as u32 {
                            } else {
                                que.push(((x, m, a, (n as u32 + 1, s.1)), nc));
                                s.1 = n as u32;
                            }
                        } else if s.0 >= n as u32 {
                        } else if s.1 < n as u32 {
                            que.push(((x, m, a, s), nc));
                            done = true;
                        } else {
                            que.push(((x, m, a, (s.0, n as u32 - 1)), nc));
                            s.0 = n as u32;
                        }
                    }
                    _ => panic!(),
                };
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
