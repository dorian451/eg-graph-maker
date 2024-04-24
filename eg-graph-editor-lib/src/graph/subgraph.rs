use super::GraphKey;
use crate::atom::Atom;
use hashlink::LinkedHashSet;
use std::{collections::HashSet, sync::Arc};
use tracing::instrument;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subgraph {
    level: usize,
    atoms: HashSet<Arc<Atom>>,
    subgraphs: LinkedHashSet<GraphKey>,
}

impl Subgraph {
    #[instrument]
    pub fn new(level: usize) -> Self {
        Self {
            level,
            atoms: Default::default(),
            subgraphs: Default::default(),
        }
    }

    #[instrument]
    pub fn level(&self) -> usize {
        self.level
    }

    #[instrument]
    pub fn set_level(&mut self, level: usize) {
        self.level = level;
    }

    #[instrument]
    pub fn atoms(&self) -> &HashSet<Arc<Atom>> {
        &self.atoms
    }

    #[instrument]
    pub fn atoms_mut(&mut self) -> &mut HashSet<Arc<Atom>> {
        &mut self.atoms
    }

    #[instrument]
    pub fn subgraphs(&self) -> &LinkedHashSet<GraphKey> {
        &self.subgraphs
    }

    #[instrument]
    pub fn subgraphs_mut(&mut self) -> &mut LinkedHashSet<GraphKey> {
        &mut self.subgraphs
    }
}
