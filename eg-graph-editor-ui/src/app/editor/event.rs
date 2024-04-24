use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export)]
#[ts(tag = "type")]
pub enum Event {
    Request(Request),
    Response(Result<(), String>),
}

#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export)]
#[ts(tag = "type")]
pub enum Request {
    // editor requests
    NewAtom {
        atom: String,
        parent: String,
    },
    NewSubgraph {
        parent: String,
    },
    MoveSubgraph {
        target: String,
        dest: String,
    },
    DeleteAtom {
        atom: String,
        parent: String,
    },
    DeleteSubgraph {
        atom: String,
        parent: String,
    },
    // rule requests
    DoubleCutDraw {
        target: String,
        target_atoms: Vec<String>,
        target_subgraphs: Vec<String>,
    },
    DoubleCutErase {
        target: String,
    },
    Insertion {
        target: String,
        new_content: String,
    },
    Erasure {
        target_subgraphs: Vec<String>,
        target_atoms: Vec<(String, String)>,
    },
    Iteration {
        backwards: bool,
        parent: String,
        parent_atoms: Vec<String>,
        parent_subgraphs: Vec<String>,
        target: String,
    },
}

#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export)]
#[ts(tag = "type")]
pub enum Action {
    AddAtom {
        target: String,
        atom: String,
    },
    DeleteAtom {
        target: String,
        atom: String,
    },
    AddSubgraph {
        target: String,
        new_subgraph: String,
    },
    DeleteSubgraph {
        target: String,
    },
    MoveSubgraph {
        target: String,
        dest: String,
    },
}
