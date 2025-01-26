#[derive(Debug)]
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

#[derive(Debug)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}
#[derive(Debug)]
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
