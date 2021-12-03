fn part1(input: &'static str) -> i32 {
    let last = input.lines().next().unwrap().len() - 1;
    let nums = input.lines().map(|s| i32::from_str_radix(s, 2).unwrap());
    let (g, e) = (0..=last)
        .map(|i| {
            let bit = 1 << i;
            let (z, o) = nums.clone().fold((0, 0), |(z, o), n| {
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
    let (nums, max) = input.lines().fold((Vec::new(), 0), |(mut nums, max), s| {
        nums.push(i32::from_str_radix(s, 2).unwrap());
        (nums, max.max(s.len()))
    });

    let mut o_nums = nums.clone();
    let mut c_nums = nums;

    let count = |nums: &[i32], bit| {
        nums.iter().fold((0, 0), |(zeros, ones), n| {
            if (n & bit) == bit {
                (zeros, ones + 1)
            } else {
                (zeros + 1, ones)
            }
        })
    };

    for i in 1..=max {
        let i = max - i;
        let bit = 1 << i;
        if o_nums.len() > 1 {
            let (zeros, ones) = count(&o_nums, bit);
            let want = if ones >= zeros { bit } else { 0 };
            o_nums = o_nums.iter().copied().filter(|n| n & bit == want).collect();
        }
        if c_nums.len() > 1 {
            let (zeros, ones) = count(&c_nums, bit);
            let want = if ones < zeros { bit } else { 0 };
            c_nums = c_nums.iter().copied().filter(|n| n & bit == want).collect();
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
