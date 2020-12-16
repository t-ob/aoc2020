use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Valid(HashSet<String>),
    Invalid(u64),
}

pub struct FieldRanges {
    ranges: HashMap<String, Vec<RangeInclusive<u64>>>,
}

impl FieldRanges {
    pub fn new(ranges: HashMap<String, Vec<RangeInclusive<u64>>>) -> FieldRanges {
        FieldRanges { ranges }
    }

    pub fn validate(&self, ticket: &Ticket) -> Vec<Status> {
        let mut validity = Vec::new();

        for ticket_field in ticket.fields() {
            let mut candidate_fields = HashSet::new();
            for (field, ranges) in self.ranges.iter() {
                for range in ranges.iter() {
                    if range.contains(ticket_field) {
                        candidate_fields.insert(field.clone());
                    }
                }
            }
            if candidate_fields.is_empty() {
                validity.push(Status::Invalid(*ticket_field))
            } else {
                validity.push(Status::Valid(candidate_fields))
            }
        }

        validity
    }
}

impl FromStr for FieldRanges {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = HashMap::new();
        for line in s.lines() {
            let mut line_iter = line.split(": ").into_iter();
            let field = line_iter.next().expect("Expected line of the form <field name>: A-B or C-D");
            let field_ranges = line_iter
                .next()
                .expect("Expected line of the form <field name>: A-B or C-D")
                .split(" or ")
                .into_iter()
                .map(|s| {
                    let mut range_iter = s.split('-').into_iter();
                    let a = range_iter.next().expect("Expected range of the form A-B").parse().expect("Expected integer");
                    let b = range_iter.next().expect("Expected range of the form A-B").parse().expect("Expected integer");
                    a..=b
                })
                .collect();
            ranges.insert(field.to_string(), field_ranges);
        }

        Ok(FieldRanges { ranges })
    }
}

pub struct Ticket {
    fields: Vec<u64>,
}

impl Ticket {
    pub fn new(fields: &[u64]) -> Ticket {
        let fields = Vec::from(fields);
        Ticket { fields }
    }
    pub fn fields(&self) -> &Vec<u64> {
        &self.fields
    }
}

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split(",")
            .map(|t| t.parse().expect("Unable to parse int"))
            .collect();

        Ok(Ticket { fields })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let mut ranges = HashMap::new();
        ranges.insert("class".to_string(), vec![1..=3, 5..=7]);
        ranges.insert("row".to_string(), vec![6..=11, 33..=44]);
        ranges.insert("seat".to_string(), vec![13..=40, 45..=50]);

        let fields = FieldRanges::new(ranges);

        assert_eq!(
            fields.validate(&Ticket::new(&[7, 3, 47])),
            vec![
                Status::Valid(
                    ["row".to_string(), "class".to_string()]
                        .iter()
                        .cloned()
                        .collect()
                ),
                Status::Valid(["class".to_string()].iter().cloned().collect()),
                Status::Valid(["seat".to_string()].iter().cloned().collect()),
            ]
        );

        assert_eq!(
            fields.validate(&Ticket::new(&[40, 4, 50])),
            vec![
                Status::Valid(
                    ["row".to_string(), "seat".to_string()]
                        .iter()
                        .cloned()
                        .collect()
                ),
                Status::Invalid(4),
                Status::Valid(["seat".to_string()].iter().cloned().collect()),
            ]
        );

        assert_eq!(
            fields.validate(&Ticket::new(&[55, 2, 20])),
            vec![
                Status::Invalid(55),
                Status::Valid(["class".to_string()].iter().cloned().collect()),
                Status::Valid(["seat".to_string()].iter().cloned().collect()),
            ]
        );

        assert_eq!(
            fields.validate(&Ticket::new(&[38, 6, 12])),
            vec![
                Status::Valid(
                    ["row".to_string(), "seat".to_string()]
                        .iter()
                        .cloned()
                        .collect()
                ),
                Status::Valid(
                    ["row".to_string(), "class".to_string()]
                        .iter()
                        .cloned()
                        .collect()
                ),
                Status::Invalid(12),
            ]
        );
    }
}
