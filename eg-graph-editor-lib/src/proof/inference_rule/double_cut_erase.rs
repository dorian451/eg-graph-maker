use super::InferenceRule;
use crate::{
    graph::Graph,
    proof::{
        action::{Action, GraphTarget},
        error::{ProofError, ProofResult},
    },
};
use std::collections::LinkedList;
use tracing::instrument;

#[instrument]
pub fn gen_actions_from_double_cut_erase(
    rule: &InferenceRule,
    graph: &Graph,
) -> ProofResult<LinkedList<Action>> {
    if let InferenceRule::DoubleCutErase { target } = rule {
        if !graph.atoms_of(target)?.is_empty() || graph.subgraphs_of(target)?.len() != 1 {
            Err(ProofError::InvalidApplicationOfRule(
                "The outer cut of a double cut should contain nothing except the inner cut"
                    .to_string(),
            ))?
        } else {
            let mut ans = LinkedList::new();
            let inner_ring = *graph.subgraphs_of(target)?.iter().next().unwrap();
            let parent = *graph.parent_of(target)?;

            for atom in graph.atoms_of(&inner_ring)? {
                ans.push_back(Action::DeleteAtom {
                    target: GraphTarget::Exists(inner_ring),
                    atom: (**atom).clone(),
                });
                ans.push_back(Action::AddAtom {
                    target: GraphTarget::Exists(parent),
                    atom: (**atom).clone(),
                });
            }

            for subgraph in graph.subgraphs_of(&inner_ring)? {
                ans.push_back(Action::MoveSubgraph {
                    target: GraphTarget::Exists(*subgraph),
                    dest: GraphTarget::Exists(parent),
                })
            }

            ans.push_back(Action::DeleteSubgraph {
                target: GraphTarget::Exists(inner_ring),
            });

            ans.push_back(Action::DeleteSubgraph {
                target: GraphTarget::Exists(*target),
            });

            Ok(ans)
        }
    } else {
        panic!("This method should only be used for erasing double cuts")
    }
}
