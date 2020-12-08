use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use day7::Graph;

fn subtree_weight(graph: &Graph, root: &str, seen: &mut HashMap<String, i64>) -> i64 {
    let mut result = 0;

    if let Some(neighbours) = graph.get_neighbours(root) {
        for (neighbour, weight) in neighbours {
            let neighbour_result: i64;
            if let Some(neighbour_subtree_weight) = seen.get(&neighbour) {
                neighbour_result = *neighbour_subtree_weight;
            } else {
                neighbour_result = subtree_weight(graph, &neighbour, seen);
                seen.insert(neighbour, neighbour_result);
            }
            result += weight + weight * neighbour_result;
        }
    }

    result
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if let Ok(graph_part_2) = buffer.parse::<Graph>() {
        let graph_part_1 = graph_part_2.reverse_edges();

        // Part 1
        let mut valid_colors = 0;

        let mut stack = vec!["shiny gold".to_string()];
        let mut seen_part_1: HashSet<String> = HashSet::new();
        seen_part_1.insert("shiny gold".to_string());
        while stack.len() > 0 {
            if let Some(neighbour) = stack.pop() {
                if let Some(colors) = graph_part_1.get_neighbours(&neighbour) {
                    for (color, _) in colors {
                        if seen_part_1.contains(&color) {
                            continue;
                        }
                        stack.push(color.clone());
                        seen_part_1.insert(color.clone());
                        valid_colors += 1;
                    }
                }
            }
        }

        println!("{}", valid_colors);

        // Part 2
        let mut seen_part_2: HashMap<String, i64> = HashMap::new();
        println!(
            "{}",
            subtree_weight(&graph_part_2, "shiny gold", &mut seen_part_2)
        )
    }

    Ok(())
}
