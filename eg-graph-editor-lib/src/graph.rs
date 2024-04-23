pub mod error;
mod subgraph;

use self::{
    error::{GraphError, GraphResult},
    subgraph::Subgraph,
};
use crate::{
    atom::Atom,
    proof::action::{error::ActionError, Action, GraphTarget},
};
use itertools::Itertools;
use nid::Nanoid;
use std::{
    collections::{HashMap, HashSet, LinkedList},
    fmt::{Debug, Write},
    mem,
    sync::Arc,
};
use tracing::instrument;

const ID_LEN: usize = 10;
pub type GraphKey = Nanoid<ID_LEN>;

#[derive(Clone)]
pub struct Graph {
    root_id: GraphKey,
    known_atoms: HashMap<Arc<Atom>, usize>,
    subgraphs: HashMap<GraphKey, Subgraph>,
    subgraph_parents: HashMap<GraphKey, GraphKey>,
}

impl Graph {
    #[instrument]
    pub fn new() -> Self {
        let root_id = GraphKey::new();

        let mut g = Self {
            root_id,
            known_atoms: Default::default(),
            subgraphs: Default::default(),
            subgraph_parents: Default::default(),
        };

        g.subgraphs.insert(root_id, Subgraph::new(0));
        g
    }

    #[instrument]
    pub fn root_id(&self) -> &GraphKey {
        &self.root_id
    }

    #[instrument]
    pub fn atoms(&self) -> impl Iterator<Item = &Arc<Atom>> {
        self.known_atoms.keys()
    }

    #[instrument]
    pub fn atoms_of(&self, target: &GraphKey) -> GraphResult<&HashSet<Arc<Atom>>> {
        self.subgraphs
            .get(target)
            .ok_or_else(|| GraphError::InvalidSubgraphTargetError(target.to_string()))
            .map(|v| v.atoms())
    }

    #[instrument]
    pub fn subgraphs_of(&self, target: &GraphKey) -> GraphResult<&HashSet<GraphKey>> {
        self.subgraphs
            .get(target)
            .ok_or_else(|| GraphError::InvalidSubgraphTargetError(target.to_string()))
            .map(|v| v.subgraphs())
    }

    #[instrument]
    pub fn parent_of(&self, target: &GraphKey) -> GraphResult<&GraphKey> {
        self.subgraph_parents
            .get(target)
            .ok_or_else(|| GraphError::InvalidSubgraphTargetError(target.to_string()))
    }

    #[instrument]
    pub fn level_of(&self, target: &GraphKey) -> GraphResult<usize> {
        self.subgraphs
            .get(target)
            .ok_or_else(|| GraphError::InvalidSubgraphTargetError(target.to_string()))
            .map(|v| v.level())
    }

    #[instrument]
    pub fn insert_atom(
        &mut self,
        target: &GraphKey,
        atom: impl Into<Atom> + Debug,
    ) -> GraphResult<()> {
        if self.subgraphs.contains_key(target) {
            let atom = Arc::new(atom.into());

            self.known_atoms
                .entry(atom.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);

            self.subgraphs
                .get_mut(target)
                .unwrap()
                .atoms_mut()
                .insert(atom);

            Ok(())
        } else {
            Err(GraphError::InvalidSubgraphTargetError(target.to_string()))
        }
    }

    #[instrument]
    pub(crate) fn insert_subgraph_with_id(
        &mut self,
        id: GraphKey,
        target: &GraphKey,
    ) -> GraphResult<GraphKey> {
        if !self.subgraphs.contains_key(target) {
            Err(GraphError::InvalidSubgraphTargetError(target.to_string()))
        } else if self.subgraphs.contains_key(&id) {
            Err(GraphError::DuplicateIdError(id.to_string()))
        } else {
            let new_level = self.subgraphs.get(target).unwrap().level() + 1;

            self.subgraphs.insert(id, Subgraph::new(new_level));
            self.subgraphs
                .get_mut(target)
                .unwrap()
                .subgraphs_mut()
                .insert(id);
            self.subgraph_parents.insert(id, *target);

            Ok(id)
        }
    }

    #[instrument]
    pub fn insert_subgraph(&mut self, target: &GraphKey) -> GraphResult<GraphKey> {
        self.insert_subgraph_with_id(self.gen_unique_unused_key(), target)
    }

    #[instrument]
    pub fn move_subgraph(&mut self, target: &GraphKey, dest: &GraphKey) -> GraphResult<()> {
        if &self.root_id == target {
            Err(GraphError::InvalidRootGraphTargetError)?
        }

        let src = *self.parent_of(target)?;
        let src = &src;

        for id in [src, target, dest] {
            if !self.subgraphs.contains_key(id) {
                Err(GraphError::InvalidSubgraphTargetError(id.to_string()))?
            }
        }

        if !self.subgraphs_of(src).unwrap().contains(target) {
            Err(GraphError::InvalidSubgraphTargetError(target.to_string()))?
        }

        self.subgraph_parents.insert(*target, *dest);

        self.subgraphs
            .get_mut(src)
            .unwrap()
            .subgraphs_mut()
            .remove(target);

        self.subgraphs
            .get_mut(dest)
            .unwrap()
            .subgraphs_mut()
            .insert(*target);

        let new_level = self.subgraphs.get(dest).unwrap().level() + 1;
        self.subgraphs.get_mut(target).unwrap().set_level(new_level);

        Ok(())
    }

