use std::time::Instant;

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .fold((0, None), |(c, p), n| {
            (if n > p.unwrap_or(n) { c + 1 } else { c }, Some(n))
        })
        .0
}

fn solve_part2(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (0..(nums.len() - 2))
        .map(|i| nums[i..][..3].iter().sum::<usize>())
        .fold((0, None), |(c, p), n| {
            (if n > p.unwrap_or(n) { c + 1 } else { c }, Some(n))
        })
        .0
}

fn main() {
    let input = include_str!("input.txt");

    let t = Instant::now();
    let solution1 = solve_part1(input);
    let time1 = Instant::now() - t;

    let t = Instant::now();
    let solution2 = solve_part2(input);
    let time2 = Instant::now() - t;

    println!("PART 1: {} ({} μs)", solution1, time1.as_micros());
    println!("PART 2: {} ({} μs)", solution2, time2.as_micros());
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(solve_part1(input), 7);
    assert_eq!(solve_part2(input), 5);
}
