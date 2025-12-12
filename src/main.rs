mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <day>");
        println!("Example: cargo run 1");
        return;
    }

    match args[1].as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        "6" => day6::run(),
        "7" => day7::run(),
        "8" => day8::run(),
        "9" => day9::run(),
        "10" => day10::run(),
        "11" => day11::run(),
        "12" => day12::run(),
        "all" => {
            day1::run();
            day2::run();
            day3::run();
            day4::run();
            day5::run();
            day6::run();
            day7::run();
            day8::run();
            day9::run();
            day10::run();
            day11::run();
        }
        _ => println!("'{}' is not a valid day.", args[1]),
    }
}
