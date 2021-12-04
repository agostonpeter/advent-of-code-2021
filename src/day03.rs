use crate::file;

pub fn solve() {
    println!("THIRD DAY");
    if let Ok(lines) = file::read_lines("inputs/input03.txt") {
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
