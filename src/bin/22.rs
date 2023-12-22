advent_of_code::solution!(22);
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks = vec![];
    let (mut mx, mut my, mut mz) = (0, 0, 0);
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (a, b) = line.trim().split_once('~').unwrap();
        let (x, y, z) = a
            .split(',')
            .take(3)
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        let (p, q, r) = b
            .split(',')
            .take(3)
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        if x > p || y > q || z > r {
            bricks.push(((r, p, q), (z, x, y)));
        } else {
            bricks.push(((z, x, y), (r, p, q)));
        }

        mx = mx.max(x);
        mx = mx.max(p);
        my = my.max(y);
        my = my.max(q);
        mz = mz.max(z);
        mz = mz.max(r);
    }
    let mut world = vec![vec![vec![0; mz as usize + 1]; my as usize + 1]; mx as usize + 1];
    let mut needed = HashSet::new();
    bricks.sort();
    for (i, (b1, b2)) in bricks.iter().enumerate() {
        let (z1, x1, y1) = *b1;
        let (z2, x2, y2) = *b2;
        assert!(z1 <= z2 && x1 <= x2 && y1 <= y2);
        let mut z = z1;
        let mut fits = true;
        while z >= 1 && fits {
            z -= 1;
            for x in x1..x2 + 1 {
                for y in y1..y2 + 1 {
                    if world[x as usize][y as usize][z as usize] > 0 {
                        fits = false;
                        break;
                    }
                }
                if !fits {
                    break;
                }
            }
        }
        z += 1;
        if z1 != z2 {
            for z0 in z1..z2 + 1 {
                world[x1 as usize][y1 as usize][(z0 - (z1 - z)) as usize] = i + 1
            }
        } else if x1 != x2 {
            for x in x1..x2 + 1 {
                world[x as usize][y1 as usize][z as usize] = i + 1
            }
        } else {
            for y in y1..y2 + 1 {
                world[x1 as usize][y as usize][z as usize] = i + 1
            }
        }
        // println!("{}", i + 1);
        // println!("{} {} {} -> {} {} {}", x1, y1, z1, x2, y2, z2);
        // println!("{} {} {} -> {} {} {}", x1, y1, z, x2, y2, z2 - (z1 - z));
        if z > 1 {
            let mut rest = HashSet::new();
            for x in x1..x2 + 1 {
                for y in y1..y2 + 1 {
                    let cur = world[x as usize][y as usize][(z - 1) as usize];
                    if cur > 0 {
                        rest.insert(cur);
                    }
                }
            }
            // println!("{:?}", rest);
            // assert!(!rest.is_empty());
            if rest.len() == 1 {
                needed.insert(*rest.iter().next().unwrap());
            }
        }
    }
    //println!("{:?}", needed);
    let ans = bricks.len() - needed.len();
    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bricks = vec![];
    let (mut mx, mut my, mut mz) = (0, 0, 0);
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (a, b) = line.trim().split_once('~').unwrap();
        let (x, y, z) = a
            .split(',')
            .take(3)
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        let (p, q, r) = b
            .split(',')
            .take(3)
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        if x > p || y > q || z > r {
            bricks.push(((r, p, q), (z, x, y)));
        } else {
            bricks.push(((z, x, y), (r, p, q)));
        }

        mx = mx.max(x);
        mx = mx.max(p);
        my = my.max(y);
        my = my.max(q);
        mz = mz.max(z);
        mz = mz.max(r);
    }
    let mut world = vec![vec![vec![0; mz as usize + 1]; my as usize + 1]; mx as usize + 1];
    bricks.sort();
    let n = bricks.len();
    let mut graph = vec![vec![]; n];
    let mut degr = vec![0; n];
    for (i, (b1, b2)) in bricks.iter().enumerate() {
        let (z1, x1, y1) = *b1;
        let (z2, x2, y2) = *b2;
        assert!(z1 <= z2 && x1 <= x2 && y1 <= y2);
        let mut z = z1;
        let mut fits = true;
        while z >= 1 && fits {
            z -= 1;
            for x in x1..x2 + 1 {
                for y in y1..y2 + 1 {
                    if world[x as usize][y as usize][z as usize] > 0 {
                        fits = false;
                        break;
                    }
                }
                if !fits {
                    break;
                }
            }
        }
        z += 1;
        if z1 != z2 {
            for z0 in z1..z2 + 1 {
                world[x1 as usize][y1 as usize][(z0 - (z1 - z)) as usize] = i + 1
            }
        } else if x1 != x2 {
            for x in x1..x2 + 1 {
                world[x as usize][y1 as usize][z as usize] = i + 1
            }
        } else {
            for y in y1..y2 + 1 {
                world[x1 as usize][y as usize][z as usize] = i + 1
            }
        }
        if z > 1 {
            let mut rest = HashSet::new();
            for x in x1..x2 + 1 {
                for y in y1..y2 + 1 {
                    let cur = world[x as usize][y as usize][(z - 1) as usize];
                    if cur > 0 {
                        rest.insert(cur - 1);
                    }
                }
            }

            for r in rest {
                graph[r].push(i);
                degr[i] += 1;
            }
        } else {
            degr[i] = 1;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        let mut degri = degr.clone();
        let mut cur = 0;
        degri[i] = 0;
        for j in i..n {
            assert!(degri[j] >= 0);
            if degri[j] == 0 {
                cur += 1;
                for &k in &graph[j] {
                    degri[k] -= 1;
                }
            }
        }
        //println!("{} {}", i, cur);
        ans += cur - 1;
    }
    //println!("{:?}", needed);

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
