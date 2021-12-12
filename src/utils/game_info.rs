use crate::schema::{
    card_types::{Card, Rank, Suit},
    player::Num,
};
use std::io;
use strum::IntoEnumIterator;

// Players Positions
pub fn get_player_num() -> Num {
    let possible_player_input = Vec::from(["p1", "p2", "p3", "p4"]);
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get your Input!");

    input = input.trim().to_lowercase();

    let index = possible_player_input
        .iter()
        .position(|&r| r == input)
        .unwrap();

    let mut player_num = Num::P4;

    for (i, value) in Num::iter().enumerate() {
        if i == index {
            player_num = value
        }
    }

    player_num
}

pub fn get_my_player_num() -> Num {
    println!("Enter my position");
    println!("Example: P1");
    get_player_num()
}

pub fn get_dealer_player_num() -> Num {
    println!("Enter the dealer position");
    println!("Example: P3");
    get_player_num()
}

// Cards Info
pub fn get_cards() -> Vec<Card> {
    let mut cards = Vec::<Card>::new();

    for rank in Rank::iter() {
        for suit in Suit::iter() {
            cards.push(Card::new(suit, rank));
        }
    }

    cards
}
