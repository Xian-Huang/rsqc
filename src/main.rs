use clap::Parser;
use rsqc::args::Args;

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
