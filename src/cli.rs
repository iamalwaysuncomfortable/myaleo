// Copyright (C) 2019-2022 Aleo Systems Inc.

use crate::AleoKeys;
use colored::Colorize;
use clap::Parser;
use snarkvm_console::account::private_key::PrivateKey;
use std::{fs, str::FromStr};

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
    /// Generate Aleo account keys
    Create {
        /// (Optional) Specify file format for key material output [options: json]
        #[clap(default_value = "none", short = 'f', long)]
        format: String,
    },
    Import {
        /// Private key to import account from
        #[clap(short = 'f', long)]
        key: String,
    },
}

impl Command {
    /// Run the parsed command
    pub fn run(&self) {
        match self {
            Command::Create { format } => {
                let keys = crate::keys::AleoKeys::new().unwrap();
                match format.as_str() {
                    "json" => {
                        println!("{}", "  The following output has been written to keys.json:".bright_blue().bold());
                        fs::write("keys.json", serde_json::to_string_pretty(&keys).unwrap()).unwrap();
                        keys.pretty_print();
                    }
                    _ => {
                        println!("{}", "  Your new Aleo account keys are below! Remember to store these in a secure location".bright_blue().bold());
                        keys.pretty_print();
                    }
                }
            }
            Command::Import { key } => {
                let private_key = PrivateKey::from_str(key).unwrap();
                println!("{}", "  The view key and Aleo account address corresponding to your private key are below!".bright_blue().bold());
                AleoKeys::try_from(&private_key).unwrap().pretty_print();
            }
        }
    }
}
