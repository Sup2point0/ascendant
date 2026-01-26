#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#![allow(dead_code)]
#![allow(unused_parens)]

use clap::Parser;

use ascendant::*;


fn main()
{
    println!(">> Running ascendant...");

    let cli = cli::Cli::parse();
    cli.exec();
}
