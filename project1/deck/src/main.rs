#[allow(dead_code)]
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["2", "3", "4"];
    //let deck = Deck { cards: Vec::new() };
    let deck = Deck { cards: vec![] };

    let cards = vec![];
    for suit in suits {
        for value in values {
            let card = format!("{value} of {suit}");
            cards.push(card);
        }
    }

    //println!("Heres your deck: {}", deck);
    println!("Heres your deck: {deck:?}");
}
