use std::cmp::PartialEq;
use std::fmt;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
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

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum PlayerNum {
    P1,
    P2,
    P3,
    P4,
}

impl fmt::Display for PlayerNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub is_trump: bool,
    pub is_mine: bool,
    pub in_player_hands: bool,
}

#[derive(Debug)]
pub struct TricksBid {
    pub player_num: PlayerNum,
    pub tricks_bid: u8,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
            is_trump: false,
            in_player_hands: true,
            is_mine: false,
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
        if self.is_trump {
            return 100;
        }

        match self.rank {
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
        }
    }
}
