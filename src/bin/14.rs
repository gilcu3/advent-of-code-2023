advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut mat = vec![];
    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        mat.push(vec![0; line.len()]);
        for (j, c) in line.trim().char_indices() {
            mat[i][j] = match c {
                'O' => 1,
                '.' => 0,
                '#' => 2,
                _ => panic!(),
            }
        }
    }
    let (n, m) = (mat.len(), mat[0].len());
    let mut col = vec![0; m];
    let mut ans = 0;
    for (i, mati) in mat.iter().enumerate() {
        for (j, matj) in mati.iter().enumerate() {
            match matj {
                0 => (),
                1 => {
                    ans += (n - col[j]) as u32;
                    col[j] += 1;
                }
                2 => col[j] = i + 1,
                _ => panic!(),
            }
        }
    }
    Some(ans)
}

fn rotate_right(mat: &mut Vec<Vec<u8>>, n: usize) {
    let mut nmat = vec![vec![0; n]; n];
    for (i, mati) in mat.iter().enumerate() {
        for (j, nmatj) in nmat.iter_mut().enumerate() {
            nmatj[n - i - 1] = mati[j];
        }
    }
    std::mem::swap(mat, &mut nmat);
}

fn move_north(mat: &mut [Vec<u8>], n: usize) {
    for j in 0..n {
        let mut last = 0;
        for i in 0..n {
            if mat[i][j] == 1 {
                mat[i][j] = 0;
                mat[last][j] = 1;
                last += 1;
            } else if mat[i][j] == 2 {
                last = i + 1;
            }
        }
    }
}
fn make_cycle(mat: &mut Vec<Vec<u8>>, n: usize) {
    for _ in 0..4 {
        move_north(mat, n);
        rotate_right(mat, n);
    }
}

fn compute_weight(mat: &mut [Vec<u8>], n: usize) -> u32 {
    let mut ans = 0;
    for (i, mati) in mat.iter().enumerate() {
        for matj in mati.iter() {
            if *matj == 1 {
                ans += (n - i) as u32;
            }
        }
    }
    ans
}
fn compare_mat(mat1: &mut [Vec<u8>], mat2: &mut [Vec<u8>], n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            if mat1[i][j] != mat2[i][j] {
                return false;
            }
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mat = vec![];
    let cycles = 1000000000;
    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        mat.push(vec![0; line.len()]);
        for (j, c) in line.trim().char_indices() {
            mat[i][j] = match c {
                'O' => 1,
                '.' => 0,
                '#' => 2,
                _ => panic!(),
            }
        }
    }
    let n = mat.len();
    let mut i = 1;
    let mut mat1 = mat.clone();
    let mut mat2 = mat.clone();
    make_cycle(&mut mat2, n);

    while i < cycles {
        if compare_mat(&mut mat1, &mut mat2, n) {
            break;
        }
        i += 1;
        make_cycle(&mut mat1, n);

        make_cycle(&mut mat2, n);
        make_cycle(&mut mat2, n);
    }
    if i != cycles {
        for _ in 0..cycles % i + 1 {
            make_cycle(&mut mat1, n);
        }
    }
    Some(compute_weight(&mut mat1, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
