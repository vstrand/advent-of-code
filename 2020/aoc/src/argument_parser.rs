use std::env;

pub fn parse_and_run(part_1: fn() -> (), part_2: fn() -> ()) {
    let args: Vec<String> = env::args().collect();
    let mut part = "1";

    if args.len() > 1 {
        // argument given as --part=<part>
        let parts: Vec<&str> = args[1].split("=").collect();
        part = parts[1];
    }

    println!("Part #{}:", part);

    if part == "1" {
        part_1();
    } else {
        part_2();
    }
}
