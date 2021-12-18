mod info;
mod schema;
mod utils;
use utils::{bids, game_info, gameplay};

// !Learn: lifetimes, error handling and the Traits to refactor the codebase

fn main() {
    let mut cards = game_info::get_cards();
    let mut my_cards = gameplay::get_my_cards(&mut cards);
    // let mut my_player_num = game_info::get_my_player_num();
    // let mut dealer_player_num = game_info::get_dealer_player_num();
    // gameplay::set_trump_suit(&mut cards, &mut my_cards);
    // let players_bids = bids::get_players_bids(my_player_num);
    // let played_cards = gameplay::get_played_cards(&mut cards);
    gameplay::set_trump_suit(&mut cards, &mut my_cards);
    let estimated_bids = bids::estimate(&cards, &mut my_cards);

    // println!("Players Bids: {:#?}", players_bids);
    // println!("Dealer Position: {:#?}", dealer_player_num);
    println!("Estimated Bids: {:#?}", estimated_bids);
    // println!("All Cards: {:#?}", cards);
    // println!("My Cards {:#?}", my_cards);
}
