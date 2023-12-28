use std::{io::{BufReader, BufRead}, fs::File, collections::{HashSet, HashMap}};
use graph::prelude::*;

fn get_edges(file: &str) -> Vec<(usize, usize)> {
    let file = File::open(file).unwrap();
    let lines = BufReader::new(file).lines();
    let mut i = 0;
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut edges = HashSet::new();
    for line in lines {
        let line = line.unwrap().clone();
        let split: Vec<&str> = line.split(": ").collect();
        let from = split.first().unwrap().trim();
        if map.get(from).is_none() {
            map.insert(String::from(from), i);
            i += 1;
        }
        let from = *map.get(from).unwrap();
        let to = split.last().unwrap().split_whitespace().map(|x| x.trim()).collect::<Vec<&str>>();
        for to in to {
            if map.get(to).is_none() {
                map.insert(String::from(to), i);
                i += 1;
            }
            let to = *map.get(to).unwrap();
            edges.insert(if from < to { (from, to) } else { (to, from) });
        }
    }
    edges.into_iter().collect::<Vec<(usize, usize)>>()
}

fn product_of_two_cycle_lengths(graph: &mut UndirectedCsrGraph<usize>) -> Option<usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut stack = vec![];
    let mut cycle_lengths = vec![];
    for i in 0..graph.node_count() {
        if !visited.contains(&i) {
            stack.push(i);
            visited.insert(i);
            let mut cycle_length = 0;
            while let Some(node) = stack.pop() {
                cycle_length += 1;
                for neighbor in graph.neighbors(node) {
                    if !visited.contains(neighbor) {
                        stack.push(*neighbor);
                        visited.insert(*neighbor);
                    }
                }
            }
            cycle_lengths.push(cycle_length);
        }
    }
    if cycle_lengths.len() == 2 {
        Some(cycle_lengths.into_iter().product())
    } else {
        None
    }
}

fn solution(file: &str) -> Option<usize> {
    let edges = get_edges(file);
    println!("{:?}", edges);
    let base_graph: UndirectedCsrGraph<usize> = GraphBuilder::new().edges(edges.clone()).build();
    let node_count = base_graph.node_count();
    for i in 0..node_count {
        for j in i+1..node_count {
            for k in j+1..node_count {
                let modified_edges = edges
                    .iter()
                    .filter(|(from, to)| 
                        !(*from == i && *to == j) &&
                        !(*from == j && *to == i) &&
                        !(*from == j && *to == k) &&
                        !(*from == k && *to == j) &&
                        !(*from == i && *to == k) &&
                        !(*from == k && *to == i)
                    )
                    .map(|(from, to)| (*from, *to))
                    .collect::<Vec<(usize, usize)>>();
                println!("{:?}", modified_edges);
                let mut graph: UndirectedCsrGraph<usize> = GraphBuilder::new().edges(modified_edges).build();
                if let Some(n) = product_of_two_cycle_lengths(&mut graph) {
                    return Some(n);
                }
            }
        }
    }
    None
}

fn main() {
    assert_eq!(solution("example.txt").unwrap(), 54);
}
