use crate::file;

pub fn solve() {
    println!("FOURTH DAY");
    if let Ok(lines) = file::read_lines("inputs/input04.txt") {
        let mut draws: Vec<String> = vec![];
        let mut line_count = 0;
        let mut this_board: Vec<String> = vec![];
        let mut boards: Vec<Vec<String>> = vec![vec![]];
        for line in lines {
            if let Ok(this_line_string) = line {
                if this_line_string.len() > 14 {
                    draws = this_line_string.split(",").map(String::from).collect();
                    //println!("{:?} len: {:?}", draws, draws.len());
                } else if this_line_string.len() > 0 {
                    let this_line: Vec<String> = this_line_string
                        .split_whitespace()
                        .map(String::from)
                        .collect();
                    this_board.extend(this_line);
                    line_count += 1;
                    if line_count == 5 {
                        //println!("{:?}", this_board);
                        boards.push(this_board);
                        line_count = 0;
                        this_board = vec![];
                    }
                }
            }
        }
        //println!("{:?}", boards);
        //println!("board_count {:?}", board_count);

        let mut result_boards = boards.clone();
        let mut bingo_count = 0;
        let mut bingo_sheet = vec![false; result_boards.len()];

        for draw in draws {
            for (i, board) in boards.iter().enumerate() {
                for (j, element) in board.iter().enumerate() {
                    if element == &draw {
                        //println!("BINGO number_string {:?}", number_string);
                        result_boards[i][j] = "X".to_string();
                    } else {
                        //println!("NOT number_string {:?}", number_string);
                    }
                }
            }
            for (i, board) in result_boards.iter().enumerate() {
                let mut row_results: [i32; 5] = [0, 0, 0, 0, 0];
                let mut column_results: [i32; 5] = [0, 0, 0, 0, 0];
                for (j, element) in board.iter().enumerate() {
                    if element == "X" {
                        row_results[j / 5] += 1;
                        column_results[j % 5] += 1;
                    }
                }
                let mut bingo = false;
                for result in row_results {
                    if result == 5 {
                        bingo = true;
                        break;
                    }
                }
                for result in column_results {
                    if result == 5 {
                        bingo = true;
                        break;
                    }
                }
                if bingo {
                    if bingo_count == 0 {
                        println!("FIRST BINGO!");
                        let mut score: i32 = 0;
                        for element in board {
                            if element != "X" {
                                let element_int: i32 = element.parse().unwrap();
                                score += element_int;
                            }
                        }
                        let draw_int: i32 = draw.parse().unwrap();
                        score *= draw_int;
                        println!("{:?}", board);
                        println!("Score of the winning board: {:?}", score);
                    }
                    if bingo_sheet[i] == false {
                        bingo_sheet[i] = true;
                        bingo_count += 1;

                        if bingo_count == result_boards.len() - 1 {
                            // TODO: there is an empty "board" at the beginning reuslt_boards
                            println!("LAST BINGO!");
                            println!("{:?}", bingo_count);

                            let mut score: i32 = 0;
                            for element in board {
                                if element != "X" {
                                    let element_int: i32 = element.parse().unwrap();
                                    score += element_int;
                                }
                            }
                            let draw_int: i32 = draw.parse().unwrap();
                            score *= draw_int;
                            println!("{:?}", board);
                            println!("Score of the last winning board: {:?}", score);
                        }
                    }
                }
            }
        }
        //println!("{:?}", result_boards);
    }
}
