use std::env;
use std::fs;

fn main() {
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

fn part_1() {
    let auth = SledRentalAuth::new();
    println!(
        "Number of valid passwords: {}",
        get_valid_passwords(auth)
    );
}

fn part_2() {
    let auth = TobogganCorporateAuth::new();
    println!(
        "Number of valid passwords: {}",
        get_valid_passwords(auth)
    );
}

fn read_policies_and_passwords() -> Vec<String> {
    let data = fs::read_to_string("data.txt")
        .expect("Unable to read data!");

    let policies_and_passwords: Vec<String> = data
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    policies_and_passwords
}

fn get_valid_passwords(auth: impl Auth) -> i32 {
    let mut valid_passwords = 0;
    // We assume that every line is of the following format:
    // 18-20 f: ffffffffffffffmxftfv
    for s in read_policies_and_passwords() {
        let policy_and_password: Vec<&str> = s.split(":").collect();

        let policy = Policy::new(policy_and_password[0].trim());
        let password = policy_and_password[1].trim();

        if auth.validates(policy, password) {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

#[derive(Debug, Copy, Clone)]
struct Policy {
    letter: char,
    min_occurrences: usize,
    max_occurrences: usize
}

impl Policy {
    pub fn new(s: &str) -> Self {
        let policy_parts: Vec<&str> = s.split_whitespace().collect();
        let min_max_occurrences: Vec<&str> = policy_parts[0].trim().split("-").collect();
        let letter = match policy_parts[1].trim().chars().next() {
            Some(c) => c,
            _ => panic!("Failed to get next char from {}", policy_parts[1].trim())
        };

        let min_occurrences = match min_max_occurrences[0].trim().parse::<usize>() {
            Ok(n) => n,
            Err(error) => panic!("Cannot parse {} into usize: {:?}", min_max_occurrences[0].trim(), error)
        };
        let max_occurrences = match min_max_occurrences[1].trim().parse::<usize>() {
            Ok(n) => n,
            Err(error) => panic!("Cannot parse {} into usize: {:?}", min_max_occurrences[1].trim(), error)
        };

        Policy {
            letter,
            min_occurrences,
            max_occurrences
        }
    }
}

trait Auth {
    fn validates(&self, policy: Policy, password: &str) -> bool;
}

struct SledRentalAuth {

}

impl SledRentalAuth {
    pub fn new() -> Self {
        SledRentalAuth {

        }
    }
}

impl Auth for SledRentalAuth {
    fn validates(&self, policy: Policy, password: &str) -> bool {
        let occurrences_in_password = password.matches(policy.letter).count();
        let is_valid_password = occurrences_in_password >= policy.min_occurrences && occurrences_in_password <= policy.max_occurrences;
        is_valid_password
    }
}

struct TobogganCorporateAuth {

}

impl TobogganCorporateAuth {
    pub fn new() -> Self {
        TobogganCorporateAuth {

        }
    }
}

impl Auth for TobogganCorporateAuth {
    fn validates(&self, policy: Policy, password: &str) -> bool {
        // In the Toggoban Corporate auth, the policy letter must occur only once
        // in the given indexes (min_occurrences, max_occurrences). Also, this
        // auth does not have zero index. Meaning that accessing any letter at position n
        // in the password must be n - 1;
        let position_1 = policy.min_occurrences - 1;
        let position_2 = policy.max_occurrences - 1;

        let password_chars: Vec<char> = password.chars().collect();

        let position_1_char = password_chars[position_1];
        let position_2_char = password_chars[position_2];

        let position_1_matches = policy.letter.eq(&position_1_char);
        let position_2_matches = policy.letter.eq(&position_2_char);

        // There can only be one match.
        if position_1_matches && position_2_matches {
            return false;
        }

        if position_1_matches || position_2_matches {
            return true;
        }

        false
    }
}
