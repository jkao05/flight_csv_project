use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
mod bfs;
use bfs::average_hops;

fn main() {
    let flight_file = File::open("flights_cleaned.csv").expect("failed to open file");
    let flight_reader = BufReader::new(flight_file);

    let mut flight_hash: HashMap<String, Vec<String>> = HashMap::new(); //make empty hashmap for flight data
    //flight_hash is formatted ()

    let mut is_first_line = true;
    let mut cut_data_index = 0; //cut data at this line bc it uses airport codes
    for line in flight_reader.lines() { //for each line in the csv file
        let line = line.expect("failed to read line");

        if is_first_line { //skip the first line bc its the column names
            is_first_line = false;
            continue;
        }

        let parts: Vec<&str> = line.trim().split(',').collect(); //split each line into its 4 parts and put into the vector parts
        let origin = parts[0].trim().to_string().to_lowercase(); //get current origin from parts
        let current_dest = parts[1].trim().to_string().to_lowercase(); //get current destination from parts

        if flight_hash.contains_key(&origin) { //if the key exists already,
            let destinations = flight_hash.get_mut(&origin).unwrap(); // get the mutable reference to the destinations 
            destinations.push(current_dest); //add the current destination to the existing destinations
        } else {
            flight_hash.insert(origin, vec![current_dest]); //else, create new key and value
        }

        if cut_data_index == 4299027 {
            break; //once we have hit the index 4299027, we are done collecting all the non number airport names 
        }
        cut_data_index += 1;
    }

    // at the end of this, we will have a hashmap that contains as a key each airport, and the according values being all the existing
    // destinations for said key


    let mut total_avg: f64 = 0.0;
    for (origin, _destination) in &flight_hash {
        let avg = average_hops(origin, &flight_hash);
        total_avg += avg;
    }

    println!("average hops = {}", total_avg / flight_hash.len() as f64);

}
