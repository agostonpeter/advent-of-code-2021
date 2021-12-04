use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    println!("FIRST DAY");
    if let Ok(lines) = read_lines("inputs/input01.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut prev_prev_line: i32 = 9999;
        let mut prev_line: i32 = 9999;
        let mut prev_window: i32 = 3 * 9999;
        let mut increase_count = 0;
        let mut increase_count_window = 0;
        for line in lines {
            if let Ok(this_line_string) = line {
                let this_line: i32 = this_line_string.parse().unwrap();
                let this_window: i32 = this_line + prev_line + prev_prev_line;
                if this_line - prev_line > 0 {
                    increase_count += 1;
                }
                if this_window - prev_window > 0 {
                    increase_count_window += 1;
                }
                //println!("{} {} {}", prev_line, this_line, this_line - prev_line);
                prev_prev_line = prev_line;
                prev_line = this_line;
                prev_window = this_window;
            }
        }
        println!("number of increases: {}", increase_count);
        println!("number of window increases: {}", increase_count_window);
    }
    println!("\n-------\n");
    println!("SECOND DAY");
    if let Ok(lines) = read_lines("inputs/input02.txt") {
        let mut position_horizontal = 0;
        let mut position_vertical = 0;

        let mut pos_hor = 0;
        let mut depth = 0;
        let mut aim = 0;

        for line in lines {
            if let Ok(this_line_string) = line {
                let this_line: Vec<_> = this_line_string.split_whitespace().collect();
                //println!("{}", this_line[0]);
                //println!("{}", this_line[1]);

                //let mut direction = String::new();
                //direction = this_line[0].parse();
                let distance: i32 = this_line[1].parse().unwrap();
                if this_line[0] == "forward" {
                    position_horizontal += distance;
                    pos_hor += distance;
                    depth += aim * distance;
                } else if this_line[0] == "up" {
                    position_vertical -= distance;
                    aim -= distance;
                } else if this_line[0] == "down" {
                    position_vertical += distance;
                    aim += distance;
                }

                /*
                println!(
                    "horizontal: {} , vertical: {}",
                    position_horizontal, position_vertical
                );
                */
            }
        }
        println!("PART ONE:");
        println!(
            "horizontal: {} , vertical: {}",
            position_horizontal, position_vertical
        );
        println!(
            "horizontal * vertical = {}",
            position_horizontal * position_vertical
        );
        println!("PART TWO:");
        println!("hor: {} , depth: {}", pos_hor, depth);
        println!("hor * depth = {}", pos_hor * depth);
    }
    println!("\n-------\n");
    println!("THIRD DAY");
    if let Ok(lines) = read_lines("inputs/input03.txt") {
        let mut counter_0 = vec![0; 12];
        let mut counter_1 = vec![0; 12];
        let mut lines_vector: Vec<String> = vec![];
        for line in lines {
            if let Ok(this_line_string) = line {
                lines_vector.push(this_line_string.clone());
                //println!("{}", this_line_string);
                for (i, c) in this_line_string.chars().enumerate() {
                    //println!("{}: {}", i, c);
                    if c == '0' {
                        counter_0[i] += 1;
                    } else if c == '1' {
                        counter_1[i] += 1;
                    }
                    // do something with character `c` and index `i`
                }
            }
        }
        let mut result_bin_string = String::new();
        for (i, c) in counter_0.iter().enumerate() {
            //println!("{}: {}", i, c);
            if c > &counter_1[i] {
                result_bin_string = result_bin_string + "0";
            } else if c < &counter_1[i] {
                result_bin_string = result_bin_string + "1";
            }
        }
        let gamma = i32::from_str_radix(&result_bin_string, 2).unwrap();
        let epsilon = i32::pow(2, 12) - 1 - &gamma;
        println!("PART ONE:");
        //println!("counter_0: {:?} , counter_1: {:?}", counter_0, counter_1);
        //println!("result binary string: {:?}", result_bin_string);
        println!(
            "gamma: {:?}, epsilon: {:?}, power consuption: {:?}",
            gamma,
            epsilon,
            gamma * epsilon
        );
        println!("PART TWO:");
        let mut i = 0;
        let mut s_vec: (Vec<String>, Vec<String>) = (vec![], vec![]);
        let mut result_vec: Vec<String> = lines_vector.clone();
        let mut result = "";

        loop {
            s_vec = split_vector(result_vec, i);
            //println!("{:?}, {:?}", s_vec.0.len(), s_vec.1.len());

            if s_vec.0.len() > s_vec.1.len() {
                result_vec = s_vec.0.clone();
            } else {
                result_vec = s_vec.1.clone();
            }
            if result_vec.len() == 1 {
                result = &result_vec[0];
                break;
            }
            i += 1;
            if i == 12 {
                println!("break on i == 12");
                break;
            }
        }
        let o_gen_result = i32::from_str_radix(&result, 2).unwrap();
        println!(
            "O generator binary: {:?}, O generator int: {:?}",
            result, o_gen_result
        );

        result_vec = lines_vector.clone();
        result = "";
        i = 0;
        loop {
            s_vec = split_vector(result_vec, i);
            //println!("{:?}, {:?}", s_vec.0.len(), s_vec.1.len());

            if s_vec.1.len() < s_vec.0.len() {
                result_vec = s_vec.1.clone();
            } else {
                result_vec = s_vec.0.clone();
            }
            if result_vec.len() == 1 {
                result = &result_vec[0];
                break;
            }
            i += 1;
            if i == 12 {
                println!("break on i == 12");
                break;
            }
        }
        let co2_scrub_result = i32::from_str_radix(&result, 2).unwrap();
        println!(
            "CO2 scrub binary: {:?}, CO2 scrub int: {:?}",
            result, co2_scrub_result
        );
        println!("life support rating: {:?}", o_gen_result * co2_scrub_result);
    }
}

fn split_vector(list: Vec<String>, position: usize) -> (Vec<String>, Vec<String>) {
    let mut results: (Vec<String>, Vec<String>) = (vec![], vec![]);
    for value in list {
        let ch = value.chars().nth(position).unwrap();
        if ch == '0' {
            results.0.push(value);
        } else {
            results.1.push(value);
        }
    }
    results
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
