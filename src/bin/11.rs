use core::cmp::{max, min};
advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ar = Vec::new();
    let mut gal = vec![];

    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        ar.push(line);
        for j in 0..line.len() {
            if line.chars().nth(j).unwrap() == '#' {
                gal.push((i, j));
            }
        }
    }
    let n = ar.len();
    let m = ar[0].len();
    let mut row = vec![0; n];
    let mut col = vec![0; m];
    for i in 0..n {
        row[i] = if i > 0 { row[i - 1] } else { 0 };
        if ar[i].chars().filter(|c| *c == '.').count() == ar[i].len() {
            row[i] += 1;
        }
    }
    for j in 0..m {
        let mut pos = true;
        for it in ar.iter() {
            if it.chars().nth(j) == Some('#') {
                pos = false;
                break;
            }
        }
        col[j] = if j > 0 { col[j - 1] } else { 0 };
        if pos {
            col[j] += 1;
        }
    }
    let mut ans: u32 = 0;
    for i in 0..gal.len() {
        for j in i..gal.len() {
            let (x1, y1) = (min(gal[i].0, gal[j].0), min(gal[i].1, gal[j].1));
            let (x2, y2) = (max(gal[i].0, gal[j].0), max(gal[i].1, gal[j].1));
            let cur = x2 - x1 + y2 - y1 + (row[x2] - row[x1]) + (col[y2] - col[y1]);
            ans += cur as u32;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ar = Vec::new();
    let mut gal = vec![];

    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        ar.push(line);
        for j in 0..line.len() {
            if line.chars().nth(j).unwrap() == '#' {
                gal.push((i, j));
            }
        }
    }
    let n = ar.len();
    let m = ar[0].len();
    let mut row = vec![0; n];
    let mut col = vec![0; m];
    for i in 0..n {
        row[i] = if i > 0 { row[i - 1] } else { 0 };
        if ar[i].chars().filter(|c| *c == '.').count() == ar[i].len() {
            row[i] += 1;
        }
    }
    for j in 0..m {
        let mut pos = true;
        for it in ar.iter() {
            if it.chars().nth(j) == Some('#') {
                pos = false;
                break;
            }
        }
        col[j] = if j > 0 { col[j - 1] } else { 0 };
        if pos {
            col[j] += 1;
        }
    }
    let mut ans: u64 = 0;
    for i in 0..gal.len() {
        for j in i..gal.len() {
            let (x1, y1) = (min(gal[i].0, gal[j].0), min(gal[i].1, gal[j].1));
            let (x2, y2) = (max(gal[i].0, gal[j].0), max(gal[i].1, gal[j].1));
            let cur = (x2 - x1 + y2 - y1) as u64
                + (row[x2] - row[x1] + col[y2] - col[y1]) as u64 * (1000000 - 1);
            ans += cur;
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
