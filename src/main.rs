mod card;
mod utils;

fn main() {
    let mut cards = utils::get_cards();
    let mut my_cards = utils::get_my_cards(&mut cards);
    utils::set_trump_suit(&mut cards, &mut my_cards);
    utils::set_played_cards(&mut cards);

    println!("All Cards: {:#?}", cards);
    println!("My Cards {:#?}", my_cards);
}
