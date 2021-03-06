use std::cmp::PartialEq;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
pub enum Trump {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    NotFound,
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub is_trump: bool,
    pub is_mine: bool,
    pub in_player_hands: bool,
    pub is_special_suit: bool,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        let is_special_suit =
            if rank == Rank::Ace || rank == Rank::King || rank == Rank::Queen || rank == Rank::Jack
            {
                true
            } else {
                false
            };

        Card {
            suit,
            rank,
            is_trump: false,
            in_player_hands: true,
            is_mine: false,
            is_special_suit,
        }
    }

    pub fn set_as_mine(&mut self) {
        self.is_mine = true
    }

    pub fn set_as_trump(&mut self) {
        self.is_trump = true
    }

    pub fn set_as_played(&mut self) {
        self.in_player_hands = false
    }

    pub fn get_ranking(&self) -> u8 {
        let mut ranking;

        ranking = match self.rank {
            Rank::Ace => 14,
            Rank::King => 13,
            Rank::Queen => 12,
            Rank::Jack => 11,
            Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
        };

        if self.is_trump {
            ranking += 10;
        }

        ranking
    }
}
