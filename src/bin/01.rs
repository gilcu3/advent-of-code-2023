advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;
    for line in input.lines(){
        let mut first: bool = false;
        let mut cur: u32 = 0;
        for c in line.chars() {
            // print char
            if c.is_ascii_digit() {
                if !first{
                    cur += 10 * c.to_digit(10).unwrap();
                    first = true;
                }
                cur = cur / 10 * 10 + c.to_digit(10).unwrap();
            }
        }
        ans += cur;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in input.lines(){
        let mut first: bool = false;
        let mut cur: u32 = 0;

        for i in 0..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c.is_ascii_digit() {
                if !first{
                    cur += 10 * c.to_digit(10).unwrap();
                    first = true;
                }
                cur = cur / 10 * 10 + c.to_digit(10).unwrap();
            }
            else if c.is_alphabetic() {
                let mut d: u32 = 0;
                for (k, dig) in digits.iter().enumerate() {
                    if line[i..].starts_with(dig) {
                        d = (k + 1) as u32;
                        break;
                    }
                }
                if d != 0 {
                    if !first{
                        cur += 10 * d;
                        first = true;
                    }
                    cur = cur / 10 * 10 + d;
                }
            }
        }
        ans += cur;
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
