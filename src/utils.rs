use crate::card::{Card, Rank, Suit};
use std::io;
use strum::IntoEnumIterator;

pub fn get_cards_from_input(cards: &Vec<Card>) -> Vec<Card> {
    // Shorcuts for user input
    let suit_shortcuts: Vec<&str> = Vec::from(["c", "d", "h", "s"]);
    let rank_shortcuts: Vec<&str> = Vec::from([
        "a", "k", "q", "j", "10", "9", "8", "7", "6", "5", "4", "3", "2",
    ]);

    let mut input = String::new();

    println!("Example: A-C, 2-D, 10-H, J-S");
    // Sample Input: A-C, 2-D, 10-H
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

pub fn estimate_num_of_bids(my_cards: &mut Vec<Card>) -> i32 {
    let mut bids_num = 0;
    let mut num_of_trumps = 0;

    for card in my_cards {
        match card.rank {
            Rank::Ace => bids_num += 1,
            Rank::King => bids_num += 1,
            _ => {}
        }

        if card.is_trump {
            num_of_trumps += 1
        }
    }

    if num_of_trumps - 3 >= 2 {
        bids_num += num_of_trumps - 3
    }

    bids_num
}

pub fn set_trump_suit(cards: &mut Vec<Card>, my_cards: &mut Vec<Card>) {
    let suit_shortcuts: Vec<&str> = Vec::from(["c", "d", "h", "s"]);

    println!("Example: D");
    println!("Enter trump suit:");

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

pub fn get_cards() -> Vec<Card> {
    let mut cards = Vec::<Card>::new();

    for rank in Rank::iter() {
        for suit in Suit::iter() {
            cards.push(Card::new(suit, rank));
        }
    }

    cards
}
