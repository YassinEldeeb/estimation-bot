use crate::schema::card_types::{Card, Suit};
use std::io;
use strum::IntoEnumIterator;

pub use super::cards::*;

pub fn set_trump_suit(cards: &mut Vec<Card>, my_cards: &mut Vec<Card>) {
    let suit_shortcuts: Vec<&str> = Vec::from(["c", "d", "h", "s"]);

    println!("Enter trump suit:");
    println!("Example: D");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get your Input!");

    let index = suit_shortcuts
        .iter()
        .position(|&r| r == input.trim().to_lowercase())
        .unwrap();

    fn updating_cards(cards: &mut Vec<Card>, s: Suit) {
        for card in &mut *cards {
            if card.suit == s {
                card.set_as_trump();
            }
        }
    }

    for (i, s) in Suit::iter().enumerate() {
        if i == index {
            // Mutating cards & my_cards to set trump suited cards
            updating_cards(cards, s);
            updating_cards(my_cards, s);
        }
    }
}
