pub use super::cards::*;
use crate::schema::card_types::{Card, Rank, Suit};
use std::io;
use strum::IntoEnumIterator;

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

// pub fn bid_on_trump_suit(my_cards: &mut Vec<Card>) -> Suit {
//     let mut num_of_powerful_cards = 0;

//     for card in my_cards {
//         match card.rank {
//             Rank::Ace => num_of_powerful_cards += 1,
//             Rank::King => num_of_powerful_cards += 1,

//         }
//     }
// }
