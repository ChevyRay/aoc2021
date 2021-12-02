fn part1(input: &'static str) -> i32 {
    let (depth, x) = input
        .lines()
        .map(|s| {
            let (dir, num) = s.split_once(" ").unwrap();
            (dir, num.parse::<i32>().unwrap())
        })
        .fold((0, 0), |(depth, x), (dir, num)| match dir {
            "forward" => (depth, x + num),
            "down" => (depth + num, x),
            "up" => (depth - num, x),
            _ => (depth, x),
        });
    depth * x
}

fn part2(input: &'static str) -> i32 {
    let (depth, x, _) = input
        .lines()
        .map(|s| {
            let (dir, num) = s.split_once(" ").unwrap();
            (dir, num.parse::<i32>().unwrap())
        })
        .fold((0, 0, 0), |(depth, x, aim), (dir, num)| match dir {
            "forward" => (depth + aim * num, x + num, aim),
            "down" => (depth, x, aim + num),
            "up" => (depth, x, aim - num),
            _ => (depth, x, aim),
        });
    depth * x
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
    let input = include_str!("day1_test.txt");
    assert_eq!(solve_part1(input), 7);
    assert_eq!(solve_part2(input), 5);
    assert_eq!(solve_part2_no_collect(input), 5);
}
