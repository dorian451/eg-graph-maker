pub mod args;
pub mod print;

use crate::{
    args::{edit::EditCommand, mode::Mode},
    print::print_graph,
};
use args::{rule::RuleCommand, Cli, Command};
use async_std::io::{stdin, stdout, ReadExt, WriteExt};
use clap::{builder::styling::AnsiColor, Parser};
use color_eyre::{
    eyre::{eyre, Error},
    owo_colors::AnsiColors,
    Report, Result,
};
use eg_graph_editor_lib::{
    atom::Atom,
    graph::{Graph, GraphKey},
    proof::{
        action::{Action, GraphTarget},
        inference_rule::InferenceRule,
    },
};
use fallible_iterator::{FallibleIterator, IteratorExt};
use std::{collections::VecDeque, env, mem, sync::Arc};
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_panic::panic_hook;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

fn init_logging() -> Result<()> {
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

#[async_std::main]
async fn main() -> Result<()> {
    init_logging()?;

    let mut graph = Graph::new();
    let mut undo_stack = VecDeque::new();
    let mut redo_stack = VecDeque::new();

    let mut current_mode = Mode::Editor;

    loop {
        print!("> ");
        stdout().flush().await?;

        let mut line = String::new();
        let _ = stdin().read_line(&mut line).await?;

        let args = Cli::try_parse_from(("> ".to_string() + &line).split_whitespace());

        let res: Result<bool> = (|| match args {
            Ok(args) => match args.cmd {
                Command::Show => {
                    println!();
                    print_graph(&graph)?;
                    println!();
                    Ok(true)
                }

                Command::Exit => Ok(false),

                Command::Mode { mode } => {
                    match mode {
                        Some(m) => current_mode = m,
                        None => println!("Current mode: {}", current_mode),
                    };
                    Ok(true)
                }

                Command::Edit { how } => {
                    redo_stack.clear();

                    if current_mode != Mode::Editor {
                        Err(eyre!("Cannot edit graph in proof mode"))?
                    }

                    match how {
                        EditCommand::NewAtom { target, atom } => undo_stack.push_front(
                            Action::apply_actions(
                                [Action::AddAtom {
                                    target: GraphTarget::Exists(GraphKey::try_from_str(&target)?),
                                    atom: atom.into(),
                                }],
                                &mut graph,
                            )
                            .unwrap(),
                        ),

                        EditCommand::NewCut { target } => undo_stack.push_front(
                            Action::apply_actions(
                                [Action::AddSubgraph {
                                    target: GraphTarget::Exists(GraphKey::try_from_str(&target)?),
                                    new_subgraph: GraphTarget::Future(0),
                                }],
                                &mut graph,
                            )
                            .unwrap(),
                        ),

                        EditCommand::DeleteAtom { target, atom } => undo_stack.push_front(
                            Action::apply_actions(
                                [Action::DeleteAtom {
                                    target: GraphTarget::Exists(GraphKey::try_from_str(&target)?),
                                    atom: atom.into(),
                                }],
                                &mut graph,
                            )
                            .unwrap(),
                        ),
                        EditCommand::DeleteCut { target } => undo_stack.push_front(
                            Action::apply_actions(
                                [Action::DeleteSubgraph {
                                    target: GraphTarget::Exists(GraphKey::try_from_str(&target)?),
                                }],
                                &mut graph,
                            )
                            .unwrap(),
                        ),

                        EditCommand::Load { new_graph } => {
                            let mut new_graph = Graph::try_from(new_graph.as_str())?;
                            undo_stack.clear();
                            redo_stack.clear();
                            mem::swap(&mut graph, &mut new_graph);
                        }
                    };
                    Ok(true)
                }

                Command::Rule { how } => {
                    redo_stack.clear();

                    match how {
                        RuleCommand::DoubleCutDraw {
                            target,
                            target_atoms,
                            target_subgraphs,
                        } => redo_stack.push_front(Action::apply_actions(
                            InferenceRule::DoubleCutDraw {
                                target: GraphKey::try_from_str(target.as_str())?,
                                target_atoms: target_atoms
                                    .into_iter()
                                    .map(|a| Arc::new(Atom::from(a)))
                                    .collect(),
                                target_subgraphs: target_subgraphs
                                    .into_iter()
                                    .map(|v| Ok::<_, Report>(GraphKey::try_from_str(v.as_str())?))
                                    .transpose_into_fallible()
                                    .collect()?,
                            }
                            .gen_actions_from_rule(&graph)?,
                            &mut graph,
                        )?),

                        RuleCommand::DoubleCutErase { target } => {
                            redo_stack.push_front(Action::apply_actions(
                                InferenceRule::DoubleCutErase {
                                    target: GraphKey::try_from_str(target.as_str())?,
                                }
                                .gen_actions_from_rule(&graph)?,
                                &mut graph,
                            )?)
                        }

                        RuleCommand::Insertion {
                            target,
                            new_content,
                        } => redo_stack.push_front(Action::apply_actions(
                            InferenceRule::Insertion {
                                target: GraphKey::try_from_str(target.as_str())?,
                                new_content,
                            }
                            .gen_actions_from_rule(&graph)?,
                            &mut graph,
                        )?),

                        RuleCommand::ErasureAtoms {
                            parent_subgraph,
                            atoms,
                        } => {
                            let parent_subgraph = GraphKey::try_from_str(parent_subgraph.as_str())?;

                            redo_stack.push_front(Action::apply_actions(
                                InferenceRule::Erasure {
                                    target_subgraphs: vec![],
                                    target_atoms: atoms
                                        .into_iter()
                                        .map(|v| (parent_subgraph, Arc::new(Atom::new(v))))
                                        .collect(),
                                }
                                .gen_actions_from_rule(&graph)?,
                                &mut graph,
                            )?)
                        }

                        RuleCommand::ErasureCuts { target_subgraphs } => {
                            redo_stack.push_front(Action::apply_actions(
                                InferenceRule::Erasure {
                                    target_subgraphs: target_subgraphs
                                        .into_iter()
                                        .map(|v| {
                                            Ok::<_, Report>(GraphKey::try_from_str(v.as_str())?)
                                        })
                                        .transpose_into_fallible()
                                        .collect()?,
                                    target_atoms: vec![],
                                }
                                .gen_actions_from_rule(&graph)?,
                                &mut graph,
                            )?)
                        }

                        RuleCommand::Iteration_a {
                            parent,
                            // parent_atoms,
                            parent_subgraphs,
                            target,
                        } => redo_stack.push_front(Action::apply_actions(
                            InferenceRule::Iteration {
                                backwards: false,
                                parent: GraphKey::try_from_str(parent.as_str())?,
                                parent_atoms: vec![],
                                parent_subgraphs: parent_subgraphs
                                    .into_iter()
                                    .map(|v| Ok::<_, Report>(GraphKey::try_from_str(v.as_str())?))
                                    .transpose_into_fallible()
                                    .collect()?,
                                target: GraphKey::try_from_str(target.as_str())?,
                            }
                            .gen_actions_from_rule(&graph)?,
                            &mut graph,
                        )?),
                        RuleCommand::Iteration_g {
                            parent,
                            parent_atoms,
                            // parent_subgraphs,
                            target,
                        } => redo_stack.push_front(Action::apply_actions(
                            InferenceRule::Iteration {
                                backwards: false,
                                parent: GraphKey::try_from_str(parent.as_str())?,
                                parent_atoms: vec![],
                                parent_subgraphs: vec![],
                                target: GraphKey::try_from_str(target.as_str())?,
                            }
                            .gen_actions_from_rule(&graph)?,
                            &mut graph,
                        )?),
                    };

                    Ok(true)
                }

                Command::Undo { times } => {
                    let mut iter = 0;
                    while let Some(things) = undo_stack.pop_front() {
                        redo_stack.push_front(Action::apply_actions(things, &mut graph).unwrap());
                        iter += 1;
                        if iter >= times {
                            break;
                        }
                    }
                    Ok(true)
                }
                Command::Redo { times } => {
                    let mut iter = 0;
                    while let Some(things) = redo_stack.pop_front() {
                        undo_stack.push_front(Action::apply_actions(things, &mut graph).unwrap());
                        iter += 1;
                        if iter >= times {
                            break;
                        }
                    }
                    Ok(true)
                }
            },
            Err(x) => {
                println!("{}", x);
                Ok(true)
            }
        })();

        if let Err(x) = res {
            println!("{}", x);
        } else if let Ok(false) = res {
            break;
        };
    }

    Ok(())
}
