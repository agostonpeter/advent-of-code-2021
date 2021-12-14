use crate::file;

pub fn solve() {
    println!("THIRTEENTH DAY");
    let mut input_strings: Vec<String>;
    let mut inputs: Vec<(usize, usize)> = vec![];
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    if let Ok(lines) = file::read_lines("inputs/input13A.txt") {
        for line in lines {
            if let Ok(this_line_string) = line {
                input_strings = this_line_string.split(",").map(String::from).collect();

                let x: usize = input_strings[0].parse().unwrap();
                let y: usize = input_strings[1].parse().unwrap();
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
                inputs.push((x, y));
            }
        }
    }
    let mut paper: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    for input in &inputs {
        paper[input.1][input.0] += 1;
    }
    let mut instructions: Vec<(String, usize)> = vec![];
    if let Ok(lines) = file::read_lines("inputs/input13B.txt") {
        for line in lines {
            if let Ok(this_line_string) = line {
                input_strings = this_line_string.split("=").map(String::from).collect();
                let line: usize = input_strings[1].parse().unwrap();
                let instruction: (String, usize) = (input_strings[0].clone(), line);
                instructions.push(instruction);
            }
        }
    }

    for (i, instruction) in instructions.iter().enumerate() {
        if instruction.0 == "fold along y" {
            paper = fold_up(paper, instruction.1);
        } else {
            paper = fold_left(paper, instruction.1);
        }
        if i == 0 {
            println!("PART ONE:");
            let dots_after_first_fold: usize = count_dots(&paper);
            println!("Dots after first fold: {:?}", dots_after_first_fold);
        }
    }
    println!("PART TWO:");
    print_paper(&paper);
}

fn print_paper(paper: &Vec<Vec<usize>>) {
    let mut size = 0;
    print!("     ");
    for n in 0..paper[0].len() {
        print!("{:?}", n / 10);
    }
    println!("");
    print!("     ");
    for n in 0..paper[0].len() {
        print!("{:?}", n % 10);
    }
    println!("");
    for line in paper {
        print!("{:4} ", size);
        size += 1;
        for cell in line {
            print_cell(cell)
        }
        println!("");
    }
}
fn print_cell(cell: &usize) {
    if *cell > 0 {
        print!("#");
    } else {
        print!(".");
    }
}
fn count_dots(paper: &Vec<Vec<usize>>) -> usize {
    let mut result: usize = 0;
    for line in paper {
        for cell in line {
            if *cell > 0 {
                result += 1;
            }
        }
    }
    result
}

fn fold_up(paper: Vec<Vec<usize>>, folding_line: usize) -> Vec<Vec<usize>> {
    let size_x: usize = paper[0].len();
    let size_y: usize = paper.len();
    let mut y: usize = folding_line;
    let mut folded_paper: Vec<Vec<usize>> = vec![vec![0; size_x]; size_y - folding_line - 1];
    for y_folding in (folding_line + 1)..size_y {
        y -= 1;
        for x in 0..size_x {
            folded_paper[y][x] = paper[y_folding][x] + paper[y][x];
        }
    }
    folded_paper
}

fn fold_left(paper: Vec<Vec<usize>>, folding_column: usize) -> Vec<Vec<usize>> {
    let size_x: usize = paper[0].len();
    let size_y: usize = paper.len();
    let mut x: usize = folding_column;
    let mut folded_paper: Vec<Vec<usize>> = vec![vec![0; size_x - folding_column - 1]; size_y];
    for x_folding in (folding_column + 1)..size_x {
        x -= 1;
        for y in 0..size_y {
            folded_paper[y][x] = paper[y][x_folding] + paper[y][x];
        }
    }
    folded_paper
}
