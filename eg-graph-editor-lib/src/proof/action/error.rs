use crate::graph::error::GraphError;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ActionError {
    #[error("Future graph {0} was not defined in the list of actions")]
    UndefinedFutureGraph(usize),

    #[error("Cannot make a new subgraph with id \"{0}\" when it already exists")]
    SubgraphIdAlreadyExists(String),

    #[error("Error operating on graph: {0}")]
    GraphError(#[from] GraphError),
}

pub type ActionResult<T> = Result<T, ActionError>;
