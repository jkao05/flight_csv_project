use std::collections::HashMap;

pub fn two_sample_t_test(data_hash: &HashMap<String, f64>,  cluster_0: &Vec<String>,  cluster_1: &Vec<String>) -> Vec<f64> {
    let mut values_0 = Vec::new();
    for code in cluster_0 {
        let value = data_hash[code];
        values_0.push(value);
    }

    let mut values_1 = Vec::new();
    for code in cluster_1 {
        let value = data_hash[code];
        values_1.push(value);
    }

    let mean_0 = mean(&values_0);
    let mean_1 = mean(&values_1);
    let std_0 = std_dev(&values_0, mean_0);
    let std_1 = std_dev(&values_1, mean_1);

    let n0 = values_0.len() as f64;
    let n1 = values_1.len() as f64;

    let se = ((std_0 * std_0 / n0) + (std_1 * std_1 / n1)).sqrt();
    let t_statistic = (mean_0 - mean_1) / se;

    // Welch-Satterthwaite approximation for degrees of freedom
    let numerator = (std_0 * std_0 / n0 + std_1 * std_1 / n1).powi(2);
    let denominator = ((std_0 * std_0 / n0).powi(2)) / (n0 - 1.0)
        + ((std_1 * std_1 / n1).powi(2)) / (n1 - 1.0);
    let degrees_of_freedom = numerator / denominator;

    return vec![t_statistic, degrees_of_freedom]
}

fn mean(data: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..data.len() {
        sum += data[i];
    }
    return sum / data.len() as f64
}

fn std_dev(data: &Vec<f64>, mean: f64) -> f64 {
    let mut sum = 0.0;
    for i in 0..data.len() {
        let diff = data[i] - mean;
        sum += diff * diff;
    }
    let var = sum / (data.len() as f64 - 1.0);
    return var.sqrt()
}