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
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        let render_string = tera.render("template.html", &self.config.context).unwrap();
        let mut file = fs::File::create(output_path).unwrap();
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
