const INPUT: &str = include_str!("input/day1.txt");

pub fn part1() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let mut iter = line
                .chars()
                .filter(|ch| ch.is_numeric())
                .filter_map(|ch| ch.to_digit(10));
            let first = iter.next();
            let last = iter.next_back().or(first);
            first.unwrap_or(0) * 10 + last.unwrap_or(0)
        })
        .sum()
}

pub fn part2() -> u32 {
    INPUT
        .lines()
        .map(str::as_bytes)
        .map(|line| (0..line.len()).filter_map(|pos| {
            if let ascii_digit @ b'1'..=b'9' = line[pos] {
                Some((ascii_digit - b'0') as u32)
            } else {
                match &line[pos.saturating_sub(2)..=pos] {
                    b"one" => Some(1),
                    b"two" => Some(2),
                    b"six" => Some(6),
                    _ => None,
                }
                .or(match &line[pos.saturating_sub(3)..=pos] {
                    b"four" => Some(4),
                    b"five" => Some(5),
                    b"nine" => Some(9),
                    _ => None,
                })
                .or(match &line[pos.saturating_sub(4)..=pos] {
                    b"three" => Some(3),
                    b"seven" => Some(7),
                    b"eight" => Some(8),
                    _ => None,
                })
            }
        }))
        .map(|mut digits| {
            let first = digits.next();
            let last = digits.next_back().or(first);
            first.unwrap_or(0) * 10 + last.unwrap_or(0)
        })
        .sum()
}

#[test]
fn test_day1() {
    assert_eq!(part1(), 56108);
    assert_eq!(part2(), 55652);
}