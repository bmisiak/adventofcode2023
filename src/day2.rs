use itertools::*;
const INPUT: &str = include_str!("input/day2.txt");

pub fn part1() -> u32 {
    let mut sum_of_game_ids = 0;
    for line in INPUT.lines() {
        let (game, turns) = line.split(": ").collect_tuple().unwrap();
        let game: u32 = game[5..].parse().unwrap();
        let mut game_max = (0, 0, 0);
        for turn in turns.split("; ") {
            for color in turn.split(", ") {
                let (num, color) = color.split(' ').collect_tuple().unwrap();
                let num: u32 = num.parse().unwrap();
                match color {
                    "red" => game_max.0 = game_max.0.max(num),
                    "green" => game_max.1 = game_max.1.max(num),
                    "blue" => game_max.2 = game_max.2.max(num),
                    _ => panic!("unknown color"),
                };
            }
        }
        if game_max.0 <= 12 && game_max.1 <= 13 && game_max.2 <= 14 {
            sum_of_game_ids += game;
        }
    }
    sum_of_game_ids
}

pub fn part2() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let (_game, turns) = line.split(": ").collect_tuple().unwrap();
            let mut game_max = [0, 0, 0];
            for turn in turns.split("; ") {
                for color in turn.split(", ") {
                    let (amount, color) = color.split(' ').collect_tuple().unwrap();
                    let amount: u32 = amount.parse().unwrap();
                    let color = match color {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!(),
                    };
                    game_max[color] = game_max[color].max(amount);
                }
            }
            game_max[0] * game_max[1] * game_max[2]
        })
        .sum()
}

#[test]
fn test_day2() {
    assert_eq!(part1(), 2541);
    assert_eq!(part2(), 66016);
}
