use super::InferenceRule;
use crate::{
    graph::{parse_graph_string_into_actions, Graph},
    proof::{
        action::Action,
        error::{ProofError, ProofResult},
    },
};
use std::collections::LinkedList;
use tracing::instrument;

#[instrument]
pub fn gen_actions_from_insertion(
    rule: &InferenceRule,
    graph: &Graph,
) -> ProofResult<LinkedList<Action>> {
    if let InferenceRule::Insertion {
        target,
        new_content,
    } = rule
    {
        let target_level = graph.level_of(target)?;
        if target_level % 2 == 0 {
            Err(ProofError::InvalidApplicationOfRule(format!(
                "Selected subgraph must be on a odd level, but is on level {}",
                target_level
            )))?
        } else {
            Ok(
                parse_graph_string_into_actions(graph, new_content, *target)?
                    .into_iter()
                    .collect::<LinkedList<_>>(),
            )
        }
    } else {
        panic!("This method should only be used for insertion")
    }
}
