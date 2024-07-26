#[allow(dead_code)]
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // new is an associated function. It belongs to the type. Does not need an instance.
    // In fact, new creates an instance of the type, Deck in this case.
    // Its no associated with an instance.
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = [2, 3, 4];

        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }

        let deck = Deck { cards };
        // "return deck;" is valid, but being an expression base language, a line with the value at the end is enough
        deck
    }

    // shuffle is a method. Its first paramenter is a value of the type. It operates on a instance.
    // Its is associated with an instance.
    fn shuffle(&mut self) {
        ()
    }
}

fn main() {
    let mut deck = Deck::new();
    println!("Heres your deck: {deck:#?}");
    //let deck = deck.shuffle();
    deck.shuffle();
    println!("Heres your deck: {deck:#?}");
}
