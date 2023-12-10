advent_of_code::solution!(9);


pub fn part_one(input: &str) -> Option<i32> {
    let mut vals: Vec<Vec<i32>> = Vec::new();
    let mut ans = 0;
    for (_i, line) in input.lines().enumerate(){
        if !line.trim().is_empty()  {
            vals.push(Vec::new());
            line.split_whitespace().for_each(|x| vals[0].push(x.parse::<i32>().unwrap()));

            while vals[vals.len() - 1].iter().min() != vals[vals.len() - 1].iter().max() || vals[vals.len() - 1][0] != 0{
                let mut nval = Vec::new();
                for i in 0..vals[vals.len() - 1].len()-1{
                    nval.push(vals[vals.len() - 1][i + 1] - vals[vals.len() - 1][i]);
                }
                //println!("{:?}",nval);
                vals.push(nval);
            }
            let n = vals.len();
            vals[n - 1].push(0);
            for i in (0..n-1).rev(){
                let c1 = *vals[i + 1].last().unwrap();
                let c = *vals[i].last().unwrap();
                vals[i].push(c + c1);
                //println!("{} {:?}",i, vals[i]);
            }
            
            ans += *vals[0].last().unwrap();
            vals.clear();
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut vals: Vec<Vec<i32>> = Vec::new();
    let mut ans = 0;
    for (_i, line) in input.lines().enumerate(){
        if !line.trim().is_empty()  {
            vals.push(Vec::new());
            line.split_whitespace().for_each(|x| vals[0].push(x.parse::<i32>().unwrap()));

            while vals[vals.len() - 1].iter().min() != vals[vals.len() - 1].iter().max() || vals[vals.len() - 1][0] != 0{
                let mut nval = Vec::new();
                for i in 0..vals[vals.len() - 1].len()-1{
                    nval.push(vals[vals.len() - 1][i + 1] - vals[vals.len() - 1][i]);
                }
                //println!("{:?}",nval);
                vals.push(nval);
            }
            let n = vals.len();
            let mut cur = 0;
            for i in (0..n-1).rev(){
                cur = vals[i][0] - cur;
            }
            
            ans += cur;
            vals.clear();
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
