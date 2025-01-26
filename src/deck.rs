use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: VecDeque::new(),
        }
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop_front().unwrap()
    }

    pub fn shuffle(&mut self) {
        let mut cards_vec: Vec<Card> = self.cards.iter().copied().collect();
        let mut rng = rand::thread_rng();
        cards_vec.shuffle(&mut rng);
        self.cards.clear();
        let mut res = VecDeque::from(cards_vec);
        self.cards.append(&mut res);
    }

    pub fn add_to_bottom<I>(&mut self, cards: I)
    where
        I: IntoIterator<Item = Card>,
    {
        for item in cards {
            self.cards.push_back(item);
        }
    }
}

pub fn create_deck() -> Deck {
    let mut deck: Deck = Deck::new();
    let suits = vec![Suit::Spades, Suit::Diamonds, Suit::Hearts, Suit::Clubs];
    let mut ranks: Vec<Rank> = (2..9).into_iter().map(|x| Rank::Number(x)).collect();

    ranks.extend(vec![Rank::Jake, Rank::Queen, Rank::King]);
    let cards: Vec<Card> = ranks
        .iter()
        .map(|rank| suits.iter().map(|suit| Card::new(*suit, *rank)))
        .flatten()
        .collect();

    deck.add_to_bottom(cards);
    deck.shuffle();
    deck
}
