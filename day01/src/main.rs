const INPUT: &str = include_str!("input.txt");

fn main() {
    let sum: i64 = INPUT.lines().map(|line| {
        let first = line.chars().find(char::is_ascii_digit).unwrap();
        let last = line.chars().rfind(char::is_ascii_digit).unwrap();

        let first = ((first as u8) - b'0') as i64;
        let last = ((last as u8) - b'0') as i64;

        (10 * first) + last
    }).sum();

    println!("p1 sum: {sum}");

    const DIGITS: &[&[u8]] = &[
        b"zero",
        b"one",
        b"two",
        b"three",
        b"four",
        b"five",
        b"six",
        b"seven",
        b"eight",
        b"nine",
    ];

    let sum: i64 = INPUT.lines().map(|line| {
        let mut bytes = line.as_bytes();
        let first = loop {
            if bytes[0].is_ascii_digit() {
                break (bytes[0] - b'0') as i64;
            } else if let Some((i, d)) = DIGITS.into_iter().enumerate().find(|(i, d)| bytes.starts_with(d)) {
                break i as i64;
            }
            bytes = &bytes[1..];
        };

        let mut bytes = line.as_bytes();
        let last = loop {
            if bytes.last().unwrap().is_ascii_digit() {
                break (bytes.last().unwrap() - b'0') as i64;
            } else if let Some((i, d)) = DIGITS.into_iter().enumerate().find(|(i, d)| bytes.ends_with(d)) {
                break i as i64;
            }
            bytes = &bytes[..bytes.len() - 1];
        };

        (10 * first) + last
    }).sum();

    println!("p2 sum: {sum}");
}
