use std::collections::HashSet;

advent_of_code::solution!(4);


pub fn part_one(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;
    for line in input.lines(){
        let mut it = line.split(':');
        it.next();
        let mut cardit = it.next().unwrap().trim().split('|');
        let mut good = HashSet::new();
        for n in cardit.next().unwrap().trim().split(' ') {
            if n.trim().len() == 0{
                continue;
            }
            good.insert(n.trim().parse::<u32>().unwrap());
        }
        let mut cur = 0;
        for n in cardit.next().unwrap().trim().split(' ') {
            if n.trim().len() == 0{
                continue;
            }
            if good.contains(&n.trim().parse::<u32>().unwrap()){
                if cur == 0{
                    cur = 1;
                }
                else{
                    cur *= 2;
                }
                
            }
        }
        ans += cur;
    }
    Some(ans)
}



pub fn part_two(input: &str) -> Option<u32> {
    let n = input.lines().count();
    let mut cc: Vec<u32> = std::iter::repeat(1).take(n).collect::<Vec<_>>();

    let mut ans: u32 = 0;
    for (i, line) in input.lines().enumerate(){
        let mut it = line.split(':');
        it.next();
        let mut cardit = it.next().unwrap().trim().split('|');
        let mut good = HashSet::new();
        for n in cardit.next().unwrap().trim().split(' ') {
            if n.trim().len() == 0{
                continue;
            }
            good.insert(n.trim().parse::<u32>().unwrap());
        }
        let mut cur = 0;
        let mut m = 0;
        for n in cardit.next().unwrap().trim().split(' ') {
            if n.trim().len() == 0{
                continue;
            }
            if good.contains(&n.trim().parse::<u32>().unwrap()){
                m += 1;
                if cur == 0{
                    cur = 1;
                }
                else{
                    cur *= 2;
                }
                
            }
        }
        for j in 0..m{
            cc[i+j + 1] += cc[i];
        }
        ans += cc[i];
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
