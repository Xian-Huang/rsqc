use std::{
    fs::File,
    i32,
    io::{BufRead, BufReader},
    simd::usizex1,
};

pub struct ReaderConfig {
    /// header 目前能为1
    pub header: i32,
    pub path: String,
}

impl Default for ReaderConfig {
    fn default() -> ReaderConfig {
        ReaderConfig {
            header: 1,
            path: "./data.csv".to_string(),
        }
    }
}

pub struct DataFrame {
    pub data: Vec<Vec<String>>,
    pub header: Vec<String>,
}

impl DataFrame {
    fn new(header: Vec<String>, data: Vec<Vec<String>>) -> Self {
        DataFrame {
            data: data,
            header: header,
        }
    }

    fn get_by_col(self: &Self, index: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for line in self.data {
            res.push(line.get(index))
        }
        res
    }
    fn get_by_colname(self: &Self, colname: &str) {
        let index = self.header.iter().position(|&x| x == colname);
        self.get_by_col(index)
    }
}

pub fn data_loader(config: &ReaderConfig) -> DataFrame {
    let file = File::open(config.path.to_string()).expect("文件未找到！");
    let buffreader = BufReader::new(file);

    let mut res: Vec<Vec<String>> = vec![];
    let mut header: Vec<String> = vec![];

    for (rownumber, line) in buffreader.lines().enumerate() {
        let line = line.unwrap();
        let row: &Vec<String> = &line.split("\t").map(|x| x.to_string()).collect();

        if rownumber == 0 {
            header = row.clone();
        } else {
            res.push(row.clone());
        }
    }
    DataFrame::new(header, res)
}
