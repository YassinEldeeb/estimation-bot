use crate::info::{MAX_BIDS, NUM_OF_CARDS_IN_EACH_SUIT, NUM_OF_PLAYERS};
use crate::schema::{bid, card_types::*, player};
use std::cmp;
use std::io;
use strum::IntoEnumIterator;

// Sample: 10-S,8-S,7-S,5-S,4-S,J-H,5-H,3-H,8-C,5-C,2-C,Q-D,10-D
pub fn estimate(cards: &Vec<Card>, my_cards: &mut Vec<Card>) -> i8 {
    let mut tricks_bid = 0;
    let mut num_of_trumps = 0_i8;

    let mut winning_trumps = 0;

    for my_card in my_cards {
        match my_card.rank {
            Rank::Ace => {
                if my_card.is_trump {
                    winning_trumps += 1;
                }
                tricks_bid += 1
            }
            Rank::King => {
                if my_card.is_trump {
                    winning_trumps += 1;
                }
                tricks_bid += 1
            }
            _ => {
                // Check if there're higher ranking cards in the same suit
                // Check if I own the higher ranking cards or not
                let mut is_it_a_winning_card = true;

                for card in cards {
                    if card.get_ranking() > my_card.get_ranking()
                        && card.suit == my_card.suit
                        && !card.is_mine
                    {
                        is_it_a_winning_card = false;
                        break;
                    }
                }
                if is_it_a_winning_card {
                    if my_card.is_trump {
                        winning_trumps += 1;
                    }
                    tricks_bid += 1;
                }
            }
        }

        if my_card.is_trump {
            num_of_trumps += 1
        }
    }

    // Add tricks bid for trump suits
    let offset = MAX_BIDS / NUM_OF_PLAYERS;
    tricks_bid += cmp::max(
        if num_of_trumps - winning_trumps - (offset - winning_trumps) + winning_trumps
            > num_of_trumps
        {
            num_of_trumps - winning_trumps
        } else {
            num_of_trumps - winning_trumps - (offset - winning_trumps)
        },
        0,
    );

    tricks_bid
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
