use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Copy, Clone, EnumIter)]
enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
    is_trump: bool,
    is_mine: bool,
    in_player_hands: bool,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
            is_trump: false,
            in_player_hands: true,
            is_mine: false,
        }
    }

    fn set_as_mine(&mut self) {
        self.is_mine = true
    }

    fn set_as_trump(&mut self) {
        self.is_trump = true
    }

    fn get_ranking(&self) -> u8 {
        if self.is_trump {
            return 100;
        }

        match self.rank {
            Rank::Ace => 14,
            Rank::King => 13,
            Rank::Queen => 12,
            Rank::Jack => 11,
            Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
        }
    }
}

fn get_cards() -> Vec<Card> {
    let mut cards = Vec::<Card>::new();

    for rank in Rank::iter() {
        for suit in Suit::iter() {
            cards.push(Card::new(suit, rank));
        }
    }

    cards
}

fn main() {
    let mut cards = get_cards();

    println!("{:#?}", cards);
}
