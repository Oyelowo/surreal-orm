/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use clap::Parser;

#[derive(Parser)]
#[clap(name = "MyApp", about = "Does awesome things")]
struct Cli {
    #[clap(long)]
    two: String,
    #[clap(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    log::info!("two: {:?}", cli.two);
    log::info!("one: {:?}", cli.one);
}
