advent_of_code::solution!(23);
use std::collections::{HashMap, HashSet};
fn dfs(graph: &Vec<Vec<usize>>, cv: usize, ev: usize, seen: &mut Vec<bool>) -> isize {
    let mut ans = -1;
    seen[cv] = true;
    if cv != ev {
        for &nv in &graph[cv] {
            if !seen[nv] {
                ans = ans.max(dfs(graph, nv, ev, seen));
            }
        }
    } else {
        ans = 0;
    }
    seen[cv] = false;
    if ans != -1 {
        ans + 1
    } else {
        -1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut mat = vec![];
    let mut matid = vec![];
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut idx = 1;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut row = vec![];
        let mut rowid = vec![];
        for c in line.chars() {
            match c {
                '#' => row.push(vec![]),
                '.' => row.push(vec![0, 1, 2, 3]),
                '>' => row.push(vec![0]),
                '^' => row.push(vec![3]),
                '<' => row.push(vec![2]),
                'v' => row.push(vec![1]),
                _ => panic!("Invalid input"),
            }
            if c != '#' {
                rowid.push(idx);
                idx += 1;
            } else {
                rowid.push(0);
            }
        }
        mat.push(row);
        matid.push(rowid)
    }
    let n = mat.len();
    let mut graph = vec![vec![]; idx];

    for i in 0..n {
        for j in 0..n {
            for d in &mat[i][j] {
                let (dx, dy) = dirs[*d as usize];
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x >= 0
                    && y >= 0
                    && x < n as i32
                    && y < n as i32
                    && !mat[x as usize][y as usize].is_empty()
                {
                    graph[matid[i][j]].push(matid[x as usize][y as usize]);
                }
            }
        }
    }
    //println!("{} {:?}", idx, graph);
    let ans = dfs(&graph, 1, idx - 1, &mut vec![false; idx]) - 1;
    Some(ans as u32)
}

fn dfs2(graph: &Vec<Vec<(usize, usize)>>, cv: usize, ev: usize, seen: &mut Vec<bool>) -> isize {
    let mut ans = -1;
    seen[cv] = true;
    if cv != ev {
        for &(nv, nw) in &graph[cv] {
            if !seen[nv] {
                let cur = dfs2(graph, nv, ev, seen);
                if cur != -1 {
                    if ans == -1 {
                        ans = cur + nw as isize;
                    } else {
                        ans = ans.max(cur + nw as isize);
                    }
                }
            }
        }
    } else {
        ans = 0
    }
    seen[cv] = false;
    ans
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mat = vec![];
    let mut matid = vec![];
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut idx = 1;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut row = vec![];
        let mut rowid = vec![];
        for c in line.chars() {
            match c {
                '#' => row.push(vec![]),
                _ => row.push(vec![0, 1, 2, 3]),
            }
            if c != '#' {
                rowid.push(idx);
                idx += 1;
            } else {
                rowid.push(0);
            }
        }
        mat.push(row);
        matid.push(rowid)
    }
    let n = mat.len();
    let mut graph = vec![vec![]; idx];

    for i in 0..n {
        for j in 0..n {
            for d in &mat[i][j] {
                let (dx, dy) = dirs[*d as usize];
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x >= 0
                    && y >= 0
                    && x < n as i32
                    && y < n as i32
                    && !mat[x as usize][y as usize].is_empty()
                {
                    graph[matid[i][j]].push(matid[x as usize][y as usize]);
                }
            }
        }
    }
    //compress graph
    let mut idx2 = 0;
    let mut edges = HashSet::new();
    let mut hh: HashMap<usize, usize> = HashMap::new();
    for i in 0..idx {
        if !graph[i].is_empty() && graph[i].len() != 2 {
            let ii = if let std::collections::hash_map::Entry::Vacant(e) = hh.entry(i) {
                e.insert(idx2);
                idx2 += 1;
                idx2 - 1
            } else {
                let tmp = *hh.get(&i).unwrap();
                tmp
            };
            for &j in &graph[i] {
                if graph[j].len() == 2 {
                    let mut jj = j;
                    let mut pj = i;
                    let mut v = 0;
                    while graph[jj].len() == 2 {
                        v += 1;
                        jj = {
                            if graph[jj][0] == pj {
                                pj = jj;
                                graph[jj][1]
                            } else {
                                pj = jj;
                                graph[jj][0]
                            }
                        };
                    }
                    let jjj = if let std::collections::hash_map::Entry::Vacant(e) = hh.entry(jj) {
                        e.insert(idx2);
                        idx2 += 1;
                        idx2 - 1
                    } else {
                        let tmp = *hh.get(&jj).unwrap();
                        tmp
                    };
                    edges.insert((ii, jjj, v + 1));
                } else {
                    let jj = if let std::collections::hash_map::Entry::Vacant(e) = hh.entry(j) {
                        e.insert(idx2);
                        idx2 += 1;
                        idx2 - 1
                    } else {
                        let tmp = *hh.get(&j).unwrap();
                        tmp
                    };
                    edges.insert((ii, jj, 1));
                }
            }
        }
    }

    let mut graph2 = vec![vec![]; idx2];
    for (i, j, v) in edges {
        graph2[i].push((j, v));
    }
    //println!("{:?}", graph2);

    let ans = dfs2(&graph2, hh[&1], hh[&(idx - 1)], &mut vec![false; idx2]);
    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
