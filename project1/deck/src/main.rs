use rand::{seq::SliceRandom, thread_rng};

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

        //let deck = Deck { cards };
        // "return deck;" is valid, but being an expression base language, a line with the value at the end is enough
        //deck
        // Return the type constructor with the value is even better!
        Deck { cards }
    }

    // shuffle is a method. Its first paramenter is a value of the type. It operates on a instance.
    // Its is associated with an instance.
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    //The line below is valid, but is better to prefix the function with the module name when the function is external.
    //let _rng = thread_rng();
    let _rng = rand::thread_rng();
    let mut deck = Deck::new();
    println!("Heres your deck: {deck:#?}");
    //let deck = deck.shuffle();
    deck.shuffle();
    println!("Heres your deck: {deck:#?}");
}
