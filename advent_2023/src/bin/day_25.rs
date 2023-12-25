use anyhow::Ok;
use humantime::format_duration;
use regex::Regex;
use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::{graph::NodeIndex, graph::UnGraph},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn get_node(
    label: &str,
    graph: &mut UnGraph<String, usize>,
    nodes: &mut HashMap<String, NodeIndex>,
) -> NodeIndex {
    // I can't believe there isn't an API in PetGraph to get a node by it's label...
    let node: &NodeIndex = nodes
        .entry(label.to_string())
        .or_insert_with(|| graph.add_node(label.to_string()));
    *node
}

fn part_a(lines: &[String]) -> usize {
    // It's Christmas Day - lets use rustworkx rather than implementing Stoer–Wagner ourselves

    let mut graph: UnGraph<String, usize, _> = UnGraph::new_undirected();
    let mut nodes = HashMap::new();

    for line in lines {
        let mut it = line.split(": ");
        let source = it.next().unwrap();
        let s_node = get_node(source, &mut graph, &mut nodes);

        for target in it.next().unwrap().split_whitespace() {
            let t_node = get_node(target, &mut graph, &mut nodes);
            graph.add_edge(s_node, t_node, 1);
        }
    }

    let (_, left_nodes) = stoer_wagner_min_cut(&graph, |_| Ok(1usize))
        .unwrap()
        .unwrap();

    let mut right_nodes = graph.clone();
    right_nodes.retain_nodes(|_, b| !left_nodes.contains(&b));

    left_nodes.len() * right_nodes.node_count()
}

#[cfg(not(tarpaulin_include))]
fn main() -> AResult<()> {
    // Figure out which exercise we are so we can load the correct input file
    let pattern = Regex::new("[0-9]+$")?;
    let name = &std::env::args().next().expect("binary name not found.");
    let ex = pattern
        .find(name)
        .expect("binary name should contain a number")
        .as_str();
    println!("Running code for Day {ex}.");

    // Load the appropriate input text
    let file = File::open(format!("./data/day_{ex}.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(lines.as_slice()));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 54);
    }
}
