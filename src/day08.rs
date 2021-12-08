use crate::file;

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/
//        digit: 0 1 2 3 4 5 6 7 8 9
// No. segments: 6 2 5 5 4 5 6 3 7 6

pub fn solve() {
    println!("EIGHTH DAY");
    if let Ok(lines) = file::read_lines("inputs/input08.txt") {
        let mut input_strings: Vec<String>;
        let mut inputs: Vec<(Vec<String>, Vec<String>)> = vec![];

        for line in lines {
            if let Ok(this_line_string) = line {
                input_strings = this_line_string.split("|").map(String::from).collect();
                let signal_patterns: Vec<String> = input_strings[0]
                    .split_whitespace()
                    .map(String::from)
                    .collect();
                let output_value: Vec<String> = input_strings[1]
                    .split_whitespace()
                    .map(String::from)
                    .collect();
                inputs.push((signal_patterns, output_value));
            }
        }
        let inputs_2 = inputs.clone();
        //println!("{:?}", inputs);
        println!("PART ONE:");
        let mut output_value_count = 0;
        for input in inputs {
            for output_value in input.1 {
                if output_value.len() == 2 // digit 1
                    || output_value.len() == 4 // digit 4
                    || output_value.len() == 3 // digit 7
                    || output_value.len() == 7
                // digit 8
                {
                    output_value_count += 1;
                }
            }
        }
        println!("No of 1,4,7,8 digits: {:?}", output_value_count);

        println!("PART TWO:");
        let mut result: i32 = 0;
        for input in inputs_2 {
            let mut segment_counts: [i32; 7] = [0, 0, 0, 0, 0, 0, 0];
            //println!("{:?}", input.0);
            let mut cf_chars: Vec<char> = vec![];
            let mut bcdf_chars: Vec<char> = vec![];
            for signal in input.0 {
                if signal.len() == 2 {
                    cf_chars = signal.chars().collect();
                    //println!("cf: {:?}", cf_chars);
                } else if signal.len() == 4 {
                    bcdf_chars = signal.chars().collect();
                    //println!("bcdf: {:?}", bcdf_chars);
                }
                for char in signal.chars() {
                    segment_counts[get_segment_number(char)] += 1;
                }
            }
            //println!("segment_counts: {:?}", segment_counts);
            let mut conversion_map: [char; 7] = ['-', '-', '-', '-', '-', '-', '-'];
            for (i, segment_count) in segment_counts.iter().enumerate() {
                if *segment_count == 8 {
                    if cf_chars.contains(&get_segment_char(i)) {
                        conversion_map[get_segment_number('c')] = get_segment_char(i);
                    } else {
                        conversion_map[get_segment_number('a')] = get_segment_char(i);
                    }
                } else if *segment_count == 7 {
                    if bcdf_chars.contains(&get_segment_char(i)) {
                        conversion_map[get_segment_number('d')] = get_segment_char(i);
                    } else {
                        conversion_map[get_segment_number('g')] = get_segment_char(i);
                    }
                } else if *segment_count == 6 {
                    conversion_map[get_segment_number('b')] = get_segment_char(i);
                } else if *segment_count == 4 {
                    conversion_map[get_segment_number('e')] = get_segment_char(i);
                } else if *segment_count == 9 {
                    conversion_map[get_segment_number('f')] = get_segment_char(i);
                }
            }
            //println!("conversion_map: {:?}", conversion_map);
            //println!("output_segments: {:?}", input.1);
            let mut decimal = 1000;
            for output_segments in input.1 {
                let mut correct_segments = String::new();
                let mut i = 0;
                for segment in conversion_map {
                    if output_segments.contains(segment) {
                        correct_segments.push(get_segment_char(i))
                    }
                    i += 1;
                }
                //println!("correct_segments: {:?}", correct_segments);
                result += get_number(&correct_segments) * decimal;
                decimal /= 10;
            }
        }
        println!("result: {:?}", result);
    }
}

fn get_segment_char(number: usize) -> char {
    match number {
        0 => return 'a',
        1 => return 'b',
        2 => return 'c',
        3 => return 'd',
        4 => return 'e',
        5 => return 'f',
        6 => return 'g',
        _ => return 'x',
    }
}
fn get_segment_number(character: char) -> usize {
    match character {
        'a' => return 0,
        'b' => return 1,
        'c' => return 2,
        'd' => return 3,
        'e' => return 4,
        'f' => return 5,
        'g' => return 6,
        _ => return 99,
    }
}
fn get_number(segment_string: &str) -> i32 {
    match segment_string {
        "abcefg" => return 0,
        "cf" => return 1,
        "acdeg" => return 2,
        "acdfg" => return 3,
        "bcdf" => return 4,
        "abdfg" => return 5,
        "abdefg" => return 6,
        "acf" => return 7,
        "abcdefg" => return 8,
        "abcdfg" => return 9,
        _ => return 99999,
    }
}
