#[allow(dead_code)]
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    //let deck = Deck{ cards: vec![] };
    let deck = Deck { cards: Vec::new() };
    //println!("Heres your deck: {}", deck);
    println!("Heres your deck: {deck:?}");
}
