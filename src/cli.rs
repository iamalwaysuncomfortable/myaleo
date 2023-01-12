// Copyright (C) 2019-2022 Aleo Systems Inc.

use clap::Parser;

/// myaleo CLI options
#[derive(Debug, Parser)]
#[clap(name = "myaleo", author = "The Aleo Team <hello@aleo.org>")]
pub struct CLI {
    /// myaleo Subcommands
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Update snarkVM to the latest version
    Create {
        /// Specify file format for key material output [options: json, pem]
        #[clap(default_value = "none", short = 'f', long)]
        format: String,
    },
    Import {
        /// Specify path to key material file
        #[clap(short = 'f', long)]
        path: String,
    },
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Create { format } => {
                let keys = crate::keys::AleoKeys::new().unwrap();
                match format.as_str() {
                    "json" => {
                        println!("{}", serde_json::to_string_pretty(&keys).unwrap());
                    }
                    _ => {
                        println!("Your new account keys!");
                        keys.pretty_print();
                    }
                }
            }
            Command::Import { path } => {
                println!("Importing key material from {}", path);
            }
        }
    }
}
