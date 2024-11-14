use clap::Parser;
use rsqc::{
    args::Args,
    reader::{data_loader, ReaderConfig},
};

fn main() {
    let args = Args::parse();
    let config = ReaderConfig::new(args.number_header, args.data); //读取配置
    let _df = data_loader(&config);
}
