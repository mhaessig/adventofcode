use std::collections::{HashMap, HashSet};

use petgraph::graph::{NodeIndex, UnGraph};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Big(&'a str),
    Small(&'a str),
}

impl<'a> Cave<'a> {
    fn new(s: &'a str) -> Self {
        if s.chars()
            .all(|c| c.is_ascii_uppercase())
        {
            Self::Big(s)
        } else if s
            .chars()
            .all(|c| c.is_ascii_lowercase())
        {
            Self::Small(s)
        } else {
            unreachable!("all input needs to be all uppercase or lowercase");
        }
    }

    fn is_small(self) -> bool {
        match self {
            Self::Small(_) => true,
            Self::Big(_) => false,
        }
    }
}

fn main() {
    let edges: Vec<(Cave, Cave)> = include_str!("../input.txt")
        //let edges: Vec<(Cave, Cave)> = include_str!("../example_input2.txt")
        //let edges: Vec<(Cave, Cave)> = include_str!("../example_input1.txt")
        .lines()
        .map(|s| s.split_once('-').unwrap())
        .map(|(n1, n2)| (Cave::new(n1), Cave::new(n2)))
        .collect();

    let mut node_weights = HashSet::new();
    let mut g = UnGraph::<Cave, ()>::with_capacity(node_weights.len(), edges.len());
    edges.iter().for_each(|(c1, c2)| {
        node_weights.insert(*c1);
        node_weights.insert(*c2);
    });

    let mut node_map: HashMap<Cave, NodeIndex> = HashMap::new();
    node_weights.into_iter().for_each(|w| {
        node_map.insert(w, g.add_node(w));
    });

    let start = *node_map.get(&Cave::new("start")).unwrap();
    let end = *node_map.get(&Cave::new("end")).unwrap();

    edges.iter().for_each(|(n1, n2)| {
        g.add_edge(*node_map.get(n1).unwrap(), *node_map.get(n2).unwrap(), ());
    });

    let num_paths = build_path(&g, start, end, false, &Vec::new(), &Vec::new());
    dbg!(num_paths);
}

fn build_path(
    g: &UnGraph<Cave, ()>,
    n: NodeIndex,
    end: NodeIndex,
    visited_one_small_cave_twice: bool,
    cant_visit: &[Cave],
    visited: &[Cave],
) -> usize {
    if n == end {
        //println!("{:?}", visited);
        return 1;
    }

    let nw = *g.node_weight(n).unwrap();
    let mut cant_visit = cant_visit.to_owned();
    let mut visited = visited.to_owned();

    let visited_one_small_cave_twice = if visited.contains(&nw) && nw.is_small() {
        visited.iter().for_each(|c| {
            if c.is_small() {
                cant_visit.push(*c);
            }
        });
        true
    } else {
        visited_one_small_cave_twice
    };

    visited.push(nw);

    if let Cave::Small(s) = nw {
        if s == "start" || visited_one_small_cave_twice {
            cant_visit.push(nw);
        }
    }

    let visit = g
        .neighbors(n)
        .filter(|i| !cant_visit.contains(g.node_weight(*i).unwrap()))
        .collect::<Vec<_>>();

    visit.into_iter().fold(0, |count, i| {
        count
            + build_path(
                g,
                i,
                end,
                visited_one_small_cave_twice,
                &cant_visit,
                &visited,
            )
    })
}
