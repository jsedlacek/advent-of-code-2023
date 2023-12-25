use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, newline, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

type Vertex = String;

type Edge = (Vertex, Vertex);

#[derive(Debug, Clone)]
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    edge_map: HashMap<Vertex, HashSet<Vertex>>,
}

impl Graph {
    fn construct_edge_map(edges: &[Edge]) -> HashMap<Vertex, HashSet<Vertex>> {
        let mut edge_map: HashMap<Vertex, HashSet<Vertex>> = HashMap::new();

        for (e1, e2) in edges {
            edge_map.entry(e1.clone()).or_default().insert(e2.clone());
            edge_map.entry(e2.clone()).or_default().insert(e1.clone());
        }

        edge_map
    }

    fn find_way(&self, start: &str, end: &str) -> Option<Vec<Vertex>> {
        let mut visited = HashSet::new();

        let mut queue: VecDeque<(Vertex, Vec<Vertex>)> =
            VecDeque::from([(start.to_string(), Vec::new())]);

        while let Some((v, path)) = queue.pop_front() {
            let mut path = path.clone();
            path.push(v.clone());
            visited.insert(v.clone());

            if v == end {
                return Some(path);
            }

            for n in self.get_neighbours(&v) {
                if !visited.contains(&n) {
                    queue.push_back((n.to_string(), path.clone()));
                }
            }
        }

        return None;
    }

    fn number_of_ways(&self, start: &str, end: &str) -> u64 {
        let mut g = self.clone();
        let mut count = 0;

        while let Some(path) = g.find_way(start, end) {
            count += 1;

            let edges = path
                .windows(2)
                .map(|a| (a[0].clone(), a[1].clone()))
                .collect::<Vec<_>>();

            g.remove_edges(edges);
        }

        count
    }

    fn remove_edges(&mut self, edges: Vec<(String, String)>) {
        self.edges
            .retain(|e| !edges.contains(e) && !edges.contains(&(e.1.clone(), e.0.clone())));
        self.edge_map = Self::construct_edge_map(&self.edges);
    }

    fn get_neighbours(&self, v: &str) -> HashSet<Vertex> {
        self.edge_map
            .get(v)
            .map_or_else(HashSet::new, HashSet::clone)
    }

    fn part1(&self) -> u64 {
        let (start, rest) = self.vertices.split_first().unwrap();

        let mut group = vec![start];

        for v in rest {
            if self.number_of_ways(start, v) > 3 {
                group.push(v);
            }
        }

        (group.len() * (self.vertices.len() - group.len())) as u64
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        // Example: "jqt: rhn xhk nvd"

        map(
            separated_list1(
                newline::<&str, _>,
                map(
                    tuple((alpha1, tag(":"), space0, separated_list1(space1, alpha0))),
                    |(node, _, _, edges)| (node, edges),
                ),
            ),
            |list| {
                let edges: Vec<(String, String)> = list
                    .iter()
                    .flat_map(|(node, edges)| {
                        edges
                            .iter()
                            .map(|edge| (node.to_string(), edge.to_string()))
                    })
                    .collect();

                let nodes = edges
                    .iter()
                    .flat_map(|(a, b)| [a.clone(), b.clone()].into_iter())
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();

                let edge_map = Self::construct_edge_map(&edges);

                Self {
                    vertices: nodes,
                    edges,
                    edge_map,
                }
            },
        )(input)
    }
}

fn main() {
    let g = Graph::parse(include_str!("input.txt")).unwrap().1;

    dbg!(g.part1());
}
