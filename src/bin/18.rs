advent_of_code::solution!(18);

fn dir2num(c: char) -> usize {
    match c {
        'R' => 1,
        'D' => 0,
        'U' => 2,
        'L' => 3,
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let dirs: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut moves = vec![];
    let (mut mx, mut nx, mut my, mut ny) = (0, 0, 0, 0);
    let (mut x, mut y) = (0, 0);
    for line in input.lines() {
        if !line.trim().is_empty() {
            let ds = line.split_whitespace().take(2).collect::<Vec<&str>>();
            let (d, s) = (
                dir2num(ds[0].chars().next().unwrap()),
                ds[1].parse::<isize>().unwrap(),
            );
            x += dirs[d].0 * s;
            y += dirs[d].1 * s;
            if x > mx {
                mx = x
            }
            if x < nx {
                nx = x
            }
            if y > my {
                my = y
            }
            if y < ny {
                ny = y
            }
            moves.push((d, s));
        }
    }
    let (sx, sy, kx, ky) = (
        (-nx) as usize,
        (-ny) as usize,
        (mx - nx + 1) as usize,
        (my - ny + 1) as usize,
    );
    let mut mat = vec![vec![false; ky]; kx];
    (x, y) = (sx as isize, sy as isize);
    mat[x as usize][y as usize] = true;
    for (d, s) in moves {
        for _ in 0..s {
            x += dirs[d].0;
            y += dirs[d].1;
            mat[x as usize][y as usize] = true;
        }
    }
    let mut outside = 0;
    let mut que = vec![];
    for (i, mati) in mat.iter_mut().enumerate() {
        if !mati[0] {
            que.push((i, 0));
            mati[0] = true;
        }
        if !mati[ky - 1] {
            que.push((i, ky - 1));
            mati[ky - 1] = true;
        }
    }
    for j in 0..ky {
        if !mat[0][j] {
            que.push((0, j));
            mat[0][j] = true;
        }
        if !mat[kx - 1][j] {
            que.push((kx - 1, j));
            mat[kx - 1][j] = true;
        }
    }
    let mut front = 0;
    while front < que.len() {
        let (cx, cy) = que[front];
        front += 1;
        outside += 1;
        for dirsd in dirs.iter() {
            let (ncx, ncy) = (cx as isize + dirsd.0, cy as isize + dirsd.1);
            if ncx >= 0
                && ncx < kx as isize
                && ncy >= 0
                && ncy < ky as isize - 1
                && !mat[ncx as usize][ncy as usize]
            {
                mat[ncx as usize][ncy as usize] = true;
                que.push((ncx as usize, ncy as usize));
            }
        }
    }
    let ans = (kx * ky - outside) as u32;
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut moves = vec![];
    let (mut mx, mut nx, mut my, mut ny) = (0, 0, 0, 0);
    let (mut x, mut y) = (0, 0);
    for line in input.lines() {
        if !line.trim().is_empty() {
            let ds = line.split_whitespace().nth(2).unwrap();
            let s = i32::from_str_radix(&ds[2..7], 16).unwrap();
            let d = ds[7..8].parse::<usize>().unwrap();
            x += (dirs[d].0 as i32) * s;
            y += (dirs[d].1 as i32) * s;
            if x > mx {
                mx = x
            }
            if x < nx {
                nx = x
            }
            if y > my {
                my = y
            }
            if y < ny {
                ny = y
            }
            moves.push((d, s));
        }
    }
    let (sx, sy, kx, _ky) = (
        (-nx) as usize,
        (-ny) as usize,
        (mx - nx + 1) as usize,
        (my - ny + 1) as usize,
    );
    let mut rows = vec![vec![]; kx];
    let n = moves.len();
    (x, y) = (sx as i32, sy as i32);
    for (i, (d, s)) in moves.iter().enumerate() {
        if *d == 1 || *d == 3 {
            for _ in 0..s - 1 {
                x += dirs[*d].0 as i32;
                y += dirs[*d].1 as i32;
                rows[x as usize].push((y, y, true));
            }
            x += dirs[*d].0 as i32;
            y += dirs[*d].1 as i32;
        } else {
            let (mut y0, mut y1) = (y, y + dirs[*d].1 as i32 * s);
            if y1 < y0 {
                (y0, y1) = (y1, y0)
            }
            rows[x as usize].push((y0, y1, moves[(i + 1) % n].0 == moves[(i + n - 1) % n].0));
            x += dirs[*d].0 as i32 * s;
            y += dirs[*d].1 as i32 * s;
        }
    }
    let mut ans = 0;
    for rowx in rows.iter_mut() {
        rowx.sort();
        let mut cur = false;
        let mut last = -1;
        for (y0, y1, change) in rowx.iter() {
            //println!("{} {} {} {}", x, y0, y1, change);
            ans += (y1 - y0 + 1) as u64;
            //println!("{} {}", ans, (y0 - last - 1));
            if cur {
                ans += (y0 - last - 1) as u64
            }
            if *change {
                cur = !cur
            }
            last = *y1;
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
