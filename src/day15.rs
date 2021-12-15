use crate::file;

const MAP_SIZE: usize = 100;
const EXTENSION_FACTOR: usize = 5;
const EXTENDED_MAP_SIZE: usize = MAP_SIZE * EXTENSION_FACTOR;

struct Cell {
    risk: usize,
    checked: bool,
    distance: usize,
}

pub fn solve() {
    println!("FIFTEENTH DAY");
    let mut map: Vec<Cell> = vec![];
    if let Ok(lines) = file::read_lines("inputs/input15.txt") {
        for line in lines {
            if let Ok(this_line_string) = line {
                for c in this_line_string.chars() {
                    let input_cell = Cell {
                        risk: c.to_digit(10).unwrap() as usize,
                        checked: false,
                        distance: 0,
                    };
                    map.push(input_cell);
                }
            }
        }
    }
    let mut extended_map = extend_map(&map);
    println!("PART ONE:");
    find_lowest_risk_route(map, false);
    println!("PART TWO:");
    find_lowest_risk_route(extended_map, true);

    /*
    FIFTEENTH DAY
    PART ONE:
    Calculating... 99.9%, iter: 9995
    lowest risk route sum risk: 388
    PART TWO:
    Calculating... 100.0%, iter: 249999
    lowest risk route sum risk: 2819
    Total time taken to run is 1599 seconds.
    */
}

fn print_map_distance(map: &Vec<Cell>, extended: bool) {
    for (i, cell) in map.iter().enumerate() {
        if extended {
            if i % EXTENDED_MAP_SIZE == 0 {
                println!("");
            }
        } else if i % MAP_SIZE == 0 {
            println!("");
        }
        print!("{:4}", cell.distance);
    }
    println!("");
}

fn print_map_risk(map: &Vec<Cell>, extended: bool) {
    for (i, cell) in map.iter().enumerate() {
        if extended {
            if i % EXTENDED_MAP_SIZE == 0 {
                println!("");
            }
        } else if i % MAP_SIZE == 0 {
            println!("");
        }
        print!("{:?}", cell.risk);
    }
    println!("");
}

fn get_neigbours(cell: usize, extended: bool) -> Vec<usize> {
    let map_size: usize;
    if extended {
        map_size = EXTENDED_MAP_SIZE;
    } else {
        map_size = MAP_SIZE;
    }
    let mut result: Vec<usize> = vec![];
    if cell == 0 {
        // top left
        result.push(1);
        result.push(map_size);
    } else if cell == map_size - 1 {
        // top right
        result.push(cell - 1);
        result.push(cell + map_size);
    } else if cell == map_size * (map_size - 1) {
        // bottom left
        result.push(cell - map_size);
        result.push(cell + 1);
    } else if cell == (map_size * map_size) - 1 {
        // bottom right
        result.push(cell - 1);
        result.push(cell - map_size);
    } else if cell / map_size == 0 {
        //top row
        result.push(cell - 1);
        result.push(cell + 1);
        result.push(cell + map_size);
    } else if cell / map_size == map_size - 1 {
        //bottom row
        result.push(cell - 1);
        result.push(cell + 1);
        result.push(cell - map_size);
    } else if cell % map_size == 0 {
        // leftmost column
        result.push(cell - map_size);
        result.push(cell + map_size);
        result.push(cell + 1);
    } else if cell % map_size == map_size - 1 {
        // leftmost column
        result.push(cell - map_size);
        result.push(cell + map_size);
        result.push(cell - 1);
    } else {
        // middle cell
        result.push(cell - map_size);
        result.push(cell + map_size);
        result.push(cell - 1);
        result.push(cell + 1);
    }
    result
}

fn get_index_of_min_distance_cell(map: &Vec<Cell>) -> usize {
    let mut result_index: usize = 0;
    let mut result_distance: usize = 0;
    let mut first_uncheked: bool = true;
    for (i, cell) in map.iter().enumerate() {
        if !cell.checked {
            if first_uncheked {
                result_index = i;
                result_distance = cell.distance;
                first_uncheked = false;
            } else if cell.distance != 0 && cell.distance < result_distance {
                result_index = i;
                result_distance = cell.distance;
            }
        }
    }
    result_index
}

fn find_lowest_risk_route(mut map: Vec<Cell>, extended: bool) -> usize {
    let map_size: usize;
    if extended {
        map_size = EXTENDED_MAP_SIZE;
    } else {
        map_size = MAP_SIZE;
    }
    let mut iter: usize = 0;
    let mut next_cell: usize = 0;
    while !map[map_size * map_size - 1].checked {
        map[next_cell].checked = true;
        let neigbours = get_neigbours(next_cell, extended);
        for neigbour in neigbours {
            if !map[neigbour].checked && map[neigbour].distance == 0 {
                map[neigbour].distance = map[next_cell].distance + map[neigbour].risk;
            }
        }
        let progress: f32 = iter as f32 / map.len() as f32;
        print!(
            "\rCalculating... {:.1}%, iter: {:?}",
            progress * 100 as f32,
            iter
        );
        next_cell = get_index_of_min_distance_cell(&map);
        iter += 1;
    }
    println!("");
    println!(
        "lowest risk route sum risk: {:?}",
        map[map_size * map_size - 1].distance
    );
    map[map_size * map_size - 1].distance
}

fn extend_map(map: &Vec<Cell>) -> Vec<Cell> {
    let mut extended_map: Vec<Cell> = vec![];
    let mut i: usize;
    let mut j: usize;
    let mut m_row: usize;
    let mut m_column: usize;
    let mut m: usize;
    for n in 0..EXTENDED_MAP_SIZE * EXTENDED_MAP_SIZE {
        i = n / MAP_SIZE % EXTENSION_FACTOR;
        j = n / (EXTENDED_MAP_SIZE * MAP_SIZE);
        m_row = n / EXTENDED_MAP_SIZE % MAP_SIZE;
        m_column = n % MAP_SIZE;
        m = m_row * MAP_SIZE + m_column;

        let input_cell = Cell {
            risk: (map[m].risk - 1 + i + j) % 9 + 1,
            checked: false,
            distance: 0,
        };
        extended_map.push(input_cell);
    }
    extended_map
}
