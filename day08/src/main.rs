use std::collections::{BTreeMap, HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");
/* const INPUT: &str =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
// */

#[derive(Copy, Clone)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn new(left: &'a str, right: &'a str) -> Self {
        Self { left, right }
    }
}

#[derive(Default)]
struct Tree<'a> {
    map: BTreeMap<&'a str, Node<'a>>,
}

impl<'a> Tree<'a> {
    fn add_row(&mut self, line: &'a str) {
        let loc = &line[..line.find(" = ").unwrap()];
        let left = &line[line.find('(').unwrap() + 1..line.find(',').unwrap()];
        let right = &line[line.find(", ").unwrap() + 2..line.find(')').unwrap()];

        self.map.insert(loc, Node::new(left, right));
    }
}

struct CycleInfo(usize, usize, Vec<usize>);

impl CycleInfo {
    fn valid_offsets(&self) -> impl Iterator<Item=usize> + '_ {
        let mut cycle_num = 0;
        let mut offset_iter = self.2.iter().copied();
        core::iter::from_fn(move || -> Option<usize> {
            offset_iter.next().or_else(|| {
                offset_iter = self.2.iter().copied();
                cycle_num += 1;
                offset_iter.next()
            }).map(|offset| cycle_num * self.1 + offset)
        })
    }
}

fn main() {
    let mut lines = INPUT.lines();

    let path = lines.next().unwrap();
    let path_len = path.chars().count();
    assert!(lines.next().unwrap().is_empty());

    let mut tree = Tree::default();
    for line in lines {
        tree.add_row(line);
    }

    let mut loc = "AAA";
    let mut steps: usize = 0;
    let mut next_step = path.chars().cycle();

    while loc != "ZZZ" {
        let node = tree.map.get(loc).expect(loc);
        loc = match next_step.next().unwrap() {
            'L' => node.left,
            'R' => node.right,
            _ => panic!("Invalid direction"),
        };
        steps += 1;
    }

    println!("p1: {steps}");

    let mut ghosts: Vec<&str> = tree.map.keys().copied()
        .filter(|&loc| loc.ends_with('A'))
        .collect();

    let find_cycle = |start: &str| {
        let mut seen = HashMap::new();
        let mut loc = start;
        let mut steps: usize = 0;
        let mut next_step = path.chars().enumerate().cycle();

        let mut valid_offsets = Vec::new();

        loop {
            let (idx, dir) = next_step.next().unwrap();
            if let Some(first) = seen.insert((loc, idx), steps) {
                break CycleInfo(first, steps - first, valid_offsets);
            }
            if loc.ends_with('Z') {
                valid_offsets.push(steps);
            }
            let node = tree.map.get(loc).expect(loc);
            loc = match dir {
                'L' => node.left,
                'R' => node.right,
                _ => panic!("Invalid direction"),
            };
            steps += 1;
        }
    };
    let cycles: Vec<_> = ghosts.iter().copied()
        .map(find_cycle)
        .collect();

    let mut iters: Vec<_> = cycles.iter()
        .map(|cycle| cycle.valid_offsets().peekable())
        .collect();

    for cycle in &cycles {
        println!("start: {}, len: {}, offsets: {:?}", cycle.0, cycle.1, &cycle.2);
    }

    let steps = {
        // (position, dist_to_next_position)
        let mut expanded: Vec<_> = cycles.iter().map(|cycle| {
            assert!(cycle.2.len() == 1); // only works in this case
            (cycle.2[0], cycle.1)
        }).collect();

        while expanded.len() > 1 {
            // It's way faster if we start from the lowest values due to the stupid looping below
            expanded.sort();
            // find the matching position for the first two
            while expanded[0].0 != expanded[1].0 {
                // stupid loops to push the values up
                while expanded[0].0 < expanded[1].0 {
                    expanded[0].0 += expanded[0].1;
                }
                while expanded[1].0 < expanded[0].0 {
                    expanded[1].0 += expanded[1].1;
                }
            }
            // dist_to_next_position is now the current position to represent a combined cycle
            expanded[0].1 = expanded[0].0;
            // remove the second value since we combined it above
            expanded.swap_remove(1);
        }
        expanded[0].0;
    };

    println!("p2: {steps}");
}
