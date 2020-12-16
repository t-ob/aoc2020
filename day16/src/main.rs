use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use day16::{FieldRanges, Status, Ticket};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut input_groups = buffer.split("\n\n").into_iter();

    let ticket_field_ranges: FieldRanges = input_groups.next().unwrap().parse().unwrap();

    let my_ticket: Ticket = input_groups
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let other_tickets = input_groups
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.parse::<Ticket>().unwrap());

    let mut part_1_sum = 0;
    let mut part_2_candidate_fields: Option<Vec<(usize, HashSet<String>)>> = None;

    for other_ticket in other_tickets {
        let mut invalid = false;
        let mut other_ticket_candidate_fields: Vec<(usize, HashSet<String>)> = Vec::new();
        for (idx, validity) in ticket_field_ranges
            .validate(&other_ticket)
            .iter()
            .enumerate()
        {
            match validity {
                Status::Valid(candidate_fields) => {
                    other_ticket_candidate_fields.push((idx, candidate_fields.clone()))
                }
                Status::Invalid(field_error) => {
                    invalid = true;
                    part_1_sum += field_error
                }
            }
        }

        match (&part_2_candidate_fields, invalid) {
            (None, false) => part_2_candidate_fields = Some(other_ticket_candidate_fields),
            (Some(candidate_fields), false) => {
                part_2_candidate_fields = Some(
                    candidate_fields.iter()
                        .zip(other_ticket_candidate_fields.iter())
                        .map(|((idx, accumulated_intersection), (_, candidate_fields))| {
                            (
                                *idx,
                                accumulated_intersection
                                    .intersection(candidate_fields)
                                    .cloned()
                                    .collect(),
                            )
                        })
                        .collect(),
                )
            }
            _ => continue,
        }
    }

    println!("{}", part_1_sum);

    if let Some(ref mut part_2_candidate_fields) = part_2_candidate_fields {
        part_2_candidate_fields.sort_by_key(|(_, candidate_fields)| candidate_fields.len());

        let mut fields = HashMap::new();
        let mut seen_fields = HashSet::new();

        for (idx, candidate_fields) in part_2_candidate_fields.iter() {
            let next_field = candidate_fields.difference(&seen_fields).next().unwrap();
            fields.insert(idx, next_field.clone());
            seen_fields.extend(candidate_fields.clone());
        }

        let my_ticket_fields = my_ticket.fields();
        let mut part_2_product = 1;
        for (idx, field) in fields {
            if field.starts_with("departure") {
                part_2_product *= my_ticket_fields[*idx];
            }
        }

        println!("{}", part_2_product);
    }

    Ok(())
}
