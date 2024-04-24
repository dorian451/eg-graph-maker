pub mod edit;
pub mod mode;
pub mod rule;

use self::{edit::EditCommand, mode::Mode, rule::RuleCommand};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    ///Shows current graph
    #[command(visible_alias = "s")]
    Show,

    ///Exits
    #[command(visible_alias = "q")]
    Exit,

    ///Toggle Editor Mode
    #[command(visible_alias = "m")]
    Mode { mode: Option<Mode> },

    ///Edit the graph in some way; requires editor mode
    #[command(visible_alias = "e")]
    Edit {
        #[command(subcommand)]
        how: EditCommand,
    },

    /// Edit the graph by applying a logical equivalence
    #[command(visible_alias = "p")]
    Rule {
        #[command(subcommand)]
        how: RuleCommand,
    },

    #[command(visible_alias = "u")]
    Undo {
        #[arg(default_value_t = 1)]
        times: u8,
    },

    #[command(visible_alias = "r")]
    Redo {
        #[arg(default_value_t = 1)]
        times: u8,
    },
}
