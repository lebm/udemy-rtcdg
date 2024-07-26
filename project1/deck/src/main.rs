#[allow(dead_code)]
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    //let values = ["2", "3", "4"];
    let values = [2, 3, 4];
    //let deck = Deck { cards: Vec::new() };

    let mut cards = vec![];
    for suit in suits {
        for value in values {
            let card = format!("{value} of {suit}");
            cards.push(card);
        }
    }

    // Equivalent to  let deck = Deck { cards: cards };
    let deck = Deck { cards };

    //println!("Heres your deck: {}", deck);
    //println!("Heres your deck: {deck:?}");
    println!("Heres your deck: {deck:#?}");
}
