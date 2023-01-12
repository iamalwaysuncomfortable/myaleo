// Copyright (C) 2019-2022 Aleo Systems Inc.

use clap::Parser;
use myaleo::CLI;

fn main() {
    let cli = CLI::parse();
    cli.command.run();
}
