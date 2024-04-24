use crate::{
    atom::Atom,
    graph::Graph,
    proof::{action::Action, inference_rule::InferenceRule},
};
use std::{collections::VecDeque, env};
use std::{error::Error, sync::Arc};
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_panic::panic_hook;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

pub fn init_logging() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .with(ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer().with_span_events(
            if let Ok("1") = env::var("RUST_LOG_TRACE_SPAN").as_deref() {
                FmtSpan::NEW | FmtSpan::CLOSE
            } else {
                FmtSpan::NONE
            },
        ))
        .try_init()?;

    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        panic_hook(panic_info);
        prev_hook(panic_info);
    }));

    Ok(())
}

fn print_graph(graph: &Graph) {
    let mut queue = VecDeque::new();

    queue.push_back(graph.root_id());

    while let Some(id) = queue.pop_front() {
        let lvl = graph.level_of(id).unwrap();
        let atoms = graph.atoms_of(id).unwrap();
        let children = graph.subgraphs_of(id).unwrap();

        println!("{}{}: {:?}", "-".repeat(lvl * 2) + ">", id, atoms);

        children.iter().for_each(|k| queue.push_back(k));
    }
}

#[test]
fn double_cut_draw_test() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    let graph = Graph::try_from("[A,B,C]")?;

    let atom_a = graph
        .atoms_of(graph.root_id())?
        .iter()
        .next()
        .unwrap()
        .clone();

    let rule = InferenceRule::DoubleCutDraw {
        target: *graph.root_id(),
        target_atoms: Vec::from([atom_a]),
        target_subgraphs: Vec::new(),
    };

    let actions = rule.gen_actions_from_rule(&graph)?;

    print_graph(&graph);

    println!("\n{:#?}", actions);

    Ok(())
}

#[test]
fn double_cut_erase_test() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    let mut graph = Graph::try_from(
        "[
            A,
            [
                [
                    B,
                    [C, D]
                ],
            ]
        ]",
    )?;

    let rule = InferenceRule::DoubleCutErase {
        target: *graph.subgraphs_of(graph.root_id())?.iter().next().unwrap(),
    };

    println!("Original:");
    print_graph(&graph);
    println!();

    let actions = rule.gen_actions_from_rule(&graph)?;

    let actions_rev = Action::apply_actions(actions, &mut graph)?;

    println!("Double cut:");
    print_graph(&graph);
    println!();

    Action::apply_actions(actions_rev, &mut graph)?;

    println!("Reversed:");
    print_graph(&graph);
    println!();

    Ok(())
}

#[test]
fn iteration_test() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    let mut graph = Graph::try_from("[A, [B], []]")?;

    let subgraphs: Vec<_> = graph.subgraphs_of(graph.root_id())?.iter().collect();

    println!("{:?}", subgraphs);

    let rule1 = InferenceRule::Iteration {
        backwards: false,
        parent: *graph.root_id(),
        parent_atoms: Vec::from([Arc::new(Atom::new("A".to_string()))]),
        parent_subgraphs: Vec::from([*subgraphs[0]]),
        target: *subgraphs[1],
    };

    let rule2 = InferenceRule::Iteration {
        backwards: true,
        parent: *subgraphs[0],
        parent_atoms: Vec::from([Arc::new(Atom::new("B".to_string()))]),
        parent_subgraphs: vec![],
        target: *graph.root_id(),
    };

    println!("Original:");
    print_graph(&graph);
    println!();

    let actions1 = rule1.gen_actions_from_rule(&graph)?;
    Action::apply_actions(actions1, &mut graph)?;

    println!("Iteration:");
    print_graph(&graph);
    println!();

    let actions2 = rule2.gen_actions_from_rule(&graph)?;
    Action::apply_actions(actions2, &mut graph)?;

    println!("Deiteration:");
    print_graph(&graph);
    println!();

    Ok(())
}
