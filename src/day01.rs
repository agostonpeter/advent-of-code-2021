use crate::file;

pub fn solve() {
    println!("FIRST DAY");
    if let Ok(lines) = file::read_lines("inputs/input01.txt") {
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
}
