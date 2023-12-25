use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let steps = 64;
    let mut mat = vec![];
    let (mut sx, mut sy) = (0, 0);
    for line in input.lines() {
        if !line.trim().is_empty() {
            let mut row = vec![];
            line.chars()
                .for_each(|c| row.push(if c == '#' { 1 } else { 0 }));
            let ss = line.find('S');
            if ss.is_some() {
                sx = mat.len();
                sy = ss.unwrap();
            }
            mat.push(row);
        }
    }
    //println!("mat: {:?}", mat);
    //println!("sx: {}, sy: {}", sx, sy);
    let mut ans = 0;
    let (n, m) = (mat.len(), mat[0].len());
    let mut que = vec![];
    que.push((sx, sy, 0));
    mat[sx][sy] = 2;
    let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut front = 0;
    while front < que.len() {
        let (x, y, d) = que[front];
        front += 1;
        if d % 2 == steps % 2 {
            ans += 1;
        }
        if d == steps {
            continue;
        }
        for (dx, dy) in dirs.iter() {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0
                && nx < n as i32
                && ny >= 0
                && ny < m as i32
                && mat[nx as usize][ny as usize] == 0
            {
                mat[nx as usize][ny as usize] = 2;
                que.push((nx as usize, ny as usize, d + 1));
            }
        }
    }
    Some(ans)
}

fn compute_distance(sv: Vec<(usize, usize)>, dsv: Vec<u32>, mat: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let n = mat.len();
    let mut dis = vec![vec![std::u32::MAX; n]; n];
    let mut que = vec![];
    for (i, (sx, sy)) in sv.iter().enumerate() {
        dis[*sx][*sy] = dsv[i];
        que.push((*sx, *sy));
    }

    let mut front = 0;
    while front < que.len() {
        let (x, y) = que[front];
        front += 1;
        for (dx, dy) in [(1, 0), (0, -1), (-1, 0), (0, 1)].iter() {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0
                && nx < n as i32
                && ny >= 0
                && ny < n as i32
                && mat[nx as usize][ny as usize] == 0
                && dis[nx as usize][ny as usize] > dis[x][y] + 1
            {
                dis[nx as usize][ny as usize] = dis[x][y] + 1;
                que.push((nx as usize, ny as usize));
            }
        }
    }
    dis
}

pub fn brute(
    sx: usize,
    sy: usize,
    mat: &Vec<Vec<u32>>,
    steps: u32,
) -> (HashMap<(i32, i32), u64>, u64) {
    let n = mat.len();
    let mut dis = HashMap::new();
    let mut que = vec![];
    let mut ans = 0;
    dis.insert((sx as i32, sy as i32), 0);
    que.push((sx as i32, sy as i32));
    let mut sq = HashMap::new();

    let mut front = 0;
    while front < que.len() {
        let (x, y) = que[front];
        front += 1;
        if dis[&(x, y)] % 2 == steps % 2 {
            let (xx, yy) = (
                if x >= 0 {
                    x / n as i32
                } else {
                    -((-x - 1) / n as i32) - 1
                },
                if y >= 0 {
                    y / n as i32
                } else {
                    -((-y - 1) / n as i32) - 1
                },
            );
            sq.insert((xx, yy), sq.get(&(xx, yy)).unwrap_or(&0) + 1);
            ans += 1;
        }
        if dis[&(x, y)] == steps {
            continue;
        }
        for (dx, dy) in [(1, 0), (0, -1), (-1, 0), (0, 1)].iter() {
            let (nx, ny) = (x + dx, y + dy);
            let mn = n as i32;
            let (kx, ky) = ((nx % mn + mn) % mn, (ny % mn + mn) % mn);
            if mat[kx as usize][ky as usize] == 0 && !dis.contains_key(&(nx, ny)) {
                dis.insert((nx, ny), dis[&(x, y)] + 1);
                que.push((nx, ny));
            }
        }
    }
    //println!("sq: {:?}", sq);
    (sq, ans)
}

