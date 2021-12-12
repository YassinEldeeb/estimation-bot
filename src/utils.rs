use crate::types::{Card, PlayerNum, Rank, Suit, TricksBid};
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

pub fn get_players_bids(my_player_num: PlayerNum) -> Vec<TricksBid> {
    let possible_player_input = Vec::from(["p1", "p2", "p3", "p4"]);

    let mut players_bids: Vec<TricksBid> = Vec::new();
    let mut already_bidded_players = String::new();

    while players_bids.len() < 4 {
        let mut input = String::new();

        if already_bidded_players.len() == 0 {
            println!("What did other players bid on?");
            println!("Example: P1:5, P2:2, P3:4");
        } else {
            let mut diff: Vec<&str> = Vec::new();
            for i in &possible_player_input {
                // Checking if the player hasn't bidded it
                // and It's not me (the bot)
                if !already_bidded_players.contains(i)
                    && my_player_num.to_string().to_lowercase() != String::from(*i)
                {
                    diff.push(i);
                }
            }

            println!("What did {} bid on?", diff.join(", "));
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get your Input!");

        let formatted: Vec<String> = input
            .split(',')
            .map(|e| e.to_lowercase().trim().to_owned())
            .collect();

        if formatted[0].len() == 0 {
            println!("Oh, so I'm the first one. Let me think..");
            // Estimate the bids here and push it to the array
            // Print the estimation for other players
            continue;
        }

        for i in formatted {
            let player_bid: Vec<&str> = i.split(":").collect();
            let tricks_bid: u8 = player_bid[1]
                .trim()
                .parse()
                .expect("Tricks number isn't valid number");
            let player_name_input = player_bid[0].trim();

            let mut player_num = PlayerNum::P4;

            let index = possible_player_input
                .iter()
                .position(|&r| r == player_name_input)
                .unwrap();

            for (i, num) in PlayerNum::iter().enumerate() {
                if i == index {
                    player_num = num
                }
            }

            let player_bid_index = already_bidded_players.contains(player_name_input);

            if player_bid_index {
                println!("{:?} have already bidded before", player_num);
                continue;
            }

            already_bidded_players.push_str(player_name_input);
            if already_bidded_players.len() != 0 {
                already_bidded_players.push_str(", ");
            }
            players_bids.push(TricksBid {
                player_num,
                tricks_bid,
            })
        }
    }

    players_bids
}

pub fn get_player_num() -> PlayerNum {
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

    let mut player_num = PlayerNum::P4;

    for (i, value) in PlayerNum::iter().enumerate() {
        if i == index {
            player_num = value
        }
    }

    player_num
}

pub fn get_dealer_player_num() -> PlayerNum {
    println!("Enter the dealer position");
    println!("Example: P3");
    get_player_num()
}

pub fn get_bot_player_num() -> PlayerNum {
    println!("Enter my position");
    println!("Example: P1");
    get_player_num()
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

    // How many bids can we win if we have x amount of trump suited cards
    // bids_num += ???

    bids_num
}

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

pub fn get_cards() -> Vec<Card> {
    let mut cards = Vec::<Card>::new();

    for rank in Rank::iter() {
        for suit in Suit::iter() {
            cards.push(Card::new(suit, rank));
        }
    }

    cards
}
