advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut tt = Vec::new();
    let mut dis = Vec::new();

    for line in input.lines() {
        if line.starts_with("Time") {
            line.split_whitespace().skip(1).for_each(|x| {
                tt.push(x.parse::<u32>().unwrap());
            });
        } else if line.starts_with("Distance") {
            line.split_whitespace().skip(1).for_each(|x| {
                dis.push(x.parse::<u32>().unwrap());
            });
        }
    }

    let mut ans: u64 = 1;
    let n = tt.len();

    for i in 0..n {
        let mx = (tt[i] + 1) / 2;
        if (tt[i] - mx) * mx <= dis[i] {
            ans = 0;
            break;
        }
        let mut a = 0;
        let mut b = mx;
        while a < b {
            let mid = (a + b) / 2;
            let s = (tt[i] - mid) * mid;
            if s > dis[i] {
                b = mid;
            } else {
                a = mid + 1;
            }
        }
        let low = a;
        a = mx;
        b = tt[i];
        while a < b {
            let mid = (a + b + 1) / 2;
            let s = (tt[i] - mid) * mid;
            if s > dis[i] {
                a = mid;
            } else {
                b = mid - 1;
            }
        }
        let high = a;
        ans *= (high - low + 1) as u64;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut tt = 0;
    let mut dis = 0;

    for line in input.lines() {
        if line.starts_with("Time") {
            tt = line
                .split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("")
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Distance") {
            dis = line
                .split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("")
                .parse::<u64>()
                .unwrap();
        }
    }

    let mx = (tt + 1) / 2;
    let mut a = 0;
    let mut b = mx;
    while a < b {
        let mid = (a + b) / 2;
        let s = (tt - mid) * mid;
        if s > dis {
            b = mid;
        } else {
            a = mid + 1;
        }
    }
    let low = a;
    a = mx;
    b = tt;
    while a < b {
        let mid = (a + b + 1) / 2;
        let s = (tt - mid) * mid;
        if s > dis {
            a = mid;
        } else {
            b = mid - 1;
        }
    }
    let high = a;
    let ans = high - low + 1;
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
