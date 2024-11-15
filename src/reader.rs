use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct ReaderConfig {
    /// header 目前能为1
    pub header: u32,
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

impl ReaderConfig {
    pub fn new(header: u32, path: String) -> Self {
        ReaderConfig { header, path }
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
            let s = line
                .get(index)
                .expect(format!("未找到{}位置表头！", index).as_str());
            res.push(s.clone())
        }
        res
    }
    pub fn get_by_colname(self: &Self, colname: &str) -> Vec<String> {
        let index = self
            .header
            .iter()
            .position(|x| x == colname)
            .expect(format!("未发现colename={}", colname).as_str());
        self.get_by_col(index.try_into().unwrap())
    }

    pub fn get_by_colnames(self: &Self, colnames: Vec<String>) -> HashMap<String, Vec<String>> {
        let mut hx: HashMap<String, Vec<String>> = HashMap::new();
        for colname in colnames {
            let col = colname.clone();
            let var_name = colname.as_str();
            hx.insert(col, self.get_by_colname(&var_name));
        }
        hx
    }

    pub fn create_table(
        self: &Self,
        sample_name: &String,
        q_name: &String,
        v_name: &String,
        res_name: &String,
        v: f32,
        result: String,
        wk: String,
    ) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut colnames: Vec<String> = vec![
            sample_name.clone(),
            q_name.clone(),
            v_name.clone(),
            res_name.clone(),
        ];
        colnames.retain(|x| x.len() != 0);
        let hx = self.get_by_colnames(colnames);
        let mut keys: Vec<&String> = hx.keys().collect();
        keys.sort(); //确定顺序
        for number in 0..self.data.len() {
            let index = number + 1 as usize;
            let sample = hx.get(sample_name).unwrap().get(number).unwrap().clone();
            let q = hx.get(q_name).unwrap().get(number).unwrap();
            let cv: String = match hx.contains_key(v_name) {
                true => hx.get(v_name).unwrap().get(number).unwrap().clone(),
                false => v.to_string(),
            };
            let cresult: String = match hx.contains_key(res_name) {
                true => hx.get(res_name).unwrap().get(number).unwrap().clone(),
                false => result.clone(),
            };
            let all = q.parse::<f32>().unwrap() * v / 1000.;
            res.push(vec![
                index.to_string(),
                sample.clone(),
                wk.clone(),
                q.clone(),
                cv,
                all.to_string(),
                cresult,
            ]);
        }
        res
    }
}
pub fn data_loader(config: &ReaderConfig) -> DataFrame {
    let file =
        File::open(config.path.to_string()).expect("数据文件未找到，请检查文件是否存在！--Data");
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
