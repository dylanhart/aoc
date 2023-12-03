use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");
/*const INPUT: &str =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";*/

fn main() {
    let grid: Vec<&[u8]> = INPUT.lines().map(|line| {
        line.as_bytes()
    }).collect();

    let check_adj = |x: usize, y: usize| -> bool {
        let check_x = |line: &[u8]| -> bool {
            if x > 0 && !line[x-1].is_ascii_digit() && line[x-1] != b'.' {
                return true;
            }
            if !line[x].is_ascii_digit() && line[x] != b'.' {
                return true;
            }
            if x < line.len() - 1 && !line[x+1].is_ascii_digit() && line[x+1] != b'.' {
                return true;
            }
            false
        };

        if y > 0 && check_x(grid[y-1]) {
            return true;
        }
        if check_x(grid[y]) {
            return true;
        }
        if y < grid.len() - 1 && check_x(grid[y+1]) {
            return true;
        }
        false
    };

    let mut sum = 0;
    let mut curr_num = 0;
    let mut adj = false;

    let mut adj_gears = HashSet::new();
    let mut gear_scores = HashMap::new();

    let check_gears = |gears: &mut HashSet<_>, x: usize, yy: usize| {
        let mut check_x = |y: usize| {
            let line = grid[y];
            if x > 0 && line[x-1] == b'*' {
                gears.insert((x-1, y));
            }
            if line[x] == b'*' {
                gears.insert((x, y));
            }
            if x < line.len() - 1 && line[x+1] == b'*' {
                gears.insert((x+1, y));
            }
        };

        if yy > 0 {
            check_x(yy-1);
        }
        check_x(yy);
        if yy < grid.len() - 1 {
            check_x(yy+1);
        }
    };

    for (y, line) in grid.iter().copied().enumerate() {
        for (x, char) in line.iter().copied().enumerate() {
            if char.is_ascii_digit() {
                adj = adj || check_adj(x, y);
                curr_num = (10 * curr_num) + (char - b'0') as i64;
                check_gears(&mut adj_gears, x, y);
            } else {
                if adj {
                    sum += curr_num;
                    for gear in adj_gears.iter() {
                        gear_scores.entry(*gear).or_insert_with(Vec::new).push(curr_num);
                    }
                }
                curr_num = 0;
                adj = false;
                adj_gears.clear();
            }
        }
        if adj {
            sum += curr_num;
            for gear in adj_gears.iter() {
                gear_scores.entry(gear.clone()).or_insert_with(Vec::new).push(curr_num);
            }
        }
        curr_num = 0;
        adj = false;
        adj_gears.clear();
    }

    println!("sum: {sum}");

    let mut gear_score: i64 = 0;
    for (pos, scores) in gear_scores.iter() {
        if scores.len() == 2 {
            println!("{pos:?}: {scores:?}");
            gear_score = gear_score.checked_add(scores[0].checked_mul(scores[1]).unwrap()).unwrap();
        }
    }
    println!("score: {gear_score}");
}
