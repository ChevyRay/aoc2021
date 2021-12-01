fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .fold((0, i32::MAX), |(c, p), n| {
            (if n > p { c + 1 } else { c }, n)
        })
        .0
}

fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|s| s.parse().ok())
        .collect::<Option<Vec<_>>>()
        .and_then(|nums| {
            Some(
                nums.windows(3)
                    .map(|n| n.iter().sum())
                    .fold((0, i32::MAX), |(c, p), n| {
                        (if n > p { c + 1 } else { c }, n)
                    })
                    .0,
            )
        })
        .unwrap()
}

fn solve_part2_no_collect(input: &str) -> i32 {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .fold((0, i32::MAX, None, None), |(n, p, a, b), c| {
            a.and_then(|a| {
                b.and_then(|b| {
                    Some((
                        if a + b + c > p { n + 1 } else { n },
                        a + b + c,
                        Some(b),
                        Some(c),
                    ))
                })
            })
            .unwrap_or_else(|| (n, p, b, Some(c)))
        })
        .0
}

fn main() {
    use std::time::Instant;

    let input = include_str!("input.txt");

    let t = Instant::now();
    let solution1 = solve_part1(input);
    let time1 = Instant::now() - t;

    let t = Instant::now();
    let solution2 = solve_part2(input);
    let time2 = Instant::now() - t;

    let t = Instant::now();
    let solution2_no_collect = solve_part2_no_collect(input);
    let time2_no_collect = Instant::now() - t;

    println!("PART 1: {} ({} μs)", solution1, time1.as_micros());
    println!("PART 2: {} ({} μs)", solution2, time2.as_micros());
    println!(
        "PART 2: {} ({} μs) (no Vec allocation)",
        solution2_no_collect,
        time2_no_collect.as_micros()
    );
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(solve_part1(input), 7);
    assert_eq!(solve_part2(input), 5);
}
