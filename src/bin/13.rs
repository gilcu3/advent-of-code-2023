advent_of_code::solution!(13);

fn solve(pat: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let (n, m) = (pat.len(), pat[0].len());
    let (mut si, mut sj) = (vec![], vec![]);
    for i in 0..n - 1 {
        let mut pos = true;
        for j in 0..m {
            for i1 in 0..i + 1 {
                let i2 = i + 1 + i - i1;
                if i2 < n && pat[i1].chars().nth(j).unwrap() != pat[i2].chars().nth(j).unwrap() {
                    pos = false;
                    break;
                }
            }
            if !pos {
                break;
            }
        }
        if pos {
            si.push(i + 1);
        }
    }
    for j in 0..m - 1 {
        let mut pos = true;
        for pati in pat {
            for j1 in 0..j + 1 {
                let j2 = j + 1 + j - j1;
                if j2 < m && pati.chars().nth(j1).unwrap() != pati.chars().nth(j2).unwrap() {
                    pos = false;
                    break;
                }
            }
            if !pos {
                break;
            }
        }
        if pos {
            sj.push(j + 1);
        }
    }
    (si, sj)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pat = Vec::new();
    let mut ans = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            let cur = solve(&pat);
            if !cur.0.is_empty() {
                ans += 100 * cur.0[0] as u32;
            } else if !cur.1.is_empty() {
                ans += cur.1[0] as u32;
            }
            pat.clear();
            continue;
        }
        pat.push(line.trim().to_string());
    }

    if !pat.is_empty() {
        let cur = solve(&pat);
        if !cur.0.is_empty() {
            ans += 100 * cur.0[0] as u32;
        } else if !cur.1.is_empty() {
            ans += cur.1[0] as u32;
        }
    }
    Some(ans)
}

fn replace_char_at(s: String, idx: usize) -> String {
    let mut r = String::with_capacity(s.len());
    for (i, d) in s.char_indices() {
        r.push(if i == idx {
            if d == '#' {
                '.'
            } else {
                '#'
            }
        } else {
            d
        });
    }
    r
}

fn solve2(pat: &Vec<String>) -> (usize, usize) {
    let (oi, oj) = solve(pat);
    let (mut si, mut sj) = (0, 0);
    let (n, m) = (pat.len(), pat[0].len());
    let mut found = false;
    for i in 0..n {
        for j in 0..m {
            let mut npat = vec![];
            pat.iter().for_each(|s| npat.push(s.clone()));
            npat[i] = replace_char_at(npat[i].clone(), j);
            let (ni, nj) = solve(&npat);
            for ii in ni.iter() {
                if oi.is_empty() {
                    si = *ii;
                    sj = 0;
                    found = true;
                    break;
                } else {
                    for oo in oi.iter() {
                        if *ii != *oo {
                            si = *ii;
                            sj = 0;
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
            for jj in nj.iter() {
                if oj.is_empty() {
                    sj = *jj;
                    si = 0;
                    found = true;
                    break;
                } else {
                    for oo in oj.iter() {
                        if *jj != *oo {
                            sj = *jj;
                            si = 0;
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
        }
        if found {
            break;
        }
    }
    assert!(found);
    (si, sj)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pat = Vec::new();
    let mut ans = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            let cur = solve2(&pat);
            if cur.0 != 0 {
                ans += 100 * cur.0 as u32;
            } else if cur.1 != 0 {
                ans += cur.1 as u32;
            }
            pat.clear();
            continue;
        }
        pat.push(line.trim().to_string());
    }

    if !pat.is_empty() {
        let cur = solve2(&pat);
        if cur.0 != 0 {
            ans += 100 * cur.0 as u32;
        } else if cur.1 != 0 {
            ans += cur.1 as u32;
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
