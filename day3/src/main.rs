use std::collections::HashSet;

use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Entry {
    Number(Number),
    Gear,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Number {
    index: usize,
    value: u64,
}

fn find_neighbors(map: &Vec<Vec<Entry>>, x: usize, y: usize) -> Option<HashSet<Number>> {
    let mut neighbors: HashSet<Number> = HashSet::new();

    let x = x as u64;
    let y = y as u64;

    for i in x.saturating_sub(1)..x + 2 {
        for j in y.saturating_sub(1)..y + 2 {
            if i == x && j == y {
                continue;
            }

            if let Some(r) = map.get(i as usize) {
                if let Some(Entry::Number(number)) = r.get(j as usize) {
                    neighbors.insert(number.clone());
                }
            }
        }
    }

    match neighbors.len() {
        2 => Some(neighbors),
        _ => None,
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Entry>>,
}

impl Map {
    fn new() -> Self {
        Map { map: vec![] }
    }

    fn set_position(&mut self, x: usize, y: usize, entry: Entry) {
        if x >= self.map.len() {
            self.map.resize(x + 1, vec![]);
        }

        let row = self.map.get_mut(x).unwrap();

        if y >= row.len() {
            row.resize(y + 1, Entry::Empty);
        }

        row[y] = entry;
    }

    fn calculate_sum_of_symbol_neighbors(&self) -> u64 {
        let mut found: HashSet<(Number, Number)> = HashSet::new();

        for (i, row) in self.map.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                match val {
                    Entry::Gear => {
                        if let Some(neighbors) = find_neighbors(&self.map, i, j) {
                            let mut neighbors = neighbors.iter();

                            let tup = (
                                neighbors.next().unwrap().clone(),
                                neighbors.next().unwrap().clone(),
                            );

                            found.insert(tup);
                        }
                    }
                    _ => (),
                }
            }
        }

        found
            .iter()
            .map(|element| element.0.value * element.1.value)
            .sum()
    }
}

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("error: filename not provided");

    let contents = std::fs::read_to_string(file_name).unwrap();

    let line_length = contents.find("\n").unwrap() + 1;

    let mut map = Map::new();

    let mut number_index = 0;

    let re = Regex::new(r"(\d+)|(\*)").unwrap();
    for capture in re.captures_iter(contents.as_str()) {
        if let Some(number) = capture.get(1) {
            let x = number.start() / line_length;
            let y1 = number.start() % line_length;
            let y2 = number.end() % line_length;

            let entry = Entry::Number(Number {
                index: number_index,
                value: number.as_str().parse::<u64>().unwrap(),
            });
            number_index += 1;

            for y in y1..y2 {
                map.set_position(x, y, entry);
            }
        }

        if let Some(symbol) = capture.get(2) {
            let x = symbol.start() / line_length;
            let y = symbol.start() % line_length;

            map.set_position(x, y, Entry::Gear);
        }
    }

    //    dbg!(&map);

    println!(
        "The sum of all gear ratios is {}",
        map.calculate_sum_of_symbol_neighbors()
    );
}
