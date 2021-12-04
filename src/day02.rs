use crate::file;

pub fn solve() {
    println!("SECOND DAY");
    if let Ok(lines) = file::read_lines("inputs/input02.txt") {
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
}
