use color_eyre::Result;
use eg_graph_editor_lib::graph::Graph;
use itertools::Itertools;
use std::collections::VecDeque;

pub fn print_graph(graph: &Graph) -> Result<()> {
    let mut queue = VecDeque::new();

    queue.push_back(graph.root_id());

    while let Some(id) = queue.pop_front() {
        let lvl = graph.level_of(id).unwrap();
        let atoms = graph.atoms_of(id).unwrap();
        let children = graph.subgraphs_of(id).unwrap();

        println!(
            "  {}{}: Atoms [{}]",
            "-".repeat(lvl * 2) + ">",
            id,
            atoms
                .iter()
                .map(|v| v.to_string())
                .intersperse(",".to_string())
                .collect::<String>()
        );

        children.iter().for_each(|k| queue.push_back(k));
    }

    Ok(())
}
