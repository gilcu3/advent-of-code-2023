use std::collections::HashMap;

advent_of_code::solution!(2);


fn parse_game_data(data: String) -> Vec<(u32, Vec<HashMap<String, u32>>)> {
    let mut games = Vec::new();

    for line in data.split("\n") {
        if line.trim().is_empty() {
            continue;
        }
        let mut it = line.trim().split(':');
        let mut it2 = it.next().unwrap().split(' ');
        it2.next();
        let gameid = it2.next().unwrap().parse::<u32>().unwrap();
        let game = it.next().unwrap();
        let mut rounds = Vec::new();

        for round in game.trim().split(';') {
            //println!("{}", round);
            let mut colors_count = HashMap::new();

            for pair in round.trim().split(',') {
                let mut iter = pair.trim().split_whitespace();
                let count = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap().to_string();
                colors_count.insert(color, count);
            }

            rounds.push(colors_count);
        }

        games.push((gameid, rounds));
    }
    games
}


pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_game_data(input.to_string());
    let mx = HashMap::from([("blue", 14), ("green", 13), ("red", 12)]);
    let mut ans: u32 = 0;
    for (gameid, rounds) in games {
        
        let mut possible: bool = true;
        for round in rounds {
            for (color, count) in round {
                if count > *mx.get(color.as_str()).unwrap() {
                    possible = false;
                    break;
                }
            }
        }
        if possible{
            ans += gameid;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_game_data(input.to_string());
    let mut ans: u32 = 0;
    for (_gameid, rounds) in games {
        
        let mut mx = HashMap::from([("blue".to_string(), 0), ("green".to_string(), 0), ("red".to_string(), 0)]);
        for round in rounds {
            for (color, count) in round {
                if count > *mx.get(&color).unwrap() {
                    mx.insert(color, count);
                }
            }
        }
        let mut power = 1;
        for (_, count) in mx {
            power *= count;
        }
        ans += power;
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
