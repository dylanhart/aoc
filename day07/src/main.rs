use std::cmp::Ordering;
use crate::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: [Label; 5]) -> Self {
        assert!(HandType::FiveOfAKind > HandType::HighCard);

        let mut freq = [0usize; 15];
        let mut counts = [0usize; 6];

        for card in cards {
            freq[card.0 as usize] += 1;
            counts[freq[card.0 as usize]] += 1;
        }

        match counts {
            [.., 1] => FiveOfAKind,
            [.., 1, _] => FourOfAKind,
            [.., 2, 1, _, _] => FullHouse,
            [.., 1, _, _] => ThreeOfAKind,
            [.., 2, _, _, _] => TwoPair,
            [.., 1, _, _, _] => OnePair,
            _ => HighCard,
        }
    }

    fn from_jokers(cards: [JokeLabel; 5]) -> Self {
        assert!(HandType::FiveOfAKind > HandType::HighCard);

        let mut freq = [0usize; 15];
        let mut counts = [0usize; 6];
        let mut jokers = 0usize;

        for card in cards {
            if card.is_joker() {
                jokers += 1;
                for freq in freq.iter_mut() {
                    *freq += 1;
                    counts[*freq] += 1;
                }
            } else {
                let freq = &mut freq[card.0 as usize];
                *freq += 1;
                counts[*freq] += 1;
            }
        }

        match (counts, jokers) {
            ([.., n], _) if n > 0 => FiveOfAKind,
            ([.., n, _], _)  if n > 0=> FourOfAKind,

            ([.., 2, 1, _, _], 0) => FullHouse,
            ([.., 2, _, _], 1) => FullHouse, // JAABB
            ([.., 2, 1, 1, _], 1) => FullHouse, // JABBB
            // More jokers will match above

            ([.., n, _, _], _) if n > 0 => ThreeOfAKind, // JJABC
            ([.., 2, _, _, _], 0) => TwoPair,
            ([.., 1, _, _, _], 0) => OnePair,
            ([.., 4, _, _, _], 1) => OnePair, // JABCD
            _ => HighCard,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Label(u8);

impl TryFrom<u8> for Label {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            n @ b'2'..=b'9' => Ok(Label(n - b'0')),
            b'T' => Ok(Label(10)),
            b'J' => Ok(Label(11)),
            b'Q' => Ok(Label(12)),
            b'K' => Ok(Label(13)),
            b'A' => Ok(Label(14)),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Hand(HandType, [Label; 5]);

impl Hand {
    fn parse(hand: &str) -> Option<Hand> {
        if hand.len() != 5 {
            return None;
        }

        let labels = hand.bytes()
            .map(Label::try_from)
            .try_fold(Vec::new(), |mut vec, label| {
                vec.push(label.ok()?);
                Some(vec)
            })?
            .try_into().ok()?;

        Some(Self(HandType::from_cards(labels), labels))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.cmp(&other.0) {
            Ordering::Equal => self.1.partial_cmp(&other.1),
            ord => Some(ord),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => self.1.cmp(&other.1),
            ord => ord,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
struct Game(Hand, i64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct JokeLabel(u8);

impl JokeLabel {
    fn is_joker(&self) -> bool {
        self.0 == 1
    }
}

impl From<Label> for JokeLabel {
    fn from(value: Label) -> Self {
        let new_value = if value.0 == 11 {
            1
        } else {
            value.0
        };

        JokeLabel(new_value)
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct JokeHand(HandType, [JokeLabel; 5]);

impl JokeHand {
    fn parse(hand: &str) -> Option<JokeHand> {
        if hand.len() != 5 {
            return None;
        }

        let labels = hand.bytes()
            .map(Label::try_from)
            .try_fold(Vec::new(), |mut vec, label| {
                vec.push(JokeLabel::from(label.ok()?));
                Some(vec)
            })?
            .try_into().ok()?;

        Some(Self(HandType::from_jokers(labels), labels))
    }
}

impl PartialOrd for JokeHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.cmp(&other.0) {
            Ordering::Equal => self.1.partial_cmp(&other.1),
            ord => Some(ord),
        }
    }
}

impl Ord for JokeHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => self.1.cmp(&other.1),
            ord => ord,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
struct JokeGame(JokeHand, i64);



fn main() {
    let mut lines = INPUT.lines();

    let mut games: Vec<Game> = INPUT.lines().map(|line| {
        let mut split = line.split_whitespace();
        let hand = split.next().and_then(Hand::parse).unwrap();
        let bid = split.next().and_then(|b| b.parse().ok()).unwrap();
        Game(hand, bid)
    }).collect();
    games.sort_by_key(|g| g.0);

    let sum: i64 = games.iter().enumerate().map(|(idx, &Game(_hand, bid))| {
        (idx as i64 + 1) * bid
    }).sum();

    println!("p1: {sum}");

    let mut games: Vec<JokeGame> = INPUT.lines().map(|line| {
        let mut split = line.split_whitespace();
        let hand = split.next().and_then(JokeHand::parse).unwrap();
        let bid = split.next().and_then(|b| b.parse().ok()).unwrap();
        JokeGame(hand, bid)
    }).collect();
    games.sort_by_key(|g| g.0);

    let sum: i64 = games.iter().enumerate().map(|(idx, &JokeGame(_hand, bid))| {
        (idx as i64 + 1) * bid
    }).sum();

    println!("p2: {sum}");
}
