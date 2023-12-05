use std::cmp::{min, Ordering};
use std::fmt::{Debug, Formatter};
use std::mem::replace;
use std::ops::Range;
use std::str::Lines;

const INPUT: &str = include_str!("input.txt");
/*
const INPUT: &str =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
// */

#[derive(Copy, Clone, Default)]
struct Entry {
    dest: usize,
    src: usize,
    count: usize,
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} -> {:?})", self.src_range(), self.dest_range())
    }
}

impl Entry {
    fn src_range(&self) -> Range<usize> {
        self.src..(self.src + self.count)
    }

    fn dest_range(&self) -> Range<usize> {
        self.dest..(self.dest + self.count)
    }

    fn lookup(&self, val: usize) -> Option<usize> {
        self.src_range().contains(&val).then(|| {
            self.dest + (val - self.src)
        })
    }

    fn intersect(&self, src: &mut Range<usize>) -> Option<Range<usize>> {
        if src.is_empty() { return None; }

        if src.start >= self.src {
            if src.start >= self.src_range().end {
                return None;
            }
            let start = replace(&mut src.start, self.src_range().end);
            let end = min(self.src_range().end, src.end);

            let start = self.dest + (start - self.src);
            let end = self.dest + (end - self.src);

            Some(start..end)
        } else if src.end >= self.src {
            if src.end > self.src_range().end {
                panic!("oh no it doubled")
            }
            let start = self.src;
            let end = replace(&mut src.end, self.src);

            let start = self.dest + (start - self.src);
            let end = self.dest + (end - self.src);

            Some(start..end)
        } else {
            None
        }
    }
}

impl PartialEq<Self> for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src
    }
}

impl PartialOrd<Self> for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.src.partial_cmp(&other.src)
    }
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.src.cmp(&other.src)
    }
}

fn main() {
    let mut lines = INPUT.lines();

    let seeds: Vec<usize> = lines.next().unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();

    let read_map = |mut lines: &mut Lines<'_>| -> Vec<Entry> {
        let mut map: Vec<Entry> = lines
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut nums = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());
                Entry {
                    dest: nums.next().unwrap(),
                    src: nums.next().unwrap(),
                    count: nums.next().unwrap(),
                }
            }).collect();
        map.sort();
        map
    };

    let mut maps = Vec::new();

    assert_eq!(lines.next().unwrap(), "");
    while let Some(header) = lines.next() {
        assert!(header.ends_with(":"));
        maps.push(read_map(&mut lines));
    }

    assert_eq!(lines.count(), 0);

    let mut locs = Vec::new();
    for seed in seeds.iter().copied() {
        print!("{seed}");

        let mut n = seed;
        for map in &maps {
            n = map.iter().find_map(|entry| entry.lookup(n)).unwrap_or(n);
            print!(" -> {n}");
        }
        println!();

        locs.push(n);
    }

    let min = locs.iter().copied().min().unwrap();
    println!("p1: {min}");

    let mut ranges: Vec<Range<usize>> = seeds.chunks(2)
        .map(|data| { data[0]..(data[0]+data[1]) })
        .collect();
    ranges.sort_by_key(|r| r.start);
    println!("seeds: {ranges:?}");
    for (i, map) in maps.iter().enumerate() {
        println!("map: {map:?}");
        ranges = ranges.into_iter()
            .flat_map(|mut r| {
                let mut ranges = Vec::new();
                for e in map {
                    if let Some(mapped) = e.intersect(&mut r) {
                        ranges.push(mapped)
                    } else if r.is_empty() || e.src >= r.end {
                        break;
                    }
                }
                if !r.is_empty() {
                    ranges.push(r);
                }
                ranges
            })
            .collect();
        ranges.sort_by_key(|r| r.start);
        println!("{i}: {ranges:?}");
    }
    let min =  ranges.first().unwrap().start;
    println!("p2: {min}");
}
