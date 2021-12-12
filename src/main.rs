mod info;
mod types;
mod utils;

fn main() {
    let mut cards = utils::get_cards();
    // let mut my_cards = utils::get_my_cards(&mut cards);
    let mut my_player_num = utils::get_bot_player_num();
    // let mut dealer_player_num = utils::get_dealer_player_num();

    // utils::set_trump_suit(&mut cards, &mut my_cards);
    let players_bids = utils::get_players_bids(my_player_num);
    // let played_cards = utils::get_played_cards(&mut cards);
    // let estimated_bids = utils::estimate_num_of_bids(&mut my_cards);

    // println!("Players Bids: {:#?}", players_bids);
    // println!("Dealer Position: {:#?}", dealer_player_num);
    // println!("Estimated Bids: {:#?}", estimated_bids);
    // println!("All Cards: {:#?}", cards);
    // println!("My Cards {:#?}", my_cards);
}
