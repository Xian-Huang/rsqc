use std::{
    fs::File,
    i32,
    io::{BufRead, BufReader},
};

pub struct ReaderConfig {
    /// header 目前能为1
    pub header: i32,
    pub path: String,
}

impl Default for ReaderConfig {
    fn default() -> ReaderConfig {
        ReaderConfig {
            header: 0,
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

    pub fn get_by_col(self: &Self, index: usize) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for line in self.data.clone() {
            let s = line.get(index).unwrap();
            res.push(s.clone())
        }
        res
    }
    pub fn get_by_colname(self: &Self, colname: &str) -> Vec<String> {
        let index = self.header.iter().position(|x| x == colname);
        self.get_by_col(index.expect("Index Error").try_into().unwrap())
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
        if rownumber < config.header as usize {
            header = row.clone();
        } else {
            res.push(row.clone());
        }
    }
    return DataFrame::new(header, res);
}
