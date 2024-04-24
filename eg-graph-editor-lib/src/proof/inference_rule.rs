mod double_cut_draw;
mod double_cut_erase;
mod erasure;
mod insertion;
mod iteration;

use self::{
    double_cut_draw::gen_actions_from_double_cut_draw,
    double_cut_erase::gen_actions_from_double_cut_erase, erasure::gen_actions_from_erasure,
    insertion::gen_actions_from_insertion, iteration::gen_actions_from_iteration,
};
use super::{action::Action, error::ProofResult};
use crate::{
    atom::Atom,
    graph::{Graph, GraphKey},
};
use std::{fmt::Debug, sync::Arc};
use tracing::instrument;

#[derive(Debug)]
pub enum InferenceRule {
    DoubleCutDraw {
        /// the parent subgraph that contains the things we want to include in the double cut
        target: GraphKey,

        /// the atoms we want to include
        target_atoms: Vec<Arc<Atom>>,

        /// the subgraphs we want to include
        target_subgraphs: Vec<GraphKey>,
    },
    DoubleCutErase {
        /// the outer ring of the double cut
        target: GraphKey,
    },
    Insertion {
        /// target subgraph to insert in
        target: GraphKey,

        /// string representation of what to insert; needs to be valid graph syntax
        new_content: String,
    },
    Erasure {
        /// subgraphs to delete
        target_subgraphs: Vec<GraphKey>,

        // atoms to delete, in (subgraph_containing_atom, atom) format
        target_atoms: Vec<(GraphKey, Arc<Atom>)>,
    },
    Iteration {
        /// whether this is iteration (false) or deiteration(true)
        backwards: bool,

        /// the parent subgraph that contains the things we want to include in the iteration
        parent: GraphKey,

        /// the atoms we want to include
        parent_atoms: Vec<Arc<Atom>>,

        /// the subgraphs we want to include
        parent_subgraphs: Vec<GraphKey>,

        /// where the selected atoms/subgraphs should go
        target: GraphKey,
    },
}

impl InferenceRule {
    #[instrument]
    pub fn gen_actions_from_rule(
        &self,
        graph: &Graph,
    ) -> ProofResult<impl IntoIterator<Item = Action> + Debug> {
        match self {
            InferenceRule::DoubleCutDraw {
                target: _,
                target_atoms: _,
                target_subgraphs: _,
            } => gen_actions_from_double_cut_draw(self, graph),

            InferenceRule::DoubleCutErase { target: _ } => {
                gen_actions_from_double_cut_erase(self, graph)
            }

            InferenceRule::Insertion {
                target: _,
                new_content: _,
            } => gen_actions_from_insertion(self, graph),

            InferenceRule::Erasure {
                target_subgraphs: _,
                target_atoms: _,
            } => gen_actions_from_erasure(self, graph),

            InferenceRule::Iteration {
                backwards: _,
                parent: _,
                parent_atoms: _,
                parent_subgraphs: _,
                target: _,
            } => gen_actions_from_iteration(self, graph),
        }
    }
}
