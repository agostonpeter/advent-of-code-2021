use crate::file;

pub fn solve() {
    println!("FOURTEENTH DAY");
    println!("PART ONE:");
    let mut input_strings: Vec<String>;
    let mut pair_insertions: Vec<(String, String, String)> = vec![];
    if let Ok(lines) = file::read_lines("inputs/input14B.txt") {
        for line in lines {
            if let Ok(this_line_string) = line {
                input_strings = this_line_string.split(" -> ").map(String::from).collect();
                let mut new_pair_1 = String::from(input_strings[0].chars().nth(0).unwrap());
                new_pair_1.push_str(&input_strings[1]);
                let mut new_pair_2 = String::from(input_strings[1].clone());
                new_pair_2.push(input_strings[0].chars().nth(1).unwrap());
                pair_insertions.push((input_strings[0].clone(), new_pair_1, new_pair_2));
            }
        }
    }
    println!("pair_insertions: {:?}", pair_insertions);
    let mut pair_counts: Vec<(String, usize)> = vec![];

    for pair_insertion in pair_insertions.iter_mut() {
        pair_counts.push((pair_insertion.0.clone(), 0));
    }
    let mut input_pairs: Vec<String> = vec![];
    let mut first_element: char = '_';
    let mut last_element: char = '_';
    if let Ok(lines) = file::read_lines("inputs/input14A.txt") {
        for line in lines {
            if let Ok(this_line_string) = line {
                println!("polymer template: {:?}", this_line_string);
                let mut prev_c = '_';
                for (i, c) in this_line_string.chars().enumerate() {
                    if i == 0 {
                        first_element = c;
                    } else {
                        let mut pair = String::from(prev_c);
                        pair.push(c);
                        input_pairs.push(pair);
                    }
                    prev_c = c;
                    last_element = c;
                }
            }
        }
    }

    for pair_count in pair_counts.iter_mut() {
        for pair in &input_pairs {
            if pair_count.0 == *pair {
                pair_count.1 += 1;
            }
        }
    }
    for n in 1..11 {
        println!("STEP: {:?}", n);
        pair_counts = single_polymerization_step(&pair_counts, &pair_insertions);
        //print_pair_counts(&pair_counts);
    }
    count_elements(&pair_counts, first_element, last_element);

    println!("PART TWO:");
    for n in 11..41 {
        println!("STEP: {:?}", n);
        pair_counts = single_polymerization_step(&pair_counts, &pair_insertions);
        //print_pair_counts(&pair_counts);
    }
    count_elements(&pair_counts, first_element, last_element);
}

fn print_pair_counts(pair_counts: &Vec<(String, usize)>) {
    println!("Pair counts:");
    for pair_count in pair_counts {
        println!("{:?}", pair_count);
    }
}

fn single_polymerization_step(
    pair_counts: &Vec<(String, usize)>,
    pair_insertions: &Vec<(String, String, String)>,
) -> Vec<(String, usize)> {
    let mut new_pairs: Vec<(String, usize)> = vec![];
    for pair_insertion in pair_insertions {
        new_pairs.push((pair_insertion.0.clone(), 0));
    }
    for pair_count in pair_counts {
        if pair_count.1 > 0 {
            for pair_insertion in pair_insertions {
                if pair_count.0 == pair_insertion.0 {
                    for new_pair_count in new_pairs.iter_mut() {
                        if new_pair_count.0 == pair_insertion.1 {
                            new_pair_count.1 += pair_count.1;
                        }
                        if new_pair_count.0 == pair_insertion.2 {
                            new_pair_count.1 += pair_count.1;
                        }
                    }
                    break;
                }
            }
        }
    }
    new_pairs
}

fn count_elements(pair_counts: &Vec<(String, usize)>, first_element: char, last_element: char) {
    let mut elements: Vec<(char, usize)> = vec![(first_element, 1), (last_element, 1)];
    for pair_count in pair_counts {
        for c in pair_count.0.chars() {
            let mut found: bool = false;
            let mut new_element: (char, usize) = ('_', 0);
            for element in elements.iter_mut() {
                if element.0 == c {
                    found = true;
                    element.1 += pair_count.1;
                    break;
                }
                new_element = (c, pair_count.1);
            }
            if !found {
                elements.push(new_element);
            }
        }
    }
    let mut most_common_element = elements[0].1 / 2;
    let mut least_common_element = elements[0].1 / 2;
    for element in elements.iter_mut() {
        element.1 = element.1 / 2;

        if element.1 > most_common_element {
            most_common_element = element.1;
        }
        if element.1 < least_common_element {
            least_common_element = element.1;
        }
    }
    println!("elements: {:?}", elements);
    println!("most_common_element: {:?}", most_common_element);
    println!("least_common_element: {:?}", least_common_element);
    println!(
        "diff between most and least common element number: {:?}",
        most_common_element - least_common_element
    );
}
