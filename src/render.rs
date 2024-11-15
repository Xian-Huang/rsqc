use std::{
    fs::{self, read_dir},
    io::Write,
    path::Path,
};

use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
pub struct Source {
    pub time: String,
    pub table: Vec<Vec<String>>,
    pub images: Vec<String>,
}

pub struct RenderConfig {
    pub template: String,
    pub context: Context,
    pub output: String,
}

pub struct Render {
    pub config: RenderConfig,
}

impl Render {
    pub fn new(config: RenderConfig) -> Self {
        Render { config }
    }
    pub fn render(self: &Self) {
        let tempale_path = Path::new(&self.config.template)
            .canonicalize()
            .unwrap()
            .join("*.html");

        let output_path = Path::new(&self.config.template)
            .canonicalize()
            .unwrap()
            .join("report.html");

        let tera = match Tera::new(tempale_path.as_path().to_str().unwrap()) {
            Ok(t) => t,
            Err(e) => {
                println!("构建Tera Teamplte Env 错误！: {}", e);
                ::std::process::exit(1);
            }
        };
        let render_string = tera
            .render("template.html", &self.config.context)
            .expect(format!("未找到模板文件template.html").as_str());
        let mut file = fs::File::create(output_path).expect(
            format!("创建report.html失败， 请检查权限以及文件是否正在运行，若正在运行，请关闭！")
                .as_str(),
        );
        let _ = file.write(render_string.into_bytes().as_slice());
    }
}

pub fn load_images(path: &Path) -> Vec<String> {
    let mut res = vec![];
    for file in read_dir(path).unwrap() {
        let file = file.unwrap();
        let filepath = file.path();
        dbg!(&filepath);
        if filepath.as_path().to_str().unwrap().ends_with(".png")
            || filepath.as_path().to_str().unwrap().ends_with(".jpg")
        {
            let new_path = filepath
                .canonicalize()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .replace("\\\\?\\", "")
                .replace("\\", "/");
            res.push(format!("file:\\{}", new_path));
        }
    }
    res
}
