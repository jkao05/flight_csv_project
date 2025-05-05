use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

mod bfs;
use bfs::average_hops;

mod clustering;
use clustering::{k_means_cluster, normalize};

mod stats;
use stats::two_sample_t_test;

fn main() {
    //put data into hashmaps:
    let flight_file = File::open("flights_cleaned.csv").expect("failed to open file");
    let flight_reader = BufReader::new(flight_file);

    let mut flight_hash: HashMap<String, Vec<String>> = HashMap::new(); //make empty hashmap for flight data

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

        //flight_hash computation

        if flight_hash.contains_key(&origin) { //if the key exists already,
            let destinations = flight_hash.get_mut(&origin).unwrap(); // get the mutable reference to the destinations
            destinations.push(current_dest); //add the current destination to the existing destinations
        } else {
            flight_hash.insert(origin.clone(), vec![current_dest]); //else, create new key and value cloned
        }

        if cut_data_index == 4299027 {
            break; //once we have hit the index 4299027, we are done collecting all the non number airport names 
        }
        cut_data_index += 1;
    }

    // at the end of this, we will have a hashmap that contains as a key each airport, and the according values being all the existing
    // destinations for said key

    //compute the average hops across the data, aka the average number of flights one must take to get from place to place
    let mut total_avg: f64 = 0.0;
    let mut highest_avg_hops: (f64, Vec<String>) = (0.0, Vec::new()); //vec of highest avg, airport(s) name(s)
    let mut smallest_avg_hops: (f64, Vec<String>) = (f64::MAX, Vec::new()); //vec of lowest avg, airport(s) name(s)
    
    for (origin, _destination) in &flight_hash {
        //compute total avg
        let avg = average_hops(origin, &flight_hash);
        total_avg += avg;
    
        //compute highest hops airport(s)
        if avg > highest_avg_hops.0 { //if this avg is greater than our highest avg so far:
            highest_avg_hops = (avg, vec![origin.to_string()]); // then make this avg and airport the highest avg
        } else if avg == highest_avg_hops.0 { //else if this avg is the same as our highest so far:
            highest_avg_hops.1.push(origin.to_string()); //add it to the list
        }
    
        //compute lowest hops airport(s)
        if avg < smallest_avg_hops.0 { //if this avg is less than our smallest avg so far:
            smallest_avg_hops = (avg, vec![origin.to_string()]); // then make this avg and airport the lowest avg
        } else if avg == smallest_avg_hops.0 { //else if this avg is the same as our lowest so far:
            smallest_avg_hops.1.push(origin.to_string()); //add it to the list
        }
    }

    println!("average hops = {}", total_avg / flight_hash.len() as f64);
    println!("most avg hops airport = {:?}", highest_avg_hops);
    println!("lowest avg hops airport = {:?}", smallest_avg_hops);
    println!("");

    //commit 4: begin computing the k means clustering

    //first things first, compute avg distance per airport and avg delay per airport

    // avg_distance_hash from distance_totals:
    let flight_file = File::open("flights_cleaned.csv").expect("failed to open file");
    let flight_reader = BufReader::new(flight_file);

    let mut avg_distance_hash: HashMap<String, f64> = HashMap::new(); //airport names and their avg distance
    let mut distance_totals: HashMap<String, (f64, usize)> = HashMap::new(); //distance totals to compute avg_distance_hash

    let mut is_first_line = true;
    let mut cut_data_index = 0;
    for line in flight_reader.lines() {
        let line = line.expect("failed to read line");

        if is_first_line { //skip first line
            is_first_line = false;
            continue;
        }

        let parts: Vec<&str> = line.trim().split(',').collect();
        let origin = parts[0].trim().to_string().to_lowercase();
        let current_dest = parts[1].trim().to_string().to_lowercase();
        let distance: f64 = parts[2].trim().parse().unwrap_or(0.0);

        if distance_totals.contains_key(&origin) {
            let entry = distance_totals.get_mut(&origin).unwrap();
            entry.0 += distance;
            entry.1 += 1;
        } else {
            distance_totals.insert(origin, (distance, 1));
        }

        if distance_totals.contains_key(&current_dest) {
            let entry = distance_totals.get_mut(&current_dest).unwrap();
            entry.0 += distance;
            entry.1 += 1;
        } else {
            distance_totals.insert(current_dest, (distance, 1));
        }

        if cut_data_index == 4299027 {
            break;
        }
        cut_data_index += 1;
    }

    for (airport, (total_dist, count)) in &distance_totals {
        avg_distance_hash.insert(airport.to_string(), total_dist / *count as f64); //compute avg_distance_hash
    }

    // avg_delay_hash from delay_totals:
    let flight_file = File::open("flights_cleaned.csv").expect("failed to open file");
    let flight_reader = BufReader::new(flight_file);

    let mut avg_delay_hash: HashMap<String, f64> = HashMap::new();
    let mut delay_totals: HashMap<String, (f64, usize)> = HashMap::new();

    let mut is_first_line = true;
    let mut cut_data_index = 0;
    for line in flight_reader.lines() {
        let line = line.expect("failed to read line");

        if is_first_line {
            is_first_line = false;
            continue;
        }

        let parts: Vec<&str> = line.trim().split(',').collect();
        let origin = parts[0].trim().to_string().to_lowercase();
        let delay: f64 = parts[3].trim().parse().unwrap_or(0.0);

        if delay_totals.contains_key(&origin) {
            let entry = delay_totals.get_mut(&origin).unwrap();
            entry.0 += delay;
            entry.1 += 1;
        } else {
            delay_totals.insert(origin, (delay, 1));
        }

        if cut_data_index == 4299027 {
            break;
        }
        cut_data_index += 1;
    }

    for (origin, (total_delay, count)) in &delay_totals {
        avg_delay_hash.insert(origin.to_string(), total_delay / *count as f64);
    }
    
    //make hashmap for avg hops
    let mut avg_hops_hash: HashMap<String, f64> = HashMap::new();

    for (airport, neighbor_vec) in &flight_hash {
        let avg_hops = neighbor_vec.len() as f64;
        avg_hops_hash.insert(airport.to_string(), avg_hops);
    }

    //secondly, add avg distance per airport, avg delay per airport, and avg hops into a new hashmap
    let mut airport_cluster_data: HashMap<String, Vec<f64>> = HashMap::new();

    for (airport, _neighbor_vec) in &flight_hash {
        let hops = avg_hops_hash[airport]; // get avg hops
        let delay = avg_delay_hash[airport]; //get avg delay
        let distance = avg_distance_hash[airport]; //get avg distance
    
        airport_cluster_data.insert(airport.to_string(), vec![hops, delay, distance]); //finalize airport_cluster_data
    }
    //normalize the data before k means clusteirng
    let airport_cluster_data = normalize(airport_cluster_data);

    //when done, use this hashmap with the k means clustering module

    let clusters = k_means_cluster(&airport_cluster_data, 10); // run k-means with 10 iterations

    // print airports in their according cluster groups
    let mut cluster_0_airports: Vec<String> = Vec::new(); //wrg group
    let mut cluster_1_airports: Vec<String> = Vec::new(); //atl group
    
    for (airport, group) in &clusters {
        if *group == 0 { //add wrg airports onto wrg vec
            cluster_0_airports.push(airport.to_string());
        } else if *group == 1 { //add atl ariports onto atl vec
            cluster_1_airports.push(airport.to_string());
        }
    }
    
    // print the groups
    println!("Cluster 0 Airports (WRG group)");
    println!("{:?}", cluster_0_airports);
    println!();

    println!("Cluster 1 Airports (ATL group)");
    println!("{:?}", cluster_1_airports);
    println!();
    
    println!("Total in Cluster 0: {}", cluster_0_airports.len());
    println!("Total in Cluster 1: {}", cluster_1_airports.len());
    println!();

    //compute statistical significance w 2 sample z test

    //t test for avg_hops_hash
    let hops_t_test: Vec<f64> = two_sample_t_test(&avg_hops_hash, &cluster_0_airports, &cluster_1_airports);
    println!("hops test statistic = {}, and df = {}", hops_t_test[0], hops_t_test[1]);

    //t test for avg_distance_hash
    let distance_t_test: Vec<f64> = two_sample_t_test(&avg_distance_hash, &cluster_0_airports, &cluster_1_airports);
    println!("distance test statistic = {}, and df = {}", distance_t_test[0], distance_t_test[1]);

    //t test for avg_delay hash
    let delay_t_test: Vec<f64> = two_sample_t_test(&avg_delay_hash, &cluster_0_airports, &cluster_1_airports);
    println!("delays test statistic = {}, and df = {}", delay_t_test[0], delay_t_test[1]);
}
