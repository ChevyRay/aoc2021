fn part1(input: &'static str) -> i32 {
    let last = input.lines().next().unwrap().len() - 1;
    let nums: Vec<i32> = input
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();
    let (g, e) = (0..=last)
        .rev()
        .map(|i| {
            let bit = 1 << i;
            let (z, o) = nums.iter().fold((0, 0), |(z, o), n| {
                if (n & bit) == bit {
                    (z, o + 1)
                } else {
                    (z + 1, o)
                }
            });
            if o > z {
                (bit, 0)
            } else {
                (0, bit)
            }
        })
        .fold((0, 0), |(g, e), (gg, ee)| (g | gg, e | ee));
    g * e
}

fn part2(input: &'static str) -> i32 {
    let last = input.lines().next().unwrap().len() - 1;
    let nums: Vec<i32> = input
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();
    let mut o_nums = nums.clone();
    let mut c_nums = nums;
    for bit in (0..=last).rev().map(|i| 1 << i) {
        let count = |nums: &[i32]| {
            nums.iter().fold((0, 0), |(z, o), n| {
                if (n & bit) == bit {
                    (z, o + 1)
                } else {
                    (z + 1, o)
                }
            })
        };
        let reduce =
            |nums: Vec<i32>, want| nums.iter().copied().filter(|n| n & bit == want).collect();
        if o_nums.len() > 1 {
            let (z, o) = count(&o_nums);
            o_nums = reduce(o_nums, if o >= z { bit } else { 0 });
        }
        if c_nums.len() > 1 {
            let (z, o) = count(&c_nums);
            c_nums = reduce(c_nums, if o < z { bit } else { 0 });
        }
    }
    o_nums[0] * c_nums[0]
}

fn main() {
    use std::time::Instant;

    let input = include_str!("day3.txt");

    let t = Instant::now();
    let solution1 = part1(input);
    let time1 = Instant::now() - t;

    let t = Instant::now();
    let solution2 = part2(input);
    let time2 = Instant::now() - t;

    assert_eq!(solution1, 4118544);
    assert_eq!(solution2, 3832770);

    println!("PART 1: {} ({} μs)", solution1, time1.as_micros());
    println!("PART 2: {} ({} μs)", solution2, time2.as_micros());
}

#[test]
fn test() {
    let input = include_str!("day3_test.txt");
    assert_eq!(part1(input), 198);
    assert_eq!(part2(input), 230);
}
