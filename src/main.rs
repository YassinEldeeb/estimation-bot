mod card;
mod info;
mod utils;

fn main() {
    let mut cards = utils::get_cards();
    let mut my_cards = utils::get_my_cards(&mut cards);
    utils::set_trump_suit(&mut cards, &mut my_cards);
    // let played_cards = utils::get_played_cards(&mut cards);
    let estimated_bids = utils::estimate_num_of_bids(&mut my_cards);

    println!("Estimated Bids: {:#?}", estimated_bids);
    // println!("All Cards: {:#?}", cards);
    // println!("My Cards {:#?}", my_cards);
}
