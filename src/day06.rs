use crate::file;

pub fn solve() {
    println!("SIXTH DAY");
    if let Ok(lines) = file::read_lines("inputs/input06.txt") {
        let mut input_strings: Vec<String>;
        let mut school_of_fish: Vec<u8> = vec![];
        let mut daily_fish_count: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        for line in lines {
            if let Ok(this_line_string) = line {
                input_strings = this_line_string.split(",").map(String::from).collect();
                for input_string in input_strings {
                    let input_int: usize = input_string.parse().unwrap();
                    let input_uint: u8 = input_string.parse().unwrap();
                    daily_fish_count[input_int] += 1 as u64;
                    school_of_fish.push(input_uint)
                }
            }
        }
        println!("PART ONE:"); // result: 379414
        every_fish_counts(school_of_fish);
        println!("PART TWO:"); // result: 1705008653296
        of_course_not_as_simple_as_part_one(daily_fish_count);
    }
}

fn every_fish_counts(mut school_of_fish: Vec<u8>) {
    /* let mut school_of_fish: Vec<u8> = vec![
        3, 4, 1, 1, 5, 1, 3, 1, 1, 3, 5, 1, 1, 5, 3, 2, 4, 2, 2, 2, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1,
        3, 1, 1, 5, 4, 1, 1, 1, 4, 1, 1, 1, 1, 2, 3, 2, 5, 1, 5, 1, 2, 1, 1, 1, 4, 1, 1, 1, 1, 3,
        1, 1, 3, 1, 1, 1, 1, 1, 1, 2, 3, 4, 2, 1, 3, 1, 1, 2, 1, 1, 2, 1, 5, 2, 1, 1, 1, 1, 1, 1,
        4, 1, 1, 1, 1, 5, 1, 4, 1, 1, 1, 3, 3, 1, 3, 1, 3, 1, 4, 1, 1, 1, 1, 1, 4, 5, 1, 1, 3, 2,
        2, 5, 5, 4, 3, 1, 2, 1, 1, 1, 4, 1, 3, 4, 1, 1, 1, 1, 2, 1, 1, 3, 2, 1, 1, 1, 1, 1, 4, 1,
        1, 1, 4, 4, 5, 2, 1, 1, 1, 1, 1, 2, 4, 2, 1, 1, 1, 2, 1, 1, 2, 1, 5, 1, 5, 2, 5, 5, 1, 1,
        3, 1, 4, 1, 1, 1, 1, 1, 1, 1, 4, 1, 1, 4, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 5, 1, 1, 3, 5,
        1, 1, 5, 5, 3, 5, 3, 4, 1, 1, 1, 3, 1, 1, 3, 1, 1, 1, 1, 1, 1, 5, 1, 3, 1, 5, 1, 1, 4, 1,
        3, 1, 1, 1, 2, 1, 1, 1, 2, 1, 5, 1, 1, 1, 1, 4, 1, 3, 2, 3, 4, 1, 3, 5, 3, 4, 1, 4, 4, 4,
        1, 3, 2, 4, 1, 4, 1, 1, 2, 1, 3, 1, 5, 5, 1, 5, 1, 1, 1, 5, 2, 1, 2, 3, 1, 4, 3, 3, 4, 3,
    ];*/
    //my input

    //let mut school_of_fish: Vec<u64> = vec![3, 4, 3, 1, 2]; // example
    let days = 80;
    for _n in 0..days {
        let mut new_fish: u32 = 0;
        for fish in &mut school_of_fish {
            if *fish == 0 as u8 {
                *fish = 7;
                new_fish += 1;
            }
            *fish -= 1;
        }
        for _i in 0..new_fish {
            school_of_fish.push(8);
        }
    }
    println!("days: {:?}, fish: {:?}", days, school_of_fish.len());
}

fn of_course_not_as_simple_as_part_one(mut daily_fish_count: [u64; 9]) {
    //let mut daily_fish_count: [u64; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0]; //example

    let days = 256;
    for _n in 0..days {
        let number_of_zero_fish = daily_fish_count[0];
        for i in 0..8 {
            daily_fish_count[i] = daily_fish_count[i + 1];
        }
        daily_fish_count[6] += number_of_zero_fish;
        daily_fish_count[8] = number_of_zero_fish;
    }
    let mut number_of_fish: u64 = 0;
    for fish in daily_fish_count {
        number_of_fish += fish;
    }
    println!("days: {:?}, fish: {:?}", days, number_of_fish);
    println!("per day: {:?}", daily_fish_count);
}