pub fn solve_part_two(input: &str, steps: u32) -> Option<u64> {
    let mut mat = vec![];
    let (mut sx, mut sy) = (0, 0);
    for line in input.lines() {
        if !line.trim().is_empty() {
            let mut row = vec![];
            line.chars()
                .for_each(|c| row.push(if c == '#' { 1 } else { 0 }));
            let ss = line.find('S');
            if ss.is_some() {
                sx = mat.len();
                sy = ss.unwrap();
            }
            mat.push(row);
        }
    }
    //let (rsq, rans) = brute(sx, sy, &mat, steps);

    let mut ans: u64 = 0;
    let n = mat.len(); // square size
    let dcenter = compute_distance(vec![(sx, sy)], vec![0], &mat);
    let mut ov = [0, 0];
    //let mut sq = HashMap::new();
    for (i, dci) in dcenter.iter().enumerate() {
        //println!("{:?}", dcenter[i]);
        for j in 0..n {
            if dci[j] <= steps && (dci[j] + steps) % 2 == 0 {
                //sq.insert((0, 0), sq.get(&(0, 0)).unwrap_or(&0) + 1);
                ans += 1;
            }
            if mat[i][j] == 0 && dci[j] != std::u32::MAX {
                ov[(i + j) % 2] += 1;
            }
        }
    }
    let mut ind = vec![vec![vec![]; 2]; 2];
    for i in 0..n {
        ind[0][0].push((i, 0));
        ind[0][1].push((i, n - 1));
        ind[1][0].push((0, i));
        ind[1][1].push((n - 1, i));
    }
    // same row or column
    for d1 in 0..2 {
        // this was  part of the general solution that is not the most optimal

        //let mut dl = vec![vec![0; n]; n];
        //let mut mx = 260;
        // for (i, dli) in dl.iter_mut().enumerate() {
        //     let tmp = compute_distance(vec![if d1 == 0 { (i, 0) } else { (0, i) }], vec![0], &mat);
        //     for j in 0..n {
        //         dli[j] = if d1 == 0 {
        //             tmp[j][n - 1]
        //         } else {
        //             tmp[n - 1][j]
        //         };
        //     }
        //     // for tx in tmp.iter() {
        //     //     for ty in tx.iter() {
        //     //         if *ty != std::u32::MAX {
        //     //             mx = mx.max(*ty)
        //     //         }
        //     //     }
        //     // }
        //     // let tmp = compute_distance(
        //     //     vec![if d1 == 0 { (i, n - 1) } else { (n - 1, i) }],
        //     //     vec![0],
        //     //     &mat,
        //     // );
        //     // for tx in tmp.iter() {
        //     //     for ty in tx.iter() {
        //     //         if *ty != std::u32::MAX {
        //     //             mx = mx.max(*ty)
        //     //         }
        //     //     }
        //     // }
        // }

        //println!("mx: {}", mx);
        let top = steps / n as u32;
        for d2 in 0..2 {
            let mut d0 = vec![0; n];
            for i in 0..n {
                d0[i] = dcenter[ind[d1][1 - d2][i].0][ind[d1][1 - d2][i].1] + 1;
            }
            let tmp = compute_distance(ind[d1][d2].clone(), vec![0; n], &mat);
            let mx = *tmp
                .iter()
                .map(|x| x.iter().filter(|x| **x != std::u32::MAX).max().unwrap())
                .max()
                .unwrap();

            for s in 0..top + 1 {
                //if d1 == 0 && d2 == 0 {println!("d0: {:?}", d0)}
                // let (cx, cy) = {
                //     if d1 == 0{
                //         if d2 == 0{(0, (s + 1) as i32)} else {(0, - (s as i32) - 1)}
                //     }
                //     else{
                //         if d2 == 0{((s + 1) as i32, 0)} else {(- (s as i32) - 1, 0)}
                //     }
                // };

                let curmx = *d0.iter().max().unwrap();
                let par = (steps + (sx + sy) as u32 + ((s + 1) * n as u32)) % 2;

                if curmx + mx <= steps {
                    //sq.insert((cx, cy), sq.get(&(cx, cy)).unwrap_or(&0) + ov[par as usize] as u64);
                    ans += ov[par as usize];
                } else {
                    let tmp = compute_distance(ind[d1][d2].clone(), d0.clone(), &mat);
                    for ti in tmp {
                        for tj in ti {
                            if tj != std::u32::MAX && tj <= steps && (tj + steps) % 2 == 0 {
                                //sq.insert((cx, cy), sq.get(&(cx, cy)).unwrap_or(&0) + 1);

                                ans += 1;
                            }
                        }
                    }
                }
                // this seems to hold in general, for some constant
                if s < 3 {
                    // this was part of the general solution that is not the most optimal
                    // let mut tmp = vec![std::u32::MAX; n];

                    // for (i, ti) in tmp.iter_mut().enumerate() {
                    //     for i0 in 0..n {
                    //         *ti = (*ti).min(d0[i0] + dl[i0][i] + 1);
                    //     }
                    // }
                    // d0 = tmp;

                    let tmp = compute_distance(ind[d1][d2].clone(), d0.clone(), &mat);
                    for j in 0..n {
                        d0[j] = if d1 == 0 {
                            tmp[j][if d2 == 0 { n - 1 } else { 0 }]
                        } else {
                            tmp[if d2 == 0 { n - 1 } else { 0 }][j]
                        } + 1;
                    }
                } else {
                    for d0i in d0.iter_mut() {
                        *d0i += n as u32;
                    }
                }
            }

            // this was part of the general solution that is not the most optimal
            // if d2 == 0 {
            //     for i in 0..n {
            //         for j in 0..i {
            //             (dl[i][j], dl[j][i]) = (dl[j][i], dl[i][j]);
            //         }
            //     }
            // }
        }
    }
    for x in 0..2 {
        for y in 0..2 {
            let dl = compute_distance(vec![(x * (n - 1), y * (n - 1))], vec![0], &mat);
            let mut mx = 0;
            for dli in dl.iter() {
                for dlj in dli {
                    if *dlj != std::u32::MAX {
                        mx = mx.max(*dlj as usize)
                    }
                }
            }

            let mut cdis = [vec![0; mx + 1], vec![0; mx + 1]];
            for (i, dli) in dl.iter().enumerate() {
                for j in 0..n {
                    if dli[j] != std::u32::MAX {
                        cdis[(i + j + (x + y) * (n - 1)) % 2][dli[j] as usize] += 1;
                    }
                }
            }
            for i in 1..mx + 1 {
                cdis[0][i] += cdis[0][i - 1];
                cdis[1][i] += cdis[1][i - 1];
            }
            //println!("{:?}", cdis);

            let od = 2 + dcenter[(1 - x) * (n - 1)][(1 - y) * (n - 1)];
            let top = steps / n as u32;
            let mut j: i32 = 0;
            while od + j as u32 * n as u32 + mx as u32 <= steps {
                j += 1;
            }
            j -= 1;
            // let (cx, cy) = {
            //     if x == 0{
            //         if y == 0{(1, 1)} else {(1, -1)}
            //     }
            //     else{
            //         if y == 0{(-1, 1)} else {(-1, -1)}
            //     }
            // };
            for i in 0..top + 1 {
                j += 2;
                while j >= 0 && od + (i + j as u32) * n as u32 + mx as u32 > steps {
                    let cur = od + (i + j as u32) * n as u32;
                    if cur <= steps {
                        let left = (steps - cur) as usize;
                        //sq.insert((cx * (i + 1) as i32, cy * (j + 1)), sq.get(&(cx * (i + 1) as i32, cy * (j + 1))).unwrap_or(&0) + cdis[left % 2][left]);
                        ans += cdis[left % 2][left];
                    }

                    j -= 1;
                }

                //println!("-> {} {} {} {} {}", x, y, i, j, sq.get(&(-1, 1)).unwrap_or(&0));

                if j >= 0 {
                    let mut p0 = 0;
                    if (steps + i * n as u32 + od + ((x + y) * (n - 1)) as u32) % 2 == 0 {
                        p0 += j / 2 + 1
                    }
                    if (steps + (i + 1) * n as u32 + od + ((x + y) * (n - 1)) as u32) % 2 == 0 {
                        p0 += (j + 1) / 2
                    }

                    // for j0 in 0..j+1{
                    //     let cur = ov[((steps + (j0 as u32 + i) * n as u32 + ((x + y) * (n - 1)) as u32 + od) % 2) as usize];
                    //     sq.insert((cx * (i + 1) as i32, cy * (j0 + 1)), sq.get(&(cx * (i + 1) as i32, cy * (j0 + 1))).unwrap_or(&0) + cur);
                    // }

                    ans += ov[0] * p0 as u64 + ov[1] * (j + 1 - p0) as u64;
                }
            }
        }
    }
    // for i in -20..20{
    //     for j in -20..20{
    //         if rsq.contains_key(&(i, j)){
    //             println!("{} {}", i, j);
    //             println!("{} {} {} {}", i, j, sq[&(i, j)], rsq[&(i, j)]);
    //             assert!(sq[&(i, j)] == rsq[&(i, j)]);
    //         }
    //     }
    // }

    // //println!("{:?}", sq);
    // //println!("{:?}", rsq);
    //println!("ans: {}; rans: {}", ans, rans);
    //assert!(ans == rans);
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_part_two(input, 26501365)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let data = [
            (6, 16),
            (10, 50),
            (50, 1594),
            (100, 6536),
            (500, 167004),
            (1000, 668697),
            (5000, 16733044),
        ];
        for (steps, ans) in data.iter() {
            let result = solve_part_two(
                &advent_of_code::template::read_file("examples", DAY),
                *steps,
            );
            assert_eq!(result, Some(*ans));
        }
    }
}
