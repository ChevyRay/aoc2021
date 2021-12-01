fn solve(input: &'static str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .fold((0, None), |(c, p), n| {
            (if n > p.unwrap_or(n) { c + 1 } else { c }, Some(n))
        })
        .0
}

fn main() {
    let input = include_str!("input.txt");
    println!("SOLUTION: {}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(solve(input), 7);
}
