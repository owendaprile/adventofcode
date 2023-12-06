// distance = (total_time - time_held) * time_held
// distance = (total_time * time_held) - (time_held * time_held)


#[derive(Clone, Copy, Debug)]
struct Race {
    time: Option<i64>,
    distance: Option<i64>,
}

impl Race {
    fn count_winning_times(&self) -> i64 {
        let mut count = 0;

        let total_time = self.time.unwrap();
        for time_held in 0..=total_time {
            if (total_time - time_held) * time_held > self.distance.unwrap() {
                count += 1;
            }
        }
        
        count
    }
}

fn read_input(contents: &String) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    
    for line in contents.lines() {
        if line.starts_with("Time") {
            for (index, time) in line.split_whitespace().skip(1).enumerate() {
                // try to get the race... if it's not there, create and insert
                let time: i64 = time.parse().unwrap();
                
                if let Some(race) = races.get_mut(index) {
                    race.time = Some(time);
                } else {
                    races.push(Race { time: Some(time), distance: None });
                }
            }
        } else if line.starts_with("Distance") {
            for (index, distance) in line.split_whitespace().skip(1).enumerate() {
                let distance: i64 = distance.parse().unwrap();
                
                if let Some(race) = races.get_mut(index) {
                    // Add the distance
                    race.distance = Some(distance);
                } else {
                    races.push(Race { time: None, distance: Some(distance) })
                }
            }
        }
    }
    
    races
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    
    let contents = std::fs::read_to_string(filename).unwrap();
    
    let races = read_input(&contents);
    
    let result: i64 = races.iter()
        .map(Race::count_winning_times)
        .product();
    
    println!("The answer is {}", result);
}
