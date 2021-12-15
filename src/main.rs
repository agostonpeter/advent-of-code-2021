use chrono::Utc;

//mod day01;
//mod day02;
//mod day03;
//mod day04;
//mod day05;
//mod day06;
//mod day07;
//mod day08;
//mod day13;
//mod day14;
mod day15;
mod file;

fn main() {
    let start_time = Utc::now().time();
    //day01::solve();
    //day02::solve();
    //day03::solve();
    //day04::solve();
    //day05::solve();
    //day06::solve();
    //day07::solve();
    //day08::solve();
    //day13::solve();
    //day14::solve();
    day15::solve();
    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run is {} seconds.", diff.num_seconds());
}
