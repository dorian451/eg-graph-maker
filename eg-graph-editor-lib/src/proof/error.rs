use thiserror::Error;

use crate::graph::error::GraphError;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ProofError {
    #[error("Invalid application of rule: {0}")]
    InvalidApplicationOfRule(String),

    #[error("Error operating on graph: {0}")]
    GraphError(#[from] GraphError),
}

pub type ProofResult<T> = Result<T, ProofError>;
