mod card;
mod utils;

fn main() {
    let mut cards = utils::get_cards();
    let my_cards = utils::get_my_cards(&mut cards);

    println!("My Cards: {:#?}", cards);
    println!("All Cards {:#?}", my_cards);
}
