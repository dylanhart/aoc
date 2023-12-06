const INPUT: &str = include_str!("input.txt");

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

    let ranges = [
        6..39,
        17..66,
        28..42,
        28..54,
    ];
    let sum: usize = ranges.iter().map(|r| r.len()).inspect(|l| println!("{l}")).product();

    println!("p1: {sum}");

    let mut lines = INPUT.lines();
    let time: usize = lines.next().unwrap()
        .split_whitespace().skip(1)
        .collect::<String>()
        .parse().unwrap();
    let dist: usize = lines.next().unwrap()
        .split_whitespace().skip(1)
        .collect::<String>()
        .parse().unwrap();

    let calc_dist = |hold_time| -> usize {
        (time - hold_time) * hold_time
    };

    let mut search = 5_000_000..40_000_000;
    while calc_dist(search.start) <= dist {
        search.start += 1;
    }
    while calc_dist(search.end - 1) <= dist {
        search.end -= 1;
    }
    println!("{search:?}");

    let diff: usize = search.len();
    println!("p2: {diff}");
}
