use super::InferenceRule;
use crate::{
    graph::Graph,
    proof::{
        action::{Action, GraphTarget},
        error::{ProofError, ProofResult},
    },
};
use fallible_iterator::{FallibleIterator, IteratorExt};
use std::collections::LinkedList;
use tracing::instrument;

#[instrument]
pub fn gen_actions_from_double_cut_draw(
    rule: &InferenceRule,
    graph: &Graph,
) -> ProofResult<LinkedList<Action>> {
    if let InferenceRule::DoubleCutDraw {
        target,
        target_atoms,
        target_subgraphs,
    } = rule
    {
        // check validity

        if target_atoms
            .iter()
            .map(Ok::<_, ProofError>)
            .transpose_into_fallible()
            .any(|a| Ok(!graph.atoms_of(target)?.contains(a)))?
            || target_subgraphs
                .iter()
                .map(Ok::<_, ProofError>)
                .transpose_into_fallible()
                .any(|v| Ok(!graph.subgraphs_of(target)?.contains(v)))?
        {
            Err(ProofError::InvalidApplicationOfRule(
                "Target subgraph does not contain all selected elements".to_string(),
            ))?
        }

        // calculate the actions

        let mut ans = LinkedList::from([
            Action::AddSubgraph {
                target: GraphTarget::Exists(*target),
                new_subgraph: GraphTarget::Future(0),
            },
            Action::AddSubgraph {
                target: GraphTarget::Future(0),
                new_subgraph: GraphTarget::Future(1),
            },
        ]);

        for atom in target_atoms {
            ans.push_back(Action::DeleteAtom {
                target: GraphTarget::Exists(*target),
                atom: (**atom).clone(),
            });

            ans.push_back(Action::AddAtom {
                target: GraphTarget::Future(1),
                atom: (**atom).clone(),
            });
        }

        for subgraph_key in target_subgraphs {
            ans.push_back(Action::MoveSubgraph {
                target: GraphTarget::Exists(*subgraph_key),
                dest: GraphTarget::Future(1),
            })
        }

        Ok(ans)
    } else {
        panic!("This method should only be used for drawing double cuts")
    }
}
