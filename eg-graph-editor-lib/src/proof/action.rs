pub mod error;

use tracing::instrument;

use self::error::{ActionError, ActionResult};
use crate::{
    atom::Atom,
    graph::{Graph, GraphKey},
};
use std::{
    borrow::Borrow,
    collections::{HashMap, LinkedList},
    fmt::Debug,
};

#[derive(Debug)]
pub enum Action {
    AddAtom {
        target: GraphTarget,
        atom: Atom,
    },
    DeleteAtom {
        target: GraphTarget,
        atom: Atom,
    },
    AddSubgraph {
        target: GraphTarget,
        new_subgraph: GraphTarget,
    },
    DeleteSubgraph {
        target: GraphTarget,
    },
    MoveSubgraph {
        target: GraphTarget,
        dest: GraphTarget,
    },
}

#[derive(Clone, Debug)]
pub enum GraphTarget {
    Exists(GraphKey),
    Future(usize),
}

impl Action {
    #[instrument]
    pub fn apply_actions<S: IntoIterator<Item = Self> + Debug>(
        actions: S,
        graph: &mut Graph,
    ) -> ActionResult<impl IntoIterator<Item = Action> + Debug> {
        let mut reversed_actions = LinkedList::new();

        let mut matched_future_targets = HashMap::new();

        for action in actions {
            match action {
                Action::AddAtom { target, atom } => {
                    let t_id = resolve_target(&target, &matched_future_targets)?;
                    graph.insert_atom(t_id, atom.clone())?;

                    reversed_actions.push_front(Action::DeleteAtom { target, atom })
                }

                Action::DeleteAtom { target, atom } => {
                    let t_id = resolve_target(&target, &matched_future_targets)?;
                    graph.remove_atom_from_subgraph(t_id, atom.borrow())?;

                    reversed_actions.push_front(Action::AddAtom { target, atom })
                }

                Action::AddSubgraph {
                    target,
                    new_subgraph,
                } => {
                    let t_id = resolve_target(&target, &matched_future_targets)?;

                    match new_subgraph {
                        GraphTarget::Exists(new_id) => {
                            if graph.level_of(&new_id).is_ok() {
                                Err(ActionError::SubgraphIdAlreadyExists(new_id.to_string()))?
                            } else {
                                graph.insert_subgraph_with_id(new_id, t_id)?;
                            }

                            reversed_actions.push_front(Action::DeleteSubgraph {
                                target: GraphTarget::Exists(new_id),
                            })
                        }

                        GraphTarget::Future(x) => {
                            let new_id = graph.insert_subgraph(t_id)?;

                            matched_future_targets.insert(x, new_id);

                            reversed_actions.push_front(Action::DeleteSubgraph {
                                target: GraphTarget::Exists(new_id),
                            })
                        }
                    }
                }

                Action::DeleteSubgraph { target } => {
                    let t_id = resolve_target(&target, &matched_future_targets)?;
                    let p_id = *graph.parent_of(t_id)?;

                    graph.remove_subgraph(t_id, false)?;

                    reversed_actions.push_front(Action::AddSubgraph {
                        target: GraphTarget::Exists(p_id),
                        new_subgraph: target,
                    })
                }

                Action::MoveSubgraph { target, dest } => {
                    let t_id = resolve_target(&target, &matched_future_targets)?;
                    let d_id = resolve_target(&dest, &matched_future_targets)?;
                    let s_id = *graph.parent_of(t_id)?;

                    graph.move_subgraph(t_id, d_id)?;

                    reversed_actions.push_front(Action::MoveSubgraph {
                        target: GraphTarget::Exists(*t_id),
                        dest: GraphTarget::Exists(s_id),
                    })
                }
            }
        }

        Ok(reversed_actions)
    }
}

#[instrument]
fn resolve_target<'a>(
    target: &'a GraphTarget,
    matched_future_targets: &'a HashMap<usize, GraphKey>,
) -> ActionResult<&'a GraphKey> {
    match target {
        GraphTarget::Exists(x) => Ok(x),
        GraphTarget::Future(x) => matched_future_targets
            .get(x)
            .ok_or(ActionError::UndefinedFutureGraph(*x)),
    }
}
