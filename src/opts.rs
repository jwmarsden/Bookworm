use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Parser)]
#[command(name = "Bookworm")]
#[command(about = "Its a very hungry worm.", long_about = None)]
pub struct Opts {
    // The action for Bookworm to execute. 
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Debug, Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Backup Repository
    Backup {
        /// Source Path (File Path)
        #[arg(short='s', long="source", default_value_t = String::from("."))]
        source: String,

        /// Destination Path (File Path)
        #[arg(short='d', long="destination", required=true)]
        destination: String,
    },
    /// Clean Repository
    Clean {

    },
    /// Generate Reports for Repository
    Report {
        
    },
}

impl Opts {
    pub fn parse_args() -> Self {
        Self::parse()
    }

}