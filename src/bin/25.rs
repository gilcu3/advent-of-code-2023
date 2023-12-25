advent_of_code::solution!(25);

use std::collections::BinaryHeap;
use std::collections::HashMap;

fn mincut(g: &mut Vec<Vec<u32>>) -> (u32, Vec<usize>) {
    let n = g.len();
    let mut best_cost = u32::MAX;
    let mut best_cut: Vec<usize> = Vec::new();
    let mut v: Vec<Vec<usize>> = vec![Vec::with_capacity(1); n];

    for (i, vi) in v.iter_mut().enumerate() {
        vi.push(i);
    }

    let mut w: Vec<u32> = vec![0; n];
    let mut exist: Vec<bool> = vec![true; n];
    let mut in_a: Vec<bool> = vec![false; n];
    let mut gg = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if g[i][j] == 1 {
                gg[i].push(j);
                gg[j].push(i);
            }
        }
    }

    for ph in 0..n - 1 {
        in_a.fill(false);
        w.fill(0);
        let mut prev = 0;
        let mut que = BinaryHeap::new();

        for i in 0..n {
            if exist[i] {
                que.push((w[i], i));
            }
        }

        for it in 0..n - ph {
            let sel;
            loop {
                let (ww, ii) = que.pop().unwrap();
                if exist[ii] && !in_a[ii] && ww == w[ii] {
                    sel = ii;
                    break;
                }
            }

            if it == n - ph - 1 {
                if w[sel] < best_cost {
                    best_cost = w[sel];
                    best_cut = v[sel].clone();
                }

                for i in 0..n {
                    if exist[i] {
                        if g[prev][i] == 0 && g[sel][i] > 0 {
                            gg[prev].push(i);
                            gg[i].push(prev);
                        }
                        g[prev][i] += g[sel][i];
                        g[i][prev] = g[prev][i];
                    }
                }
                let mut a = v[sel].clone();
                v[prev].append(&mut a);
                exist[sel] = false;
            } else {
                in_a[sel] = true;
                for i in gg[sel].iter() {
                    if g[sel][*i] > 0 && exist[*i] && !in_a[*i] {
                        w[*i] += g[sel][*i];
                        que.push((w[*i], *i));
                    }
                }
                prev = sel;
            }
        }
    }

    (best_cost, best_cut)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hh = HashMap::new();
    let mut edges = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (pref, suf) = line.split_once(": ").unwrap();
        if !hh.contains_key(pref) {
            hh.insert(pref, hh.len());
        }
        for v in suf.split(' ') {
            if !hh.contains_key(v) {
                hh.insert(v, hh.len());
            }
            edges.push((hh[pref], hh[v]));
        }
    }
    let n = hh.len();
    let mut g = vec![vec![0; n]; n];
    for (u, v) in edges {
        g[u][v] = 1;
        g[v][u] = 1;
    }
    let (_cost, cut) = mincut(&mut g);
    // println!("{:?}", hh);
    // println!("n: {}, m: {}", n, m);
    // println!("cost: {}", cost);
    // println!("cut: {:?}", cut);
    let ans = cut.len() * (n - cut.len());
    Some(ans as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
