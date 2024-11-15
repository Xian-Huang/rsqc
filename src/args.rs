use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// 数据位置
    #[arg(short, long)]
    pub data: String,

    /// 表头行数
    #[arg(short, long, default_value = "1")]
    pub number_header: u32,

    /// 样本名称列
    #[arg(short, long, default_value = "Sample ID")]
    pub sample: String,

    /// 浓度列名称
    #[arg(short, long, default_value = "Nucleic Acid Conc.")]
    pub ch: String,

    ///体积
    #[arg(short, long)]
    pub volume: f32,

    ///体积列名称 此参数存在时 -v/volume无效
    #[arg(long, default_value = "")]
    pub volume_name: String,

    ///胶图位置
    #[arg(short, long, default_value = "./images")]
    pub images: String,

    /// 模板位置
    #[arg(short, long, default_value = "./template")]
    pub template: String,

    /// 模板位置
    #[arg(short, long, default_value = "./")]
    pub output_path: String,

    /// 文库类型
    #[arg(short, long, default_value = "DNA")]
    pub wk: String,

    /// 结论列名称
    #[arg(long, default_value = "合格")]
    pub res_name: String,
    
    /// result 是否合格
    #[arg(short, long, default_value = "合格")]
    pub result: String,
}
