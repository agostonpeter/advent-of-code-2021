use crate::file;

pub fn solve() {
    println!("SEVENTH DAY");
    if let Ok(lines) = file::read_lines("inputs/input07.txt") {
        let mut input_strings: Vec<String>;
        let mut inputs: Vec<i32> = vec![];

        for line in lines {
            if let Ok(this_line_string) = line {
                input_strings = this_line_string.split(",").map(String::from).collect();
                for input_string in input_strings {
                    let input_int: i32 = input_string.parse().unwrap();
                    inputs.push(input_int)
                }
            }
        }
        println!("PART ONE:"); //result: distance: 353800, at 343
        let mut distance: i64 = 0;
        let mut min_distance: u64 = 0;
        let mut n_at_min = 0;
        for n in 0..9999 {
            for input in &inputs {
                distance += (n as i64 - *input as i64).abs();
            }
            //println!("n: {:?},  {:?}", n, distance);
            if min_distance == 0 {
                min_distance = distance as u64;
                n_at_min = n;
            } else if (distance as u64) < min_distance {
                min_distance = distance as u64;
                n_at_min = n;
            }
            distance = 0;
        }
        println!("distance: {:?}, at {:?}", min_distance, n_at_min);

        println!("PART TWO:");
        let mut fuel: i64 = 0;
        let mut min_fuel: i64 = 0;
        n_at_min = 0;
        for n in 0..9999 {
            for input in &inputs {
                let distance = (n as i64 - *input as i64).abs();
                fuel += distance * (distance + 1) / 2;
            }
            if min_fuel == 0 {
                min_fuel = fuel;
                n_at_min = n;
            } else if fuel < min_fuel {
                min_fuel = fuel;
                n_at_min = n;
            }
            fuel = 0;
        }
        println!("fuel: {:?}, at {:?}", min_fuel, n_at_min);
    }
}
