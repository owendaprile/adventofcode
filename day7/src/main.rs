mod camel_cards;

use camel_cards::*;

fn make_game(contents: &String) -> Vec<(Hand, u32)> {
    let mut game = vec![];

    for line in contents.lines() {
        let mut line = line.split_whitespace();

        let hand_str = Hand::from_str(line.next().unwrap());
        let bid = line.next().unwrap().parse::<u32>().unwrap();

        game.push((hand_str, bid));
    }

    game
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(filename).unwrap();

    let mut game = make_game(&contents);

    game.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let result: u64 = game
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u64 * hand.1 as u64)
        .sum();

    println!("Result: {:?}", result);
}
