fn part1(input: &'static str) -> i32 {
    let (d, x) = input
        .lines()
        .filter_map(|s| {
            s.split_once(' ')
                .map(|(i, n)| (i, n.parse::<i32>().unwrap()))
        })
        .fold((0, 0), |(d, x), (i, n)| match i {
            "forward" => (d, x + n),
            "down" => (d + n, x),
            "up" => (d - n, x),
            _ => (d, x),
        });
    d * x
}

fn part2(input: &'static str) -> i32 {
    let (d, x, _) = input
        .lines()
        .filter_map(|s| {
            s.split_once(' ')
                .map(|(i, n)| (i, n.parse::<i32>().unwrap()))
        })
        .fold((0, 0, 0), |(d, x, a), (i, n)| match i {
            "forward" => (d + a * n, x + n, a),
            "down" => (d, x, a + n),
            "up" => (d, x, a - n),
            _ => (d, x, a),
        });
    d * x
}

fn main() {
    use std::time::Instant;

    let input = include_str!("day2.txt");

    let t = Instant::now();
    let solution1 = part1(input);
    let time1 = Instant::now() - t;

    let t = Instant::now();
    let solution2 = part2(input);
    let time2 = Instant::now() - t;

    println!("PART 1: {} ({} μs)", solution1, time1.as_micros());
    println!("PART 2: {} ({} μs)", solution2, time2.as_micros());
}

#[test]
fn test() {
    let input = include_str!("day2_test.txt");
    assert_eq!(part1(input), 150);
    assert_eq!(part2(input), 900);
}
