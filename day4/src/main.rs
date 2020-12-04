use std::iter::FromIterator;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use regex::Regex;

trait ValidatesPassport {
    fn validate(&self, passport: &HashMap<String, String>) -> bool;
}

struct Part1Validator {
    required_keys: HashSet<String>,
}

impl Part1Validator {
    pub fn new(required_keys: HashSet<String>) -> Part1Validator {
        Part1Validator { required_keys }
    }
}

impl ValidatesPassport for Part1Validator {
    fn validate(&self, passport: &HashMap<String, String>) -> bool {
        let keys: HashSet<String> = HashSet::from_iter(passport.keys().map(|k| k.to_string()));
        self.required_keys.is_subset(&keys)
    }
}

enum Field {
    Year(i32, i32),
    Measure(String, i32, i32),
    Hex,
    OneOf(HashSet<String>),
    Number(usize),
}

struct Part2Validator {
    required_keys: HashMap<String, Vec<Field>>,
}

impl Part2Validator {
    pub fn new(required_keys: HashMap<String, Vec<Field>>) -> Part2Validator {
        Part2Validator { required_keys }
    }
}

impl ValidatesPassport for Part2Validator {
    fn validate(&self, passport: &HashMap<String, String>) -> bool {
        let keys: HashSet<String> = HashSet::from_iter(passport.keys().map(|k| k.to_string()));
        for (key, fields) in self.required_keys.iter() {
            if !keys.contains(key) {
                return false;
            }
            let val = passport.get(key).unwrap();
            let valid = fields
                .iter()
                .map(|field| match field {
                    Field::Year(min, max) => {
                        if let Ok(year) = val.parse::<i32>() {
                            return *min <= year && year <= *max;
                        }
                        false
                    }
                    Field::Measure(suffix, min, max) => {
                        let chars = val.chars();
                        let (bbb, ccc): (String, String) = chars.partition(|c| c.is_numeric());
                        if ccc != *suffix {
                            return false;
                        }
                        if let Ok(bbbb) = bbb.parse::<i32>() {
                            return *min <= bbbb && bbbb <= *max;
                        }

                        false
                    }
                    Field::Hex => {
                        if val.len() != 7 {
                            return false;
                        }
                        let mut chars = val.chars();
                        if chars.next().unwrap() != '#' {
                            return false;
                        }
                        for char in chars {
                            if !"0123456789abcdef".contains(char) {
                                return false;
                            }
                        }

                        true
                    }
                    Field::OneOf(items) => items.contains(val),
                    Field::Number(digits) => {
                        if val.len() != *digits {
                            return false;
                        }
                        for c in val.chars() {
                            if !c.is_numeric() {
                                return false;
                            }
                        }
                        true
                    }
                })
                .fold(false, |a, b| a || b);

            if !valid {
                return false;
            }
        }

        true
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut part_1_valid_passports = 0;
    let part_1_validator = Part1Validator::new(HashSet::from_iter(
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(|s| s.to_string()),
    ));

    let mut part_2_required_fields: HashMap<String, Vec<Field>> = HashMap::new();
    part_2_required_fields.insert(String::from("byr"), vec![Field::Year(1920, 2002)]);
    part_2_required_fields.insert(String::from("iyr"), vec![Field::Year(2010, 2020)]);
    part_2_required_fields.insert(String::from("eyr"), vec![Field::Year(2020, 2030)]);
    part_2_required_fields.insert(
        String::from("hgt"),
        vec![
            Field::Measure(String::from("cm"), 150, 193),
            Field::Measure(String::from("in"), 59, 76),
        ],
    );
    part_2_required_fields.insert(String::from("hcl"), vec![Field::Hex]);
    part_2_required_fields.insert(
        String::from("ecl"),
        vec![Field::OneOf(HashSet::from_iter(
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .map(|s| s.to_string()),
        ))],
    );
    part_2_required_fields.insert(String::from("pid"), vec![Field::Number(9)]);

    let mut part_2_valid_passports = 0;
    let part_2_validator = Part2Validator::new(part_2_required_fields);

    let passport_separator = Regex::new(r"\n\n").unwrap();
    let field_separator = Regex::new(r"\s+").unwrap();
    for passport_data in passport_separator.split(&buffer) {
        let mut passport: HashMap<String, String> = HashMap::new();
        for kv_pair in field_separator.split(passport_data) {
            let mut kv_iter = kv_pair.split(":");
            match (&kv_iter.next(), &kv_iter.next()) {
                (Some(k), Some(v)) => {
                    passport.insert(k.to_string(), v.to_string());
                }
                _ => break,
            }
        }

        if part_1_validator.validate(&passport) {
            part_1_valid_passports += 1;
        }

        if part_2_validator.validate(&passport) {
            part_2_valid_passports += 1;
        }
    }

    println!("{}", part_1_valid_passports);
    println!("{}", part_2_valid_passports);

    Ok(())
}