    #[instrument]
    pub fn remove_atom_from_subgraph(
        &mut self,
        target: &GraphKey,
        atom: &Atom,
    ) -> GraphResult<Option<Atom>> {
        if self.subgraphs.contains_key(target) {
            self.subgraphs
                .get_mut(target)
                .unwrap()
                .atoms_mut()
                .remove(atom);

            Ok(self.decrement_atom(atom))
        } else {
            Err(GraphError::InvalidSubgraphTargetError(target.to_string()))
        }
    }

    #[instrument]
    pub fn remove_subgraph(&mut self, target: &GraphKey, recursive: bool) -> GraphResult<()> {
        if target == &self.root_id {
            Err(GraphError::InvalidRootGraphTargetError)
        } else if !self.subgraphs.contains_key(target)
            || (!recursive
                && (!self.subgraphs.get(target).unwrap().atoms().is_empty()
                    || !self.subgraphs.get(target).unwrap().subgraphs().is_empty()))
        {
            Err(GraphError::InvalidSubgraphTargetError(target.to_string()))
        } else {
            let parent_id = self.subgraph_parents.remove(target).unwrap();

            if let Some(x) = self.subgraphs.get_mut(&parent_id) {
                if !x.subgraphs_mut().remove(target) {
                    panic!("Parent graph does not have target registered as child")
                }
            }

            let mut removed_graph = self.subgraphs.remove(target).unwrap();

            mem::take(removed_graph.atoms_mut())
                .into_iter()
                .for_each(|v| {
                    self.decrement_atom(&v);
                });

            mem::take(removed_graph.subgraphs_mut())
                .into_iter()
                .for_each(|c| {
                    self.remove_subgraph(&c, recursive).unwrap();
                });

            Ok(())
        }
    }

    #[instrument]
    pub(crate) fn gen_unique_unused_key(&self) -> GraphKey {
        loop {
            let gen_id = GraphKey::new();

            if !self.subgraphs.contains_key(&gen_id) {
                break gen_id;
            }
        }
    }

    #[instrument]
    fn decrement_atom(&mut self, atom: &Atom) -> Option<Atom> {
        *self.known_atoms.get_mut(atom).unwrap() -= 1;

        if self.known_atoms.get(atom).unwrap() < &1_usize {
            self.known_atoms
                .remove_entry(atom)
                .map(|(k, _)| Arc::into_inner(k).unwrap())
        } else {
            None
        }
    }
}

impl Default for Graph {
    #[instrument]
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Graph> for String {
    #[instrument]
    fn from(graph: &Graph) -> Self {
        transform_graph_into_string(graph, graph.root_id())
    }
}

impl TryFrom<&str> for Graph {
    type Error = ActionError;

    #[instrument]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut graph = Graph::new();

        let actions = parse_graph_string_into_actions(&graph, value, *graph.root_id())?;

        Action::apply_actions(actions, &mut graph)?;

        Ok(graph)
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graph")
            .field("root_id", &self.root_id)
            // .field("known_atoms", &self.known_atoms)
            // .field("subgraphs", &self.subgraphs)
            // .field("subgraph_parents", &self.subgraph_parents)
            .finish()
    }
}

#[instrument]
pub fn transform_graph_into_string(graph: &Graph, id: &GraphKey) -> String {
    format!(
        "[{}]",
        Iterator::chain(
            graph.atoms_of(id).unwrap().iter().map(|e| e.to_string()),
            graph
                .subgraphs_of(id)
                .unwrap()
                .iter()
                .map(|e| transform_graph_into_string(graph, e))
        )
        .intersperse(",".to_string())
        .collect::<String>()
    )
}

#[instrument]
pub fn parse_graph_string_into_actions(
    graph: &Graph,
    substr: &str,
    root: GraphKey,
) -> GraphResult<LinkedList<Action>> {
    if !(substr.starts_with('[') && substr.ends_with(']')) {
        Err(GraphError::ParseError)
    } else {
        let mut actions = LinkedList::new();

        let mut levels = LinkedList::new();
        let mut counter = 0;
        let mut curr_atom = String::new();

        for c in substr.chars() {
            match c {
                '[' => {
                    if curr_atom.is_empty() {
                        levels.push_front(if levels.is_empty() {
                            GraphTarget::Exists(root)
                        } else {
                            counter += 1;

                            let ans = GraphTarget::Future(counter);

                            actions.push_back(Action::AddSubgraph {
                                target: levels.front().ok_or(GraphError::ParseError).cloned()?,
                                new_subgraph: ans.clone(),
                            });

                            ans
                        });
                    } else {
                        Err(GraphError::ParseError)?
                    }
                }
                ']' => {
                    if !curr_atom.is_empty() {
                        actions.push_back(Action::AddAtom {
                            target: levels.front().ok_or(GraphError::ParseError).cloned()?,
                            atom: mem::take(&mut curr_atom).into(),
                        });
                    }

                    levels.pop_front();
                }
                ',' => {
                    if !curr_atom.is_empty() {
                        actions.push_back(Action::AddAtom {
                            target: levels.front().ok_or(GraphError::ParseError).cloned()?,
                            atom: mem::take(&mut curr_atom).into(),
                        });
                    }
                }
                c if !c.is_whitespace() => {
                    curr_atom
                        .write_char(c)
                        .map_err(|_| GraphError::ParseError)?;
                }
                _ => (),
            }
        }

        if levels.is_empty() {
            Ok(actions)
        } else {
            Err(GraphError::ParseError)
        }
    }
}
