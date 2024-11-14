use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// 数据位置
    #[arg(short, long)]
    data: String,

    /// 表头行数
    #[arg(short, long, default_value = "0")]
    number_header: u32,

    /// 样本名称列
    #[arg(short, long, default_value = "sdf")]
    sample: String,

    /// 浓度列名称
    #[arg(short, long, default_value = "sdf")]
    ch: String,

    ///体积
    #[arg(short, long)]
    volume: f32,

    ///胶图位置
    #[arg(short, long, default_value = "./images")]
    images: String,
}
