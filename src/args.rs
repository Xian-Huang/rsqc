use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// 数据位置
    #[arg(short, long)]
    pub data: String,

    /// 表头行数
    #[arg(short, long, default_value = "0")]
    pub number_header: u32,

    /// 样本名称列
    #[arg(short, long, default_value = "sdf")]
    pub sample: String,

    /// 浓度列名称
    #[arg(short, long, default_value = "sdf")]
    pub ch: String,

    ///体积
    #[arg(short, long)]
    pub volume: f32,

    ///胶图位置
    #[arg(short, long, default_value = "./images")]
    pub images: String,
}
