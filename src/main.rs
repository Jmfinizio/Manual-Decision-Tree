use std::fs::File;
use std::io::prelude::*;
use rand::Rng;


fn generate_file(path: &str, n: usize) {
    // Generate a random file of edges for vertices 0.n
    let mut file = File::create(path).expect("Unable to create file");
    for _ in 0..n {
        // How many neighbors will this node have
        let mut rng = rand::thread_rng();
        let random_number: i32 = rng.gen_range(-100000000..=100000000);
        let random_label: u8 = rng.gen_range(0..=1);
        let tuple = format!("({},{})\n", random_number, random_label);
                file.write_all(tuple.as_bytes()).expect("Unable to write file");
    }
}        

fn read_file(path: &str) -> Vec<(i32, u8)> {
    let mut result: Vec<(i32, u8)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim_matches(|c| c == '(' || c == ')').split(',').collect();
        if v.len() == 2 {
            if let (Ok(x), Ok(y)) = (v[0].trim().parse(), v[1].trim().parse()) {
                result.push((x, y));
            } else {
                eprintln!("Error with line: {}", line_str);
            }
        } else {
            eprintln!("Line format error: {}", line_str);
        }
    }
    return result;
}

fn calculate_accuracy(data: &[(i32,u8)], threshold: i32) -> (f64, bool) {
    let mut counter = 0;
    let mut best_accuracy = 0.0;
    let mut one_to_the_right = false;
    for (x,z) in data {
        if x >= &threshold && *z == 1 {
            counter += 1
        }
        else if x < &threshold && *z == 0 {
            counter += 1
        }
    }
    let first_accuracy: f64 = counter as f64 /data.len() as f64;
    counter = 0;
    for (x,z) in data {
        if x >= &threshold && *z == 0 {
            counter += 1
        }
        else if x < &threshold && *z == 1 {
            counter += 1
        }
    }
    let second_accuracy: f64 = counter as f64 /data.len() as f64;
    if first_accuracy > second_accuracy {
        best_accuracy = first_accuracy;
        one_to_the_right = true;
    }
    else if second_accuracy > first_accuracy {
        best_accuracy = second_accuracy;
        one_to_the_right = false;
    }
    (best_accuracy, one_to_the_right)
}

fn decision_tree(data: &[(i32,u8)]) -> (i32,f64,bool) {
    let mut best_accuracy: f64 = 0.0;
    let mut one_to_the_right: bool = false;
    let mut best_threshold = 0;
    for (x,z) in data {
        let threshold = x;
        let accuracy = calculate_accuracy(data, *threshold as i32);
        if accuracy.0 > best_accuracy {
            best_accuracy = accuracy.0;
            one_to_the_right = accuracy.1;
            best_threshold = *threshold as i32;
        }
    }
    (best_threshold, best_accuracy, one_to_the_right)
}
    


fn main() {
    generate_file("data.txt", 100);
    let data = read_file("data.txt");
    let result = decision_tree(&data);
    if result.2 == true {
        println!("The best split is on {:?}, with an accuracy of {:?}. Anything greater than {:?} should be
        labeled 1, and anything less should be labeled 0.", result.0, result.1, result.0);  
    }
    else if result.2 == false{
        println!("The best split is on {:?}, with an accuracy of {:?}. Anything greater than {:?} should be
        labeled 1, and anything less should be labeled 0.", result.0, result.1, result.0);  
    }

}
