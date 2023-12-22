use std::collections::BinaryHeap;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut mat = vec![];
    for line in input.lines() {
        if !line.trim().is_empty() {
            mat.push(
                line.chars()
                    .map(|x| x.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            );
        }
    }

    let (n, m) = (mat.len(), mat[0].len());
    let mut dist = vec![vec![vec![std::u32::MAX; 2]; m]; n];
    let mut que = BinaryHeap::new();
    for (d, dirsd) in dirs.iter().enumerate().take(2) {
        let mut cur = 0;
        let (mut nx, mut ny): (isize, isize) = (0, 0);
        for _ in 0..3 {
            nx += dirsd.0;
            ny += dirsd.1;

            if 0 <= nx && nx < n as isize && 0 <= ny && ny < m as isize {
                cur += mat[nx as usize][ny as usize] as i32;
                dist[nx as usize][ny as usize][d] = cur as u32;
                que.push((-cur, (nx, ny, d)));
            }
        }
    }
    while !que.is_empty() {
        let (mut cur, (x, y, d)) = que.pop().unwrap();
        cur = -cur;
        //println!("{} {} {} {}", x, y, d, cur);
        if x == (n - 1) as isize && y == (m - 1) as isize {
            return Some(cur as u32);
        }
        if dist[x as usize][y as usize][d % 2] < cur as u32 {
            continue;
        }
        for nd in [(d + 1) % 4, (d + 3) % 4] {
            let mut ncur = 0;
            let nd2 = nd % 2;
            let (mut nx, mut ny) = (x, y);
            for _ in 0..3 {
                nx += dirs[nd].0;
                ny += dirs[nd].1;
                if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                    ncur += mat[nx as usize][ny as usize] as u32;
                    if dist[nx as usize][ny as usize][nd2] > dist[x as usize][y as usize][d] + ncur
                    {
                        dist[nx as usize][ny as usize][nd2] =
                            dist[x as usize][y as usize][d] + ncur;
                        que.push((-(dist[nx as usize][ny as usize][nd2] as i32), (nx, ny, nd2)));
                    }
                }
            }
        }
    }

    panic!("No solution found");
}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut mat = vec![];
    for line in input.lines() {
        if !line.trim().is_empty() {
            mat.push(
                line.chars()
                    .map(|x| x.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            );
        }
    }

    let (n, m) = (mat.len(), mat[0].len());
    let mut dist = vec![vec![vec![std::u32::MAX; 2]; m]; n];
    let mut que = BinaryHeap::new();
    for (d, dirsd) in dirs.iter().enumerate().take(2) {
        let mut cur = 0;
        let (mut nx, mut ny): (isize, isize) = (0, 0);
        for i in 0..10 {
            nx += dirsd.0;
            ny += dirsd.1;

            if 0 <= nx && nx < n as isize && 0 <= ny && ny < m as isize {
                cur += mat[nx as usize][ny as usize] as i32;
                if 3 <= i {
                    dist[nx as usize][ny as usize][d] = cur as u32;
                    que.push((-cur, (nx, ny, d)));
                }
            }
        }
    }
    while !que.is_empty() {
        let (mut cur, (x, y, d)) = que.pop().unwrap();
        cur = -cur;

        if dist[x as usize][y as usize][d] < cur as u32 {
            continue;
        }
        if x == (n - 1) as isize && y == (m - 1) as isize {
            return Some(cur as u32);
        }
        for nd in [(d + 1) % 4, (d + 3) % 4] {
            let mut ncur = 0;
            let nd2 = nd % 2;
            let (mut nx, mut ny) = (x, y);
            for i in 0..10 {
                nx += dirs[nd].0;
                ny += dirs[nd].1;
                if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                    ncur += mat[nx as usize][ny as usize] as u32;
                    if 3 <= i
                        && dist[nx as usize][ny as usize][nd2]
                            > dist[x as usize][y as usize][d] + ncur
                    {
                        dist[nx as usize][ny as usize][nd2] =
                            dist[x as usize][y as usize][d] + ncur;
                        que.push((-(dist[nx as usize][ny as usize][nd2] as i32), (nx, ny, nd2)));
                    }
                }
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(94));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
