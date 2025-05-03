use std::collections::{HashMap, HashSet};

pub fn average_hops(start: &str, flight_hash: &HashMap<String, Vec<String>>) -> f64 {
    let mut airports_visited: HashSet<String> = HashSet::new(); //airports we have already visited
    let mut airports_to_visit: Vec<String> = vec![start.to_string()]; //airports we still need to visit
    let mut distance: HashMap<String, usize> = HashMap::new(); //hop distance from the start airport

    airports_visited.insert(start.to_string()); //mark the start airport as visited
    distance.insert(start.to_string(), 0); //initialize distance from start airport to itself as 0

    while !airports_to_visit.is_empty() { //while there are airports to look thru:
        let current_airport = airports_to_visit.remove(0); //get the current airport to look at + remove it from airports we need to look at

        for neighbor in &flight_hash[&current_airport] { //go thru each neighbor of current airport
            if !airports_visited.contains(neighbor) { //if we have not visited this neighbor yet
                airports_visited.insert(neighbor.to_string()); //add neighbor as visited
                airports_to_visit.push(neighbor.to_string()); //add neighbor as airport we must look at later
                let d = distance[&current_airport] + 1; //distance to neighbor is one more than current airport
                distance.insert(neighbor.to_string(), d); //update the distance to this neighbor
            }
        }

    }

    let mut total_hops = 0;
    let mut reachable_airports = 0;

    for &d in distance.values() { //go thru each airport's num hops away from start
        if d > 0 { //if the airport is not itself
            total_hops += d; //get total hops
            reachable_airports += 1; //num airports
        }
    }
    //return avg num of hops for this airport
    return total_hops as f64 / reachable_airports as f64 

}
