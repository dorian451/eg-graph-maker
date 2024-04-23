use super::InferenceRule;
use crate::{
    graph::{parse_graph_string_into_actions, transform_graph_into_string, Graph},
    proof::{
        action::{Action, GraphTarget},
        error::{ProofError, ProofResult},
    },
};
use fallible_iterator::{FallibleIterator, IteratorExt};
use itertools::Itertools;
use std::collections::{LinkedList, VecDeque};
use tracing::instrument;

#[instrument]
pub fn gen_actions_from_iteration(
    rule: &InferenceRule,
    graph: &Graph,
) -> ProofResult<LinkedList<Action>> {
    if let InferenceRule::Iteration {
        parent,
        parent_atoms,
        parent_subgraphs,
        target,
    } = rule
    {
        // check validity

        if parent_atoms
            .iter()
            .map(Ok::<_, ProofError>)
            .transpose_into_fallible()
            .any(|a| Ok(!graph.atoms_of(parent)?.contains(a)))?
            || parent_subgraphs
                .iter()
                .map(Ok::<_, ProofError>)
                .transpose_into_fallible()
                .any(|v| Ok(!graph.subgraphs_of(parent)?.contains(v)))?
        {
            Err(ProofError::InvalidApplicationOfRule(
                "Invalid Selection".to_string(),
            ))?
        } else {
            // should probably optimize this at some point, but it works for now

            let str_graph = format!(
                "[{}]",
                Iterator::chain(
                    parent_atoms.iter().map(|e| e.to_string()),
                    parent_subgraphs
                        .iter()
                        .map(|e| transform_graph_into_string(graph, e))
                )
                .intersperse(",".to_string())
                .collect::<String>()
            );

            Ok(parse_graph_string_into_actions(graph, &str_graph, *target)?)
        }
    } else {
        panic!("This method should only be used for erasure")
    }
}
