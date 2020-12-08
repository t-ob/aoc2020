use std::iter::FromIterator;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn parse_line(line: &str) -> Result<(String, HashSet<(String, i64)>), String> {
    let mut parent_children = line.split(" bags contain ");

    if let (Some(parent_color), Some(child_colors)) =
        (parent_children.next(), parent_children.next())
    {
        if child_colors == "no other bags." {
            return Ok((parent_color.to_string(), HashSet::from_iter(vec![])));
        }

        let mut colors: HashSet<(String, i64)> = HashSet::new();

        for child_color in child_colors.split(",") {
            let mut tokens = child_color.split_ascii_whitespace();
            if let (Some(qty), Some(t1), Some(t2)) = (tokens.next(), tokens.next(), tokens.next()) {
                if let Ok(qty) = qty.parse::<i64>() {
                    colors.insert((format!("{} {}", t1, t2), qty));
                } else {
                    return Err(format!("Unable to parse string: {}", child_color));
                }
            }
        }

        return Ok((parent_color.to_string(), colors));
    }

    Err(format!("Unable to parse string: {}", line))
}

pub struct Graph {
    edge_list: HashMap<String, HashSet<(String, i64)>>,
}

impl Graph {
    pub fn new() -> Graph {
        let edge_list = HashMap::new();
        Graph { edge_list }
    }

    pub fn add_edge(&mut self, u: String, v: String, wgt: i64) {
        let edge_list = &mut self.edge_list;
        if !edge_list.contains_key(&u) {
            edge_list.insert(u.clone(), HashSet::new());
        }

        let neighbours = edge_list.get_mut(&u).unwrap();
        neighbours.insert((v.clone(), wgt));
    }

    pub fn get_neighbours(&self, u: &str) -> Option<HashSet<(String, i64)>> {
        if let Some(neighbours) = self.edge_list.get(u) {
            return Some(neighbours.clone());
        }
        None
    }

    pub fn reverse_edges(&self) -> Graph {
        let mut reversed_graph = Graph::new();

        for (u, vs) in self.edge_list.iter() {
            for (v, w) in vs.iter() {
                reversed_graph.add_edge(v.clone(), u.clone(), *w);
            }
        }

        reversed_graph
    }
}

impl FromStr for Graph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Self::new();

        for line in s.lines() {
            if let Ok((u, foo)) = parse_line(line.trim()) {
                for (v, wgt) in foo.iter() {
                    graph.add_edge(u.clone(), v.clone(), *wgt);
                }
            }
        }

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest(input, color, colors,
        case("light red bags contain 1 bright white bag, 2 muted yellow bags.", "light red", vec![("bright white", 1), ("muted yellow", 2)]),
        case("dark orange bags contain 3 bright white bags, 4 muted yellow bags.", "dark orange", vec![("bright white", 3), ("muted yellow", 4)]),
        case("bright white bags contain 1 shiny gold bag.", "bright white", vec![("shiny gold", 1)]),
        case("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.", "muted yellow", vec![("shiny gold", 2), ("faded blue", 9)]),
        case("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.", "shiny gold", vec![("dark olive", 1), ("vibrant plum", 2)]),
        case("dark olive bags contain 3 faded blue bags, 4 dotted black bags.", "dark olive", vec![("faded blue", 3), ("dotted black", 4)]),
        case("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.", "vibrant plum", vec![("faded blue", 5), ("dotted black", 6)]),
        case("faded blue bags contain no other bags.", "faded blue", vec![]),
        case("dotted black bags contain no other bags.", "dotted black", vec![])

    )]
    fn test_parse_line(input: &str, color: &str, colors: Vec<(&str, i64)>) {
        assert_eq!(
            parse_line(input),
            Ok((
                color.to_string(),
                HashSet::from_iter(colors.iter().map(|(x, y)| (x.to_string(), *y)))
            ))
        );
    }

    #[test]
    fn test_graph_parse() {
        let input = "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
dotted black bags contain no other bags.
faded blue bags contain no other bags.";

        let graph: Graph = input.parse().unwrap();

        assert_eq!(
            graph.get_neighbours(&"shiny gold".to_string()),
            Some(HashSet::from_iter(vec![
                ("dark olive".to_string(), 1),
                ("vibrant plum".to_string(), 2)
            ]))
        )
    }
}
