use itertools::Itertools;

const INPUT: &str = include_str!("input/day3.txt");

pub fn part1() -> u32 {
    let bytes = INPUT.as_bytes();
    let total_len = bytes.len();
    let line_len = bytes.iter().position(|&ch| ch == b'\n').unwrap();
    let lines: Vec<_> = bytes.chunks_exact(line_len+1).collect();
    
    let lines = INPUT.lines().map(str::as_bytes).tuple_windows();
    for (prev_line, cur_line, next_line) in lines {
        let mut parsing_number = None;
        for (pos, ch) in cur_line.iter().enumerate() {
            match ch {
                b'0'..=b'9' => parsing_number = Some(parsing_number.unwrap_or(0) * 10 + ch),
                b'.' => {},
                _ => {},
            }
        }
    }
    0
}

pub fn part2() -> u32 {
    0
}

#[test]
fn test_day3() {
    assert_eq!(part1(), 4361);
    assert_eq!(part2(), 0);
}