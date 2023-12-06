use std::ops::Range;

const INPUT: &str = include_str!("input.txt");

fn solve(time: usize, record: usize) -> Range<usize> {
    // record = -x*x + time*x
    let time = time as f64;
    let record = record as f64;

    let b2_4ac = ((time * time) - (4.0 * record)).sqrt();

    let lower = (time - b2_4ac) / 2.0;
    let lower = (lower + 1.0).floor() as usize;

    let upper = (time + b2_4ac) / 2.0;
    let upper = (upper - 1.0).ceil() as usize;

    lower..(upper + 1)
}

fn main() {
    let mut lines = INPUT.lines();
    let times: Vec<usize> = lines.next().unwrap()
        .split_whitespace().skip(1)
        .map(|n| n.parse().unwrap())
        .collect();
    let dists: Vec<usize> = lines.next().unwrap()
        .split_whitespace().skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    let product: usize = times.iter().zip(dists.iter())
        .map(|(&time, &dist)| solve(time, dist).len())
        .product();

    println!("p1: {product}");

    let mut lines = INPUT.lines();
    let time: usize = lines.next().unwrap()
        .split_whitespace().skip(1)
        .collect::<String>()
        .parse().unwrap();
    let dist: usize = lines.next().unwrap()
        .split_whitespace().skip(1)
        .collect::<String>()
        .parse().unwrap();

    println!("p2: {}", solve(time, dist).len());
}
