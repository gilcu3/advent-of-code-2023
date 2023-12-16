advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut ans = 0;
    let mut mat = vec![];
    for line in input.lines() {
        if !line.trim().is_empty() {
            mat.push(
                line.chars()
                    .map(|c| match c {
                        '/' => 0,
                        '\\' => 1,
                        '|' => 2,
                        '-' => 3,
                        '.' => 4,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }
    let (n, m) = (mat.len(), mat[0].len());
    let mut vis = vec![vec![vec![false; 4]; m]; n];
    let mut seen = vec![vec![false; m]; n];
    let mut que = vec![];
    que.push((0, 0, 1));
    vis[0][0][1] = true;
    seen[0][0] = true;
    ans += 1;
    let mut front = 0;
    while front < que.len() {
        let (x, y, d) = que[front];
        front += 1;
        //println!("{} {} {}", x, y, d);
        let dd = match mat[x][y] {
            0 => vec![(3, 0), (0, 3), (1, 2), (2, 1)],
            1 => vec![(0, 1), (1, 0), (2, 3), (3, 2)],
            2 => vec![(2, 2), (0, 0), (1, 0), (1, 2), (3, 0), (3, 2)],
            3 => vec![(1, 1), (3, 3), (0, 1), (0, 3), (2, 1), (2, 3)],
            4 => vec![(0, 0), (2, 2), (1, 1), (3, 3)],
            _ => panic!(),
        };
        for (d0, nd) in dd {
            if d == d0 {
                let (nx, ny) = (x as i32 + dirs[nd].0, y as i32 + dirs[nd].1);
                if nx >= 0
                    && nx < n as i32
                    && ny >= 0
                    && ny < m as i32
                    && !vis[nx as usize][ny as usize][nd]
                {
                    vis[nx as usize][ny as usize][nd] = true;

                    que.push((nx as usize, ny as usize, nd));
                    if !seen[nx as usize][ny as usize] {
                        ans += 1;
                        seen[nx as usize][ny as usize] = true;
                    }
                }
            }
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut ans = 0;
    let mut mat = vec![];
    for line in input.lines() {
        if !line.trim().is_empty() {
            mat.push(
                line.chars()
                    .map(|c| match c {
                        '/' => 0,
                        '\\' => 1,
                        '|' => 2,
                        '-' => 3,
                        '.' => 4,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }
    let (n, m) = (mat.len(), mat[0].len());
    let mut tried = vec![vec![vec![false; 4]; m]; n];
    let mut vis = vec![vec![vec![2 * (n + m); 4]; m]; n];
    let mut seen = vec![vec![2 * (n + m); m]; n];
    for i in 0..2 * (n + m) {
        let mut cur = 0;

        let mut que = vec![];

        if i < n {
            que.push((i, 0, 1));
            vis[i][0][1] = i;
            seen[i][0] = i;
            cur += 1;
        } else if i < 2 * n {
            que.push((i - n, m - 1, 3));
            vis[i - n][m - 1][3] = i;
            seen[i - n][m - 1] = i;
            cur += 1;
        } else if i < 2 * n + m {
            que.push((0, i - 2 * n, 0));
            vis[0][i - 2 * n][0] = i;
            seen[0][i - 2 * n] = i;
            cur += 1;
        } else {
            que.push((0, i - 2 * n - m, 2));
            vis[0][i - 2 * n - m][2] = i;
            seen[0][i - 2 * n - m] = i;
            cur += 1;
        }

        if tried[que[0].0][que[0].1][(que[0].2 + 2) % 4] {
            continue;
        }

        let mut front = 0;
        while front < que.len() {
            let (x, y, d) = que[front];
            front += 1;

            //println!("{} {} {}", x, y, d);
            let dd = match mat[x][y] {
                0 => vec![(3, 0), (0, 3), (1, 2), (2, 1)],
                1 => vec![(0, 1), (1, 0), (2, 3), (3, 2)],
                2 => vec![(2, 2), (0, 0), (1, 0), (1, 2), (3, 0), (3, 2)],
                3 => vec![(1, 1), (3, 3), (0, 1), (0, 3), (2, 1), (2, 3)],
                4 => vec![(0, 0), (2, 2), (1, 1), (3, 3)],
                _ => panic!(),
            };
            if x == 0 || x == n - 1 || y == 0 || y == m - 1 {
                tried[x][y][d] = true;
            }
            for (d0, nd) in dd {
                if d == d0 {
                    if x == 0 || x == n - 1 || y == 0 || y == m - 1 {
                        tried[x][y][nd] = true;
                    }
                    let (nx, ny) = (x as i32 + dirs[nd].0, y as i32 + dirs[nd].1);
                    if nx >= 0
                        && nx < n as i32
                        && ny >= 0
                        && ny < m as i32
                        && i != vis[nx as usize][ny as usize][nd]
                    {
                        vis[nx as usize][ny as usize][nd] = i;

                        que.push((nx as usize, ny as usize, nd));
                        if i != seen[nx as usize][ny as usize] {
                            cur += 1;
                            seen[nx as usize][ny as usize] = i;
                        }
                    }
                }
            }
        }
        ans = std::cmp::max(ans, cur);
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
