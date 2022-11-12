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

    Encode {
        #[arg(required = true)]
        path: Option<OsString>,

        #[arg(required = true)]
        chunk: Option<OsString>,

        #[arg(required = true)]
        message: Option<OsString>,
    },

    Decode {
        #[arg(required = true)]
        path: Option<OsString>,

        #[arg(required = true)]
        chunk: Option<OsString>,
    },

    Remove {
        #[arg(required = true)]
        path: Option<OsString>,

        #[arg(required = true)]
        chunk: Option<OsString>,
    },

    Print {
        #[arg(required = true)]
        path: Option<OsString>,
    }
}