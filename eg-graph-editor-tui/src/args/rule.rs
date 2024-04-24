use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum RuleCommand {
    #[command(visible_alias = "dcd")]
    DoubleCutDraw {
        /// the parent subgraph that contains the things we want to include in the double cut
        target: String,

        /// the atoms we want to include
        target_atoms: Vec<String>,

        /// the subgraphs we want to include
        target_subgraphs: Vec<String>,
    },
    #[command(visible_alias = "dce")]
    DoubleCutErase {
        /// the outer ring of the double cut
        target: String,
    },
    #[command(visible_alias = "in")]
    Insertion {
        /// target subgraph to insert in
        target: String,

        /// string representation of what to insert; needs to be valid graph syntax
        new_content: String,
    },
    #[command(visible_alias = "ea")]
    ErasureAtoms {
        ///The cut containing all the atoms we want to delete
        parent_subgraph: String,

        ///The atoms to delete
        atoms: Vec<String>,
    },
    #[command(visible_alias = "ec")]
    ErasureCuts {
        /// subgraphs to delete
        target_subgraphs: Vec<String>,
    },
    #[command(visible_alias = "itera")]
    Iteration_a {
        /// the parent subgraph that contains the things we want to include in the iteration
        parent: String,
        /// where the selected atoms/subgraphs should go
        target: String,

        /// the subgraphs we want to include
        parent_subgraphs: Vec<String>,
    },
    #[command(visible_alias = "iterg")]
    Iteration_g {
        /// the parent subgraph that contains the things we want to include in the iteration
        parent: String,

        /// where the selected atoms/subgraphs should go
        target: String,
        /// the atoms we want to include
        parent_atoms: Vec<String>,
    },
    // #[command(visible_alias = "deiter")]
    // Deiteration {
    //     /// the parent subgraph that contains the things we want to include in the iteration
    //     parent: String,

    //     /// the atoms we want to include
    //     parent_atoms: Vec<String>,

    //     /// the subgraphs we want to include
    //     parent_subgraphs: Vec<String>,

    //     /// where the selected atoms/subgraphs should go
    //     target: String,
    // },
}
