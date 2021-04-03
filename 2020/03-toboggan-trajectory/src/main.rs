use aoc::argument_parser;
use aoc::reader;

fn main() {
    argument_parser::parse_and_run(part_1, part_2);
}

fn part_1() {
    let slope = (3, 1);
    println!(
        "Trees encountered: {}",
        count_trees_for_slope(slope)
    );
}

fn part_2() {
    let slopes: Vec<(usize, usize)> = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    let total_trees_encountered = slopes
        .into_iter()
        .map(|slope| count_trees_for_slope(slope))
        .reduce(|a, b| a * b);

    if let Some(s) = total_trees_encountered {
        println!("Sum: {}", s);
    } else {
        panic!("Unable to reduce sum of trees encountered");
    }
}

fn count_trees_for_slope(slope: (usize, usize)) -> u32 {
    let mut trees_encountered = 0;
    let map = reader::read_data();

    let map_max_height = map.len();

    let mut current_y: usize = 0;
    let mut current_x: usize = 0;

    while current_y < map_max_height {
        let cells: Vec<u8> = map[current_y].bytes().collect();

        if cells[current_x].eq(&b'#') {
            trees_encountered += 1;
        }

        // Move X (slope.0) cells to the right, indefinitely, before
        // moving Y (slope.1) rows down in the next iteration.
        current_x = (current_x + slope.0) % cells.len();
        current_y += slope.1;
    }

    return trees_encountered;
}
