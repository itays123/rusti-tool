#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn suit(&self) -> &Suit {
        &self.suit
    }
    pub fn rank(&self) -> &Rank {
        &self.rank
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // safety: see implementation of partial_cmp above
        self.rank.cmp(&other.rank)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
    Number(usize),
    Jake,
    Queen,
    King,
    Joker,
}

impl Rank {
    pub fn value(&self) -> usize {
        use Rank::*;
        match self {
            Number(x) => *x,
            Jake => 11,
            Queen => 12,
            King => 13,
            Joker => 0,
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // safety: see implementation of partial_cmp above
        self.partial_cmp(other).unwrap()
    }
}
