use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum EditCommand {
    /// Insert at an atom at the cut marked by <target>
    #[command(visible_alias = "na")]
    NewAtom { target: String, atom: String },

    /// Insert a new sub cut at the cut marked by <target>
    #[command(visible_alias = "nc")]
    NewCut { target: String },

    /// Delete an atom belonging to <target>
    #[command(visible_alias = "da")]
    DeleteAtom { target: String, atom: String },

    /// Delete a cut
    #[command(visible_alias = "dc")]
    DeleteCut { target: String },

    ///Load a new subgraph
    /// WARNING: WILL OVERWRITE THE OLD GRAPH
    #[command(visible_alias = "l")]
    Load { new_graph: String },
}
