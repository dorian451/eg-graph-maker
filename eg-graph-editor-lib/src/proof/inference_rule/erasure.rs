use super::InferenceRule;
use crate::{
    graph::{parse_graph_string_into_actions, Graph},
    proof::{
        action::{Action, GraphTarget},
        error::{ProofError, ProofResult},
    },
};
use fallible_iterator::{FallibleIterator, IteratorExt};
use std::collections::{LinkedList, VecDeque};
use tracing::instrument;

#[instrument]
pub fn gen_actions_from_erasure(
    rule: &InferenceRule,
    graph: &Graph,
) -> ProofResult<LinkedList<Action>> {
    if let InferenceRule::Erasure {
        target_subgraphs,
        target_atoms,
    } = rule
    {
        if target_subgraphs
            .iter()
            .map(Ok::<_, ProofError>)
            .transpose_into_fallible()
            .any(|v| Ok(graph.level_of(v)? % 2 == 0))?
            || target_atoms
                .iter()
                .map(Ok::<_, ProofError>)
                .transpose_into_fallible()
                .any(|(p, _)| Ok(graph.level_of(p)? % 2 == 1))?
        {
            Err(ProofError::InvalidApplicationOfRule(
                "Erasure can only delete things from even levels".to_string(),
            ))?
        } else {
            let mut ans = LinkedList::new();

            let mut queue = VecDeque::from_iter(target_subgraphs.iter().cloned());
            while let Some(id) = queue.pop_front() {
                for p in graph.subgraphs_of(&id)? {
                    queue.push_back(*p)
                }

                ans.push_front(Action::DeleteSubgraph {
                    target: GraphTarget::Exists(id),
                });

                for a in graph.atoms_of(&id)? {
                    ans.push_front(Action::DeleteAtom {
                        target: GraphTarget::Exists(id),
                        atom: (**a).clone(),
                    })
                }
            }

            for (p, a) in target_atoms {
                ans.push_front(Action::DeleteAtom {
                    target: GraphTarget::Exists(*p),
                    atom: (**a).clone(),
                })
            }

            Ok(ans)
        }
    } else {
        panic!("This method should only be used for erasure")
    }
}
