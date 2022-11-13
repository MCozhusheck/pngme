use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(about = "A CLI tool to encode and decode secret messages in png file", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encodes secret message into PNG file
    Encode {
        #[arg(required = true)]
        path: Option<OsString>,

        #[arg(required = true)]
        chunk: Option<OsString>,

        #[arg(required = true)]
        message: Option<OsString>,
    },

    /// Decodes secret message in given chunk
    Decode {
        #[arg(required = true)]
        path: Option<OsString>,

        #[arg(required = true)]
        chunk: Option<OsString>,
    },

    /// Deletes secret message
    Remove {
        #[arg(required = true)]
        path: Option<OsString>,

        #[arg(required = true)]
        chunk: Option<OsString>,
    },

    /// Prints whole file
    Print {
        #[arg(required = true)]
        path: Option<OsString>,
    },
}
