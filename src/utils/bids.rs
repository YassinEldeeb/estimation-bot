use crate::schema::{bid, card_types::*, player};
use std::io;
use strum::IntoEnumIterator;

pub fn estimate(my_cards: &mut Vec<Card>) -> i32 {
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

pub fn get_players_bids(my_player_num: player::Num) -> Vec<bid::Tricks> {
    let possible_player_input = Vec::from(["p1", "p2", "p3", "p4"]);

    let mut players_bids: Vec<bid::Tricks> = Vec::new();
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

            let mut player_num = player::Num::P4;

            let index = possible_player_input
                .iter()
                .position(|&r| r == player_name_input)
                .unwrap();

            for (i, num) in player::Num::iter().enumerate() {
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
            players_bids.push(bid::Tricks {
                player_num,
                tricks_bid,
            })
        }
    }

    players_bids
}
