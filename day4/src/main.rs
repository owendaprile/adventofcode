// Line by line... AGAIN!
// Load the card numbers, then the winning numbers
// do the calculation

#[derive(Debug, Clone)]
struct Card {
    id: u64,
    numbers: Vec<u64>,
    winning_numbers: Vec<u64>,
}

//fn calculate_winning_amount(cards: Vec<Card>) -> u64 {
//    let mut total = 0;
//
//    for card in cards {
//        let mut matches = 0;
//
//        for number in card.numbers {
//            if card.winning_numbers.contains(&number) {
//                matches += 1;
//            }
//        }
//
//        if matches > 0 {
//            total += u64::pow(2, matches - 1);
//        }
//    }
//
//    total
//}

fn calculate_answer(cards: Vec<Card>) -> u64 {
    let mut cards = cards.clone();

    // loop until there are no more cards

    let mut i = 0;
    loop {
        let mut matches = 0;

        if let Some(card) = cards.get(i) {
            for number in &card.numbers {
                if card.winning_numbers.contains(&number) {
                    matches += 1;
                }
            }

            for match_num in card.id..matches + card.id {
                if let Some(card) = cards.get(match_num as usize) {
                    cards.push(card.clone());
                }
            }

        } else {
            break;
        }

        i += 1;
    }

//    for card in &cards {
//        let mut matches = 0;
//
//        for number in &card.numbers {
//            if card.winning_numbers.contains(&number) {
//                matches += 1;
//            }
//        }
//
//        for match_num in card.id..matches + card.id {
//            if let Some(card) = cards.get(match_num as usize) {
//                cards.push(card.clone());
//            }
//        }
//    }

    cards.len() as u64
}

fn main() {
    let filename = std::env::args().nth(1).expect("no filename given");

    let contents = std::fs::read_to_string(filename).unwrap();

    let mut cards: Vec<Card> = vec![];

    for line in contents.lines() {
        let mut line = line.split(":");

        let card_number = line
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let mut line = line.next().unwrap().trim().split("|");

        let mut winning_numbers: Vec<u64> = vec![];
        for num_str in line.next().unwrap().trim().split(" ") {
            if let Ok(num) = num_str.parse::<u64>() {
                winning_numbers.push(num);
            }
        }

        let mut numbers: Vec<u64> = vec![];
        for num_str in line.next().unwrap().trim().split(" ") {
            if let Ok(num) = num_str.parse::<u64>() {
                numbers.push(num);
            }
        }

        let card = Card {
            id: card_number,
            numbers,
            winning_numbers,
        };

        cards.push(card);
    }

    let result = calculate_answer(cards);

    println!("Result: {}", result);
}
