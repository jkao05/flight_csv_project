use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let flight_file = File::open("flights_cleaned.csv").expect("failed to open file");
    let flight_reader = BufReader::new(flight_file);

    let mut flight_hash: HashMap<String, Vec<String>> = HashMap::new(); //make empty hashmap for flight data
    //flight_hash is formatted ()

    let mut is_first_line = true;

    for line in flight_reader.lines() { //for each line in the csv file
        let line = line.expect("failed to read line");

        if is_first_line { //skip the first line bc its the column names
            is_first_line = false;
            continue;
        }

        let parts: Vec<&str> = line.trim().split(',').collect(); //split each line into its 4 parts and put into the vector parts
        let origin = parts[0].trim().to_string().to_lowercase(); //get current origin from parts
        let current_dest = parts[1].trim().to_string().to_lowercase(); //get current destination from parts

        let mut destinations: Vec<String> = Vec::new();

        if flight_hash.contains_key(&origin) { //if we have already added this origin to the hashmap before:
            destinations = flight_hash.get(&origin).unwrap().clone(); //then get the value for this origin aka the destination(s)
        }

        destinations.push(current_dest); //add the current destination to the destination(s)
        flight_hash.insert(origin, destinations); //update the new destinations
    }
    // at the end of this, we will have a hashmap that contains as a key each airport, and the according values being all the existing
    // destinations for said key

    //print each origin, then its vector of destinations
    for (origin, destinations) in &flight_hash {
        println!("{}", origin);
        println!("{:?}", destinations);
        println!();
    }
}