use ring_algorithm::chinese_remainder_theorem;
use std::collections::HashMap;
advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut mm = HashMap::new();
    let mut rules = HashMap::new();
    let mut idx = 0;
    let mut start = 0;
    let mut state = HashMap::new();
    for line in input.lines() {
        if !line.trim().is_empty() {
            let (mut pref, mut suf) = line.split_once("->").unwrap();
            pref = pref.trim();
            suf = suf.trim();
            let mut typ: u8 = 0;
            if pref.starts_with('&') {
                pref = &pref[1..];
                typ = 1;
            } else if pref.starts_with('%') {
                pref = &pref[1..];
                typ = 2;
            }
            if !mm.contains_key(pref) {
                mm.insert(pref, idx);
                if typ == 0 {
                    start = idx
                }
                idx += 1;
            }
            let mut rule = vec![];
            for r in suf.split(',').map(|x| x.trim()) {
                if !mm.contains_key(r) {
                    mm.insert(r, idx);
                    idx += 1;
                }
                rule.push(mm[r]);
            }

            rules.insert(mm[pref], (typ, rule));
            if typ == 2 {
                state.insert(mm[pref], HashMap::from([(mm[pref], 0)]));
            } else if typ == 1 {
                state.insert(mm[pref], HashMap::new());
            }
        }
    }
    for rule in &rules {
        for r in rule.1 .1.clone() {
            if rules.contains_key(&r) && rules[&r].0 == 1 {
                state.get_mut(&r).unwrap().insert(*rule.0, 0);
            }
        }
    }
    let (mut ans0, mut ans1) = (0, 0);
    for _ in 0..1000 {
        let mut que = vec![(start, start, 0)];

        let mut front = 0;
        while front < que.len() {
            let (cur, bcur, pulse) = que[front];
            front += 1;
            if pulse == 0 {
                ans0 += 1
            } else {
                ans1 += 1
            }

            if rules.contains_key(&cur) {
                let (typ, rule) = rules[&cur].clone();
                let npulse = {
                    if typ == 0 {
                        pulse
                    } else if typ == 2 {
                        if pulse == 0 {
                            if state[&cur][&cur] == 0 {
                                state.insert(cur, HashMap::from([(cur, 1)]));
                                1
                            } else {
                                state.insert(cur, HashMap::from([(cur, 0)]));
                                0
                            }
                        } else {
                            2
                        }
                    } else if typ == 1 {
                        let mut st = state[&cur].clone();
                        //assert!(st.contains_key(&bcur));
                        st.insert(bcur, pulse);
                        let mut pos = true;
                        for b in st.values() {
                            if *b == 0 {
                                pos = false;
                                break;
                            }
                        }
                        state.insert(cur, st);
                        if pos {
                            0
                        } else {
                            1
                        }
                    } else {
                        panic!();
                    }
                };
                if npulse != 2 {
                    for nxt in rule {
                        que.push((nxt, cur, npulse));
                    }
                }
            }
        }
    }
    let ans = ans0 * ans1;
    Some(ans)
}

// this is indeed a very guessy solution
pub fn part_two(input: &str) -> Option<u64> {
    let mut mm = HashMap::new();
    let mut rules = HashMap::new();
    let mut idx = 0;
    let (mut start, mut finish) = (0, 0);
    let mut state = HashMap::new();
    for line in input.lines() {
        if !line.trim().is_empty() {
            let (mut pref, mut suf) = line.split_once("->").unwrap();
            pref = pref.trim();
            suf = suf.trim();
            let mut typ: u8 = 0;
            if pref.starts_with('&') {
                pref = &pref[1..];
                typ = 1;
            } else if pref.starts_with('%') {
                pref = &pref[1..];
                typ = 2;
            }
            if !mm.contains_key(pref) {
                mm.insert(pref, idx);
                if typ == 0 {
                    start = idx
                }
                idx += 1;
            }
            let mut rule = vec![];
            for r in suf.split(',').map(|x| x.trim()) {
                if !mm.contains_key(r) {
                    mm.insert(r, idx);
                    if r == "rx" {
                        finish = idx;
                    }
                    idx += 1;
                }
                rule.push(mm[r]);
            }

            rules.insert(mm[pref], (typ, rule));
            if typ == 2 {
                state.insert(mm[pref], HashMap::from([(mm[pref], 0)]));
            } else if typ == 1 {
                state.insert(mm[pref], HashMap::new());
            }
        }
    }
    for rule in &rules {
        for r in rule.1 .1.clone() {
            if rules.contains_key(&r) && rules[&r].0 == 1 {
                state.get_mut(&r).unwrap().insert(*rule.0, 0);
            }
        }
    }
    let mut done = false;
    let mut target = HashMap::new();
    for (i, ch) in ["vz", "bq", "qh", "lt"].iter().enumerate() {
        target.insert(mm[ch], i);
    }
    let mut cc = vec![vec![]; 4];

    for i in 0..8500 {
        if done {
            break;
        }
        let mut que = vec![(start, start, 0)];
        let mut front = 0;
        let mut good = [false; 4];
        while front < que.len() {
            let (cur, bcur, pulse) = que[front];
            front += 1;
            if pulse == 1 && mm["ft"] == cur {
                good[target[&bcur]] = true;
            }

            if rules.contains_key(&cur) {
                let (typ, rule) = rules[&cur].clone();
                let npulse = {
                    if typ == 0 {
                        pulse
                    } else if typ == 2 {
                        if pulse == 0 {
                            if state[&cur][&cur] == 0 {
                                state.insert(cur, HashMap::from([(cur, 1)]));
                                1
                            } else {
                                state.insert(cur, HashMap::from([(cur, 0)]));
                                0
                            }
                        } else {
                            2
                        }
                    } else if typ == 1 {
                        let mut st = state[&cur].clone();
                        //assert!(st.contains_key(&bcur));
                        st.insert(bcur, pulse);
                        let mut pos = true;
                        for b in st.values() {
                            if *b == 0 {
                                pos = false;
                                break;
                            }
                        }
                        state.insert(cur, st);
                        if pos {
                            0
                        } else {
                            1
                        }
                    } else {
                        panic!();
                    }
                };
                if npulse != 2 {
                    for nxt in rule {
                        if nxt == finish && npulse == 0 {
                            done = true;
                            break;
                        }
                        que.push((nxt, cur, npulse));
                    }
                }
            }
        }
        for j in 0..4 {
            if good[j] {
                cc[j].push(i);
            }
        }
    }
    let (mut u, mut m) = (vec![], vec![]);
    for cci in cc {
        let d = cci[1] - cci[0];
        u.push(cci[0] % d);
        m.push(d);
    }
    let ans = chinese_remainder_theorem::<i64>(&u, &m).unwrap() + 1;
    Some(ans as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {}
}
