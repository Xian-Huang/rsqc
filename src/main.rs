use clap::Parser;
use rsqc::{
    args::Args,
    reader::{data_loader, ReaderConfig},
    render::{Render, RenderConfig, Source},
};
use tera::Context;

fn main() {
    let args = Args::parse();
    let config = ReaderConfig::new(args.number_header, args.data); //读取配置
    let _df = data_loader(&config);
    let table = vec![vec!["1", "23", "4", "5", "4"]
        .iter()
        .map(|&x| x.to_string())
        .collect()];
    let product = Source {
        time: "20240-11-14".to_string(),
        table,
        images: vec!["src/images/image.png".to_string()],
    };
    let render_config = RenderConfig {
        template: "D:/ldy/qc_report/template".to_string(),
        context: Context::from_serialize(product).unwrap(),
        output: "./".to_string(),
    };
    let render = Render::new(render_config);
    let _ = render.render();
}
