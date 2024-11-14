use std::{
    fs::{self},
    io::Write,
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
        let tera = match Tera::new("D:/ldy/qc_report/template/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        let render_string = tera.render("index.html", &self.config.context).unwrap();
        let mut file = fs::File::create("D:/ldy/qc_report/template/main.html").unwrap();
        let _ = file.write(render_string.into_bytes().as_slice());
    }
}
