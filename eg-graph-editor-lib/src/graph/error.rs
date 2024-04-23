use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GraphError {
    #[error("Cannot target root graph with this operation")]
    InvalidRootGraphTargetError,

    #[error("Graph with id {0} does not exist")]
    InvalidSubgraphTargetError(String),

    #[error("Graph already has a subgraph with id {0}")]
    DuplicateIdError(String),

    #[error("Could not parse graph from string")]
    ParseError,
}

pub type GraphResult<T> = Result<T, GraphError>;
