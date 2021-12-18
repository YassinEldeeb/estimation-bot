pub use super::cards::*;
use crate::schema::card_types::{Card, Suit};
use std::io;
use strum::IntoEnumIterator;

pub fn set_trump_suit(cards: &mut Vec<Card>, my_cards: &mut Vec<Card>) {
    let suit_shortcuts: Vec<&str> = Vec::from(["c", "d", "h", "s", "sans"]);

    let bot_bid = bid_on_trump_suit(my_cards);

    match bot_bid {
        Some(value) => {
            println!("My Bid is: {:?}", value);
        }
        None => {
            println!("Trump: Pass");
        }
    }
    println!("Enter trump suit:");
    println!("Example: D");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get your Input!");

    let input = input.trim().to_lowercase();
    if input == "sans" {
        println!("No Trump Suit in this Round!");
        return;
    }

    let index = suit_shortcuts.iter().position(|&r| r == input).unwrap();

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
// Sample: 4-H,3-H,J-H,8-H,5-H,7-D,J-C,9-C,K-C,4-C,Q-C,3-S,Q-S
pub fn bid_on_trump_suit(my_cards: &mut Vec<Card>) -> Option<Suit> {
    // Get:
    //  1- num of cards in each suit
    //  2- num of special cards in each suit
    //  3- return the suit with the most cards & special cards num
    //  4- If there're values that are equal return None
    //  5- If the cards num is more than or equal 4 is considered a trump suit
    struct SuitInfo {
        trump_suit: Suit,
        num_of_cards: u8,
        num_of_special_cards: u8,
        none: bool,
    }
    let mut winning_trump_info = SuitInfo {
        trump_suit: Suit::Clubs,
        num_of_cards: 0,
        num_of_special_cards: 0,
        none: true,
    };

    let mut trump_infos: Vec<SuitInfo> = vec![];

    for suit in Suit::iter() {
        let mut num_of_cards = 0;
        let mut num_of_special_cards = 0;

        for card in &mut *my_cards {
            if card.suit == suit {
                num_of_cards += 1;
                if card.is_special_suit {
                    num_of_special_cards += 1;
                }
            }
        }

        trump_infos.push(SuitInfo {
            trump_suit: suit,
            num_of_cards,
            num_of_special_cards,
            none: false,
        });
        if num_of_cards >= 5
            && num_of_cards > winning_trump_info.num_of_cards
            && if num_of_cards < 5 {
                num_of_special_cards >= winning_trump_info.num_of_special_cards
            } else {
                true
            }
        {
            winning_trump_info = SuitInfo {
                trump_suit: suit,
                num_of_cards,
                num_of_special_cards,
                none: false,
            };
        }
    }

    if winning_trump_info.none {
        None
    } else {
        Some(winning_trump_info.trump_suit)
    }
}
