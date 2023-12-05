use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{env, thread};

use nohash::IntSet;

#[derive(Clone, Copy)]
struct Range {
    source_start: i64,
    dest_start: i64,
    count: i64,
}

#[derive(Clone)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new() -> Self {
        Map { ranges: vec![] }
    }

    fn push_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    fn map_source_to_dest(&self, source: i64) -> i64 {
        for range in &self.ranges {
            if (range.source_start..range.source_start + range.count).contains(&source) {
                return &source + (range.dest_start - range.source_start);
            }
        }

        source
    }
}

fn main() {
    let file_name = env::args().nth(1).unwrap();

    let contents = std::fs::read_to_string(file_name).unwrap();

    let mut mappings: Vec<Map> = vec![];
    let mut mapping = Map::new();

    let mut pairs: Vec<(i64, i64)> = vec![];

    for line in contents.lines() {
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Save the starting seeds
        if line.contains("seeds:") {
            let line: Vec<&str> = line.split_whitespace().skip(1).collect();

            for i in (0..line.len()).step_by(2) {
                let min: i64 = line[i].parse().unwrap();
                let length: i64 = line[i + 1].parse().unwrap();

                pairs.push((min, min + length));
            }

            continue;
        }

        // Start a new mapping when a new "blank-to-blank" is found
        if line.contains("-to-") {
            if !mapping.ranges.is_empty() {
                mappings.push(mapping);
                mapping = Map::new();
            }

            continue;
        }

        let line: Vec<&str> = line.split_whitespace().collect();

        let range = Range {
            source_start: line[1].parse::<i64>().unwrap(),
            dest_start: line[0].parse::<i64>().unwrap(),
            count: line[2].parse::<i64>().unwrap(),
        };

        mapping.push_range(range);
    }

    mappings.push(mapping);

    // Loop through every pair.
    // Check each pair on a new thread.
    // Thread locks mutex and stores minimum when complete.
    // Join all threads.
    
    let min: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(None));

    let mut threads = Vec::new();
    
    for index in 0..pairs.len() {
        let min = Arc::clone(&min);
        let pair = pairs[index].clone();
        let mappings = mappings.clone();

        threads.push(thread::spawn(move || {
            println!("Spawning thread for {:?}", pair);
            
            let mut local_min: Option<i64> = None;

            for seed in pair.0..pair.1 {
                let mut val = seed;

                for mapping in &mappings {
                    val = mapping.map_source_to_dest(val);
                }

                local_min = Some(match local_min {
                    None => val,
                    Some(min) => std::cmp::min(min, val),
                });
            }

            let mut global_min = min.lock().unwrap();

            *global_min = Some(match *global_min {
                None => local_min.unwrap(),
                Some(global_min) => std::cmp::min(global_min, local_min.unwrap()),
            });
            
            println!("Thread for {:?} finished. Minimum location found was {:?}", pair, local_min);
        }));
    }
    
    threads.into_iter().for_each(|thread| {
        thread.join().unwrap();
    });
    
//    for pair in &pairs {
//        let min = Arc::clone(&min);
//        let pair = pair.clone();
//        let mappings = mappings.clone();
//        
//        thread::spawn(move || {
//            let mut local_min: Option<i64> = None;
//            
//            for seed in pair.0..pair.1 {
//                let mut val = seed;
//                
//                for mapping in &mappings {
//                    val = mapping.map_source_to_dest(val);
//                }
//                
//                local_min = Some(match local_min {
//                    None => val,
//                    Some(min) => std::cmp::min(min, val),
//                });
//            }
//            
//            let mut global_min = min.lock().unwrap();
//            
//            *global_min = Some(match *global_min {
//                None => local_min.unwrap(),
//                Some(global_min) => std::cmp::min(global_min, local_min.unwrap()),
//            });
//        }).join().unwrap();
//    }
    
    println!("The minimum location is {}", min.lock().unwrap().unwrap());
    
    
    // Check for intersections between seed to soil source ranges and input seed ranges

    // Starting source range:              |----------------------|
    // Input seed range:                           |-----------------------------|
    // We only want to check these:                |--------------|
    //                                             ^ seed start
    //                                                            ^ source end
    // OR: input seed range:            |--------|
    // we only want to check these:        |-----|

    //    let mut seeds: IntSet<i64> = IntSet::default();
    //
    //    for range in &mappings[0].ranges {
    //        let source_range = range.source_start..range.source_start + range.count;
    //        println!("Source range: {:#?}", source_range);
    //
    //        for pair in &pairs {
    //            let seed_range = pair.0..pair.1;
    //            println!("Seed range: {:#?}", seed_range);
    //
    //            if source_range.contains(&seed_range.start) || source_range.contains(&seed_range.end) {
    //                // So... we want the higher of (source_start, seed_start) and lower of (source_end, seed_end).
    //                let start = std::cmp::max(source_range.start, seed_range.start);
    //                let end = std::cmp::min(source_range.end, seed_range.end);
    //
    //                for seed in start..end {
    //                    if seed % 1_000_000 == 0 {
    //                        println!("Adding seed: {}", &seed);
    //                    }
    //
    //                    seeds.insert(seed);
    //                }
    //            }
    //        }
    //    }
    //
    //    let mut min: Option<i64> = None;
    //
    //    for seed in &seeds {
    //        let mut val = seed.clone();
    //
    //        for mapping in &mappings {
    //            val = mapping.map_source_to_dest(val);
    //        }
    //
    //        if val == 0 {
    //            dbg!(&val);
    //        }
    //
    //        min = Some(match min {
    //            None => val,
    //            Some(min) => std::cmp::min(min, val),
    //        });
    //    }
    //
    //    println!("The minimum location is {}", min.unwrap());
}
