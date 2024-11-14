use std::path::Path;

use clap::Parser;
use rsqc::{
    args::Args,
    get_time,
    reader::{data_loader, ReaderConfig},
    render::{load_images, Render, RenderConfig, Source},
};
use tera::Context;

fn main() {
    let args = Args::parse();
    let config = ReaderConfig::new(args.number_header, args.data); //读取配置
    let df = data_loader(&config);
    let images = load_images(Path::new(&args.images));
    let product = Source {
        time: get_time(),
        table: df.create_table(vec![args.sample, args.ch], args.volume, args.note),
        images: images,
    };
    let render_config = RenderConfig {
        template: args.template,
        context: Context::from_serialize(product).unwrap(),
        output: args.output_path,
    };
    let render = Render::new(render_config);
    let _ = render.render();
}
