use std::collections::HashMap;

pub fn normalize(data: HashMap<String, Vec<f64>>) -> HashMap<String, Vec<f64>> {

    //noramlized = (value - min)/(max-min)
    let mut min_vals = vec![f64::MAX; 3];
    let mut max_vals = vec![f64::MIN; 3];

    // find min and max
    for values in data.values() {
        for i in 0..3 {
            if values[i] < min_vals[i] {
                min_vals[i] = values[i];
            }
            if values[i] > max_vals[i] {
                max_vals[i] = values[i];
            }
        }
    }

    // create new hashmap with normalized values
    let mut normalized: HashMap<String, Vec<f64>> = HashMap::new();

    for (key, values) in data {
        let mut scaled: Vec<f64> = Vec::new();
        for i in 0..3 {
            let min = min_vals[i];
            let max = max_vals[i];
            let raw = values[i];
            let norm: f64;
            if max > min {
                norm = (raw - min) / (max - min); //apply formula
            } else {
                norm = 0.0;
            }
            scaled.push(norm);
        }
        normalized.insert(key, scaled); //update the key w new scaled values
    }
    return normalized
}


pub fn k_means_cluster(data: &HashMap<String, Vec<f64>>,   iterations: usize,) -> HashMap<String, usize> {
    let mut centroids: [Vec<f64>; 2] = [
    [1.0, 0.6007, 0.0041].to_vec(),  // wrg data
    [0.0, 0.5328, 0.1593].to_vec()   // atl data
    ];

    let mut assignments: HashMap<String, usize> = HashMap::new();

    for _ in 0..iterations {
        // assign groups:
        assignments.clear();
        for (airport, features) in data {
            let mut closest = 0;
            let mut min_distance = f64::MAX;

            for (i, centroid) in centroids.iter().enumerate() {
                let dist = euclidean_distance(features, centroid);
                if dist < min_distance {
                    min_distance = dist;
                    closest = i;
                }
            }

            assignments.insert(airport.to_string(), closest);
        }

        // update centroids:
        let mut new_centroids = vec![vec![0.0; 3], vec![0.0; 3]];
        let mut counts = [0, 0];

        for (airport, cluster) in &assignments {
            let features = &data[airport];
            counts[*cluster] += 1;
            for i in 0..3 {
                new_centroids[*cluster][i] += features[i];
            }
        }

        for i in 0..2 {
            if counts[i] > 0 {
                for j in 0..3 {
                    new_centroids[i][j] /= counts[i] as f64;
                }
                for j in 0..3 {
                    centroids[i][j] = new_centroids[i][j];
                }
            }
        }
    }

    return assignments
}

fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    return sum.sqrt()
}