use std::collections::HashMap;
advent_of_code::solution!(7);

fn card_rank(card: (String, u32)) -> u32 {
    let s = card.0;
    let mut cc = HashMap::new();
    for c in s.chars(){
        let count = cc.entry(c).or_insert(0);
        *count += 1;
    }
    match cc.len() {
        5 => 1,
        4 => 2,
        3 => {
            if cc.values().max().unwrap() == &2 {
                3
            }
            else{
                4
            }
        },
        2 => {
            if cc.values().max().unwrap() == &4 {
                6
            }
            else{
                5
            }
        },
        1 => 7,
        _ => panic!("Invalid card")
    }
}

fn char_rank(c: char) -> u32 {
    match c {
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => c.to_digit(10).unwrap() - 1,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cards: Vec<(String, u32)> = Vec::new();
    
    for line in input.lines(){
        if !line.trim().is_empty() {
            let (card, bid) = line.split_at(line.find(' ').unwrap());
            cards.push((card.to_string(), bid.trim().parse::<u32>().unwrap()));
        }
        
    }

    cards.sort_by(|a, b| {
        // Comparaison des chaînes (String)
        let ra = card_rank(a.clone());
        let rb = card_rank(b.clone());
        if ra != rb {
            ra.cmp(&rb)
        }
        else{
            for i in 0..5{
                let ca = char_rank(a.0.chars().nth(i).unwrap());
                let cb = char_rank(b.0.chars().nth(i).unwrap());
                if ca != cb {
                    return ca.cmp(&cb);
                }
            }
            std::cmp::Ordering::Equal
        }
    });
    let mut ans: u32 = 0;
    for (i, card) in cards.iter().enumerate() {
        ans += (i as u32 + 1)  * card.1;
    }
    Some(ans)
}

fn card_rank2(card: (String, u32)) -> u32 {
    let s = card.0;
    let mut cc = HashMap::new();
    let mut jj = 0;
    for c in s.chars(){
        if c == 'J'{
            jj += 1;
        }
        else{
            let count = cc.entry(c).or_insert(0);
            *count += 1;
        }
        
    }
    if jj == 0{
        match cc.len() {
            5 => 1,
            4 => 2,
            3 => {
                if cc.values().max().unwrap() == &2 {
                    3
                }
                else{
                    4
                }
            },
            2 => {
                if cc.values().max().unwrap() == &4 {
                    6
                }
                else{
                    5
                }
            },
            1 => 7,
            _ => panic!("Invalid card")
        }
    }
    else{
        match cc.len() {
            5 => panic!("Invalid card"),
            4 => 2,
            3 => 4,
            2 => {
                if jj == 1 {
                    if cc.values().max().unwrap() == &3 {
                        6
                    }
                    else{
                        5
                    }
                }
                else{
                    6
                }
            },
            1 => 7,
            0 => 7,
            _ => panic!("Invalid card")
        }
    }
    
}

fn char_rank2(c: char) -> u32 {
    match c {
        'T' => 9,
        'J' => 0,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => c.to_digit(10).unwrap() - 1,
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<(String, u32)> = Vec::new();
    
    for line in input.lines(){
        if !line.trim().is_empty() {
            let (card, bid) = line.split_at(line.find(' ').unwrap());
            cards.push((card.to_string(), bid.trim().parse::<u32>().unwrap()));
        }
        
    }

    cards.sort_by(|a, b| {
        // Comparaison des chaînes (String)
        let ra = card_rank2(a.clone());
        let rb = card_rank2(b.clone());
        if ra != rb {
            ra.cmp(&rb)
        }
        else{
            for i in 0..5{
                let ca = char_rank2(a.0.chars().nth(i).unwrap());
                let cb = char_rank2(b.0.chars().nth(i).unwrap());
                if ca != cb {
                    return ca.cmp(&cb);
                }
            }
            std::cmp::Ordering::Equal
        }
    });
    let mut ans: u32 = 0;
    for (i, card) in cards.iter().enumerate() {
        ans += (i as u32 + 1)  * card.1;
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
