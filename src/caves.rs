use std::collections::{HashMap, HashSet};

fn is_lowercase(value: &String) -> bool {
    value.to_lowercase() == *value
}

pub struct Caves<'a> {
    connections: &'a HashMap<String, Vec<String>>,
    double_slot: Option<String>,
}

impl<'a> Caves<'a> {
    pub fn new(connections: &'a HashMap<String, Vec<String>>) -> Self {
        Self {
            connections,
            double_slot: None,
        }
    }

    pub fn solve(
        &self,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        two_visits: Option<String>,
    ) -> u32 {
        let node = path.last().unwrap().clone();
        if node == "end" {
            path.pop();
            return 1;
        }
        println!("{:?}", path);
        let mut total = 0;
        if let Some(adj) = self.connections.get(&node) {
            for adj_node in adj {
                let mut new_visit = two_visits.clone();
                if is_lowercase(adj_node) {
                    if Some(adj_node) == two_visits.as_ref() {
                        continue;
                    }

                    if visited.contains(adj_node) {
                        if two_visits == None {
                            new_visit = Some(adj_node.clone());
                        } else {
                            continue;
                        }
                    } else {
                        visited.insert(adj_node.clone());
                    }
                }
                path.push(adj_node.clone());
                total += self.solve(path, visited, new_visit);
            }
        };
        visited.remove(&node);
        path.pop();
        return total;
    }
}
