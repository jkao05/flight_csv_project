use std::collections::{HashMap, HashSet};

pub fn bfs_airport(start: &str, flight_hash: &HashMap<String, Vec<String>>) -> (Vec<String>, usize) {
    let mut airports_visited: HashSet<String> = HashSet::new(); //airports we have already visited
    let mut airports_to_visit: Vec<String> = vec![start.to_string()]; //airports we still need to visit
    let mut distance: HashMap<String, usize> = HashMap::new(); //hop distance from the start airport

    airports_visited.insert(start.to_string()); //mark the start airport as visited
    distance.insert(start.to_string(), 0); //initialize distance from start airport to itself as 0

    let mut furthest_airports: Vec<String> = vec![start.to_string()]; //list of airports that are furthest away
    let mut max_distance = 0; //initialize max hop distance from the start airport

    while !airports_to_visit.is_empty() { //while there are airports to look thru:
        let current_airport = airports_to_visit.remove(0); //get the current airport to look at + remove it from airports we need to look at

        for neighbor in &flight_hash[&current_airport] { //go thru each neighbor of current airport
            if !airports_visited.contains(neighbor) { //if we have not visited this neighbor yet
                airports_visited.insert(neighbor.to_string()); //add neighbor as visited
                airports_to_visit.push(neighbor.to_string()); //add neighbor as airport we must look at later
                let d = distance[&current_airport] + 1; //distance to neighbor is one more than current airport
                distance.insert(neighbor.to_string(), d); //update the distance to this neighbor

                if d > max_distance { //if this is the new longest distance
                    max_distance = d; //update the longest distance
                    furthest_airports = vec![neighbor.to_string()]; //reset the list to just this airport
                } else if d == max_distance { //if this airport is tied for longest distance
                    furthest_airports.push(neighbor.to_string()); //add to the list of furthest airports
                }
            }
        }

    }

    (furthest_airports, max_distance)
}

