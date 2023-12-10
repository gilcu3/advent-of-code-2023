use std::vec;

advent_of_code::solution!(10);

fn todir(from: usize, c: char) -> usize{
    match c{
        '|' => {
            if from % 2 == 1 {
                (from + 2) % 4
            }else{
                4
            }
        },
        '-' => {
            if from % 2 == 0 {
                (from + 2) % 4
            }else{
                4
            }
        },
        'L' => {
            if from == 0 {
                1
            }
            else if from == 1{
                0
            }
            else{
                4
            }
        },
        'J' => {
            if from == 2 {
                1
            }
            else if from == 1{
                2
            }
            else{
                4
            }
        },
        '7' => {
            if from == 2 {
                3
            }
            else if from == 3{
                2
            }
            else{
                4
            }
        },
        'F' => {
            if from == 0 {
                3
            }
            else if from == 3{
                0
            }
            else{
                4
            }
        },
        _ => 4,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut mm: Vec<String> = Vec::new();
    for (_i, line) in input.lines().into_iter().enumerate(){
        if line.trim().len() > 0  {
            mm.push(line.trim().to_string());
        }
    }
    
    let n = mm.len();
    let m = mm[0].len();
    let  (mut si, mut sj) = (-1, -1);
    for i in 0..n{
        for j in 0..m{
            if mm[i].chars().nth(j).unwrap() == 'S'{
                si = i as i32;
                sj = j as i32;
            }
        }
    }
    //println!("{} {}", si, sj);
    let mut dis:Vec<Vec<isize>> = vec![vec![-1; m]; n];
    dis[si as usize][sj as usize] = 0;
    let mut que = Vec::new();
    let dirs = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];
    for d in 0..4{
        let (di, dj) = dirs[d];
        let (ni, nj) = (si + di, sj + dj);
        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32{
            let c = mm[ni as usize].chars().nth(nj as usize).unwrap();
            let dd = todir((d + 2) % 4, c);
            //println!("{} {} {} {} {}", ni, nj, c, d, dd);
            if dd < 4{
                que.push((ni, nj, dd));
                dis[ni as usize][nj as usize] = 1;
            }
            
        }
    }
    let mut ans =0;
    let mut front = 0;
    while front < que.len(){
        let (i, j, d) = que[front];
        //println!("{} {} {}", i, j, d);
        front += 1;
        ans = std::cmp::max(ans, dis[i as usize][j as usize]);
        let (di, dj) = dirs[d];
        let (ni, nj) = (i + di, j + dj);
        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 && dis[ni as usize][nj as usize] == -1{
            let c = mm[ni as usize].chars().nth(nj as usize).unwrap();
            let dd = todir((d + 2) % 4, c);
            if dd < 4{
                que.push((ni, nj, dd));
                dis[ni as usize][nj as usize] = dis[i as usize][j as usize] + 1;
            }
        }
        
    }
    Some(ans as usize)
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut mm: Vec<String> = Vec::new();
    for (_i, line) in input.lines().into_iter().enumerate(){
        if line.trim().len() > 0  {
            mm.push(line.trim().to_string());
        }
    }
    
    let n = mm.len();
    let m = mm[0].len();
    let  (mut si, mut sj) = (-1, -1);
    for i in 0..n{
        for j in 0..m{
            if mm[i].chars().nth(j).unwrap() == 'S'{
                si = i as i32;
                sj = j as i32;
            }
        }
    }
    //println!("{} {}", si, sj);
    let mut seen = vec![vec![false; m]; n];
    seen[si as usize][sj as usize] = true;
    let mut que = Vec::new();
    let dirs = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut ddir = Vec::new();
    for d in 0..4{
        let (di, dj) = dirs[d];
        let (ni, nj) = (si + di, sj + dj);
        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32{
            let c = mm[ni as usize].chars().nth(nj as usize).unwrap();
            let dd = todir((d + 2) % 4, c);
            //println!("{} {} {} {} {}", ni, nj, c, d, dd);
            if dd < 4{
                ddir.push(d);
                que.push((ni, nj, dd));
                seen[ni as usize][nj as usize] = true;
            }
            
        }
    }
    
    let c = match (ddir[0], ddir[1]){
        (0, 3) => {
            "F"
        },
        (0, 1) => {
            "L"
        },
        (1, 2) => {
            "J"
        },
        (2, 3) => {
            "7"
        },
        (1, 3) => {
            "|"
        },
        (0, 2) => {
            "-"
        },
        _ => panic!(),
    };
    mm[si as usize] = mm[si as usize].replace("S", c);
    
    let mut front = 0;
    while front < que.len(){
        let (i, j, d) = que[front];
        //println!("{} {} {}", i, j, d);
        front += 1;
        let (di, dj) = dirs[d];
        let (ni, nj) = (i + di, j + dj);
        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 && !seen[ni as usize][nj as usize]{
            let c = mm[ni as usize].chars().nth(nj as usize).unwrap();
            let dd = todir((d + 2) % 4, c);
            if dd < 4{
                que.push((ni, nj, dd));
                seen[ni as usize][nj as usize] = true;
            }
        }
        
    }
    //println!("{}", mm[si as usize].chars().nth(sj as usize).unwrap());
    let mut ans = 0;
    for i in 0..n{
        let mut c = 0;
        let mut ss = '.';
        for j in 0..m{
            if seen[i][j]{
                let cc = mm[i].chars().nth(j).unwrap();
                match cc{
                    'F' => {
                        c += 1;
                        ss = 'F';
                    },
                    'L' => {
                        c += 1;
                        ss = 'L';
                    },
                    'J' => {
                        if ss != 'F'{
                            c += 1;
                        } 
                        ss = '.';
                    },
                    '7' => {
                        if ss != 'L'{
                            c += 1;
                        } 
                        ss = '.';
                    },
                    '|' => {
                        c += 1;
                    },
                    '-' => {
                        
                    },
                    _ => {
                        panic!();
                    }
                }
            }
            else if c % 2 == 1{
                //println!("{} {} {}", i, j, mm[i].chars().nth(j).unwrap());
                ans += 1;
            }
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let tests = &[(1, 4), (2, 4), (3, 8), (4, 8)];

        for (part, expected) in tests {
            let result = part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, *part,
            ));
            assert_eq!(result, Some(*expected));
        }
    }

    #[test]
    fn test_part_two() {
        let tests = &[(5, 4), (6, 4), (7, 8), (8, 10)];

        for (part, expected) in tests {
            let result = part_two(&advent_of_code::template::read_file_part(
                "examples", DAY, *part,
            ));
            assert_eq!(result, Some(*expected));
        }
    }
}
