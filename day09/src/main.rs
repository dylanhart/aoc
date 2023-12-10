const INPUT: &str = include_str!("input.txt");

fn extrapolate(ns: &[i64]) -> i64
{
    if ns.iter().all(|&n| n == 0) {
        return 0;
    }

    let last = ns.last().unwrap();

    let diffs: Vec<i64> = {
        ns[1..].iter().copied().scan(ns[0], |prev, curr| {
            Some(curr - core::mem::replace(prev, curr))
        }).collect()
    };

    last + extrapolate(&diffs)
}

fn main() {
    let mut lines = INPUT.lines();

    let mut sum = 0;
    let mut sum2 = 0;
    for line in lines {
        let mut ns: Vec<i64> = line.split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        sum += extrapolate(&ns);
        ns.reverse();
        sum2 += extrapolate(&ns);
    }

    println!("p1: {sum}, p2: {sum2}");
}
