use crate::schema::card_types::*;
use std::io;
use strum::IntoEnumIterator;

pub fn get_cards_from_input(cards: &Vec<Card>) -> Vec<Card> {
    // Shorcuts for user input
    let suit_shortcuts = Vec::from(["c", "d", "h", "s"]);
    let rank_shortcuts = Vec::from([
        "a", "k", "q", "j", "10", "9", "8", "7", "6", "5", "4", "3", "2",
    ]);

    let mut input = String::new();

    println!("Example: A-C, 2-D, 10-H, J-S");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get your Input!");

    // Get a vector of the cards info.
    let formatted: Vec<String> = input
        .split(',')
        .map(|e| e.to_lowercase().trim().to_owned())
        .collect();

    let mut cards_from_input: Vec<Card> = Vec::new();

    for input in formatted {
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

        let index = cards
            .iter()
            .position(|r| r.rank == rank && r.suit == suit)
            .unwrap();

        cards_from_input.push(cards[index]);
    }

    cards_from_input
}

pub fn get_my_cards(cards: &mut Vec<Card>) -> Vec<Card> {
    println!("Enter your cards:");

    let mut my_cards = get_cards_from_input(cards);

    for card in cards {
        for my_card in &mut my_cards {
            if card.suit == my_card.suit && card.rank == my_card.rank {
                card.set_as_mine();
            }
        }
    }

    my_cards
}

pub fn get_played_cards(cards: &mut Vec<Card>) -> Vec<Card> {
    println!("Enter played cards:");

    let mut played_cards = get_cards_from_input(cards);

    for card in cards {
        for card in &mut played_cards {
            if card.suit == card.suit && card.rank == card.rank {
                card.set_as_played();
            }
        }
    }

    played_cards
}
