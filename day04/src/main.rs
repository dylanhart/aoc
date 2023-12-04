use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let scores: Vec<usize> = INPUT.lines().map(|mut line| {
        line = &line["Card 210: ".len()..];
        let winners = &line[.."20 42 99 64 58 19 11  8 78  2 ".len()];
        let winners: HashSet<i64> = winners.split(" ")
            .filter(|&n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers = &line["20 42 99 64 58 19 11  8 78  2 | ".len()..];
        let count = numbers.split(" ")
            .filter(|&n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .filter(|n| winners.contains(&n))
            .count();
        count
    }).collect();

    let sum: i64 = scores.iter().copied().map(|score| {
        if score > 0 {
            1 << (score - 1)
        } else {
            0
        }
    }).sum();

    println!("p1: {sum}");

    let mut counts = vec![1; scores.len()];
    for i in 0..counts.len() {
        for s in 1..=(scores[i] as usize){
            counts[i + s] += counts[i];
        }
    }

    let sum: usize = counts.iter().sum();

    println!("p2: {sum}");
}
