use crate::file;
use std::cmp;

pub fn solve() {
    println!("FIFTH DAY");
    if let Ok(lines) = file::read_lines("inputs/input05.txt") {
        let mut input_strings: Vec<String>;
        let mut inputs: Vec<((i32, i32), (i32, i32))> = vec![];
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;

        for line in lines {
            if let Ok(this_line_string) = line {
                input_strings = this_line_string.split(" -> ").map(String::from).collect();
                let mut coordinates_strings: Vec<String> =
                    input_strings[0].split(",").map(String::from).collect();
                let x1: i32 = coordinates_strings[0].parse().unwrap();
                let y1: i32 = coordinates_strings[1].parse().unwrap();
                let first_coordinate: (i32, i32) = (x1, y1);
                coordinates_strings = input_strings[1].split(",").map(String::from).collect();
                let x2: i32 = coordinates_strings[0].parse().unwrap();
                let y2: i32 = coordinates_strings[1].parse().unwrap();
                let second_coordinate: (i32, i32) = (x2, y2);
                let coordinates: ((i32, i32), (i32, i32)) = (first_coordinate, second_coordinate);
                //println!("coordinates: {:?}", coordinates);
                inputs.push(coordinates);
                if x1 > max_x {
                    max_x = x1;
                }
                if x2 > max_x {
                    max_x = x2;
                }
                if y1 > max_y {
                    max_y = y1;
                }
                if y2 > max_y {
                    max_y = y2;
                }
            }
        }
        let inputs_copy = inputs.clone();
        println!("PART ONE:");
        println!("max coordinates: {:?},{:?}", max_x, max_y);

        let mut map: Vec<Vec<i32>> = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];
        //print_map(&map);
        for input in inputs {
            if input.0 .0 == input.1 .0 {
                //println!("GOOD: {:?}", input);
                for n in cmp::min(input.0 .1, input.1 .1)..cmp::max(input.0 .1, input.1 .1) + 1 {
                    map[input.0 .0 as usize][n as usize] += 1;
                }
            } else if input.0 .1 == input.1 .1 {
                //println!("GOOD: {:?}", input);
                for n in cmp::min(input.0 .0, input.1 .0)..cmp::max(input.0 .0, input.1 .0) + 1 {
                    map[n as usize][input.0 .1 as usize] += 1;
                }
            } else {
                //println!("nope: {:?}", input);
            }
        }
        //print_map(&map);
        let mut count_of_hot_spots: i32 = 0;
        for line in &map {
            for spot in line {
                if *spot >= 2 {
                    count_of_hot_spots += 1;
                }
            }
        }

        println!("count_of_hot_spots: {:?}", count_of_hot_spots);

        println!("PART TWO:");
        for input in inputs_copy {
            if input.0 .0 == input.1 .0 {
                //println!("nope: {:?}", input);
            } else if input.0 .1 == input.1 .1 {
                //println!("nope: {:?}", input);
            } else {
                //println!("GOOD: {:?}", input);
                let smaller_x_coordinate: (i32, i32);
                let larger_x_coordinate: (i32, i32);
                if input.0 .0 < input.1 .0 {
                    smaller_x_coordinate = input.0;
                    larger_x_coordinate = input.1;
                } else {
                    smaller_x_coordinate = input.1;
                    larger_x_coordinate = input.0;
                }
                let mut y = smaller_x_coordinate.1;
                for x in smaller_x_coordinate.0..larger_x_coordinate.0 + 1 {
                    map[x as usize][y as usize] += 1;
                    if larger_x_coordinate.1 > smaller_x_coordinate.1 {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                }
            }
        }
        //print_map(&map);
        count_of_hot_spots = 0;
        for line in &map {
            for spot in line {
                if *spot >= 2 {
                    count_of_hot_spots += 1;
                }
            }
        }
        println!("count_of_hot_spots: {:?}", count_of_hot_spots);
    }
}

fn print_map(map: &Vec<Vec<i32>>) {
    println!("MAP:");
    for line in map {
        println!("{:?}", line);
    }
}
