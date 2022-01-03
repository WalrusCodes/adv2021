use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn insert_edge(&mut self, a: &str, b: &str) {
        if !self.edges.contains_key(a) {
            self.edges.insert(a.to_string(), Vec::new());
        }
        self.edges.get_mut(a).unwrap().push(b.to_string());
    }
    fn parse(lines: &str) -> Graph {
        let mut graph = Graph {
            edges: HashMap::new(),
        };

        // let mut edges = HashMap::new();
        for line in lines.lines() {
            let mut it = line.split('-');
            let from = it.next().unwrap();
            let to = it.next().unwrap();
            graph.insert_edge(from, to);
            graph.insert_edge(to, from);
        }
        graph
    }
    fn count_paths(&self, at: &str, visited: &mut HashSet<String>, small_revisit_ok: bool) -> u64 {
        if at == "end" {
            return 1;
        }
        let small_cave = at.chars().next().unwrap().is_lowercase();
        let remove_small_cave = if small_cave {
            if visited.contains(at) {
                false
            } else {
                visited.insert(at.to_string());
                true
            }
        } else {
            false
        };

        let mut count = 0u64;
        for edge in self.edges.get(at).unwrap() {
            if edge == "start" {
                continue;
            }
            if visited.contains(edge) {
                if small_revisit_ok {
                    count += self.count_paths(edge, visited, false);
                }
            } else {
                count += self.count_paths(edge, visited, small_revisit_ok);
            }
        }

        if remove_small_cave {
            visited.remove(at);
        }
        count
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");
    let graph = Graph::parse(&contents);
    dbg!(&graph);
    dbg!(graph.count_paths("start", &mut HashSet::new(), false));
    dbg!(graph.count_paths("start", &mut HashSet::new(), true));
}
