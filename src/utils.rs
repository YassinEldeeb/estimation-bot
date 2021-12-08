use crate::card::{Card, Rank, Suit};
use std::io;
use strum::IntoEnumIterator;

pub fn get_my_cards(cards: &mut Vec<Card>) -> Vec<Card> {
    // Shorcuts for user input
    let suit_shortcuts: Vec<&str> = Vec::from(["c", "d", "h", "s"]);
    let rank_shortcuts: Vec<&str> = Vec::from([
        "a", "k", "q", "j", "10", "9", "8", "7", "6", "5", "4", "3", "2",
    ]);

    let mut input = String::new();

    println!("Example: A-C, 2-D, 10-H, J-S");
    println!("Enter your cards:");

    // Sample Input: A-C, 2-D, 10-H
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get your Input!");

    // Get a vector of the cards info.
    let formatted: Vec<String> = input
        .split(',')
        .map(|e| e.to_lowercase().trim().to_owned())
        .collect();

    let mut my_cards: Vec<Card> = Vec::new();

    for input in &formatted {
        // Placeholders for actual values
        let mut rank = Rank::Ace;
        let mut suit = Suit::Clubs;

        for (i, e) in input.split("-").enumerate() {
            // Checking if It's the first [*rank*, suit]
            if i % 2 == 0 {
                let index = rank_shortcuts.iter().position(|&r| r == e).unwrap();

                for (i, r) in Rank::iter().enumerate() {
                    if i == index {
                        rank = r
                    }
                }
            } else {
                let index = suit_shortcuts.iter().position(|&r| r == e).unwrap();

                for (i, s) in Suit::iter().enumerate() {
                    if i == index {
                        suit = s
                    }
                }
            }
        }

        let cards_clone = cards.clone();
        let index = cards_clone
            .clone()
            .iter()
            .position(|&r| r.rank == rank && r.suit == suit)
            .unwrap();

        cards[index].set_as_mine();
        my_cards.push(cards[index]);
    }

    my_cards
}

pub fn get_cards() -> Vec<Card> {
    let mut cards = Vec::<Card>::new();

    for rank in Rank::iter() {
        for suit in Suit::iter() {
            cards.push(Card::new(suit, rank));
        }
    }

    cards
}
