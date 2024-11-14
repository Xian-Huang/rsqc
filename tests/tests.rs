#[cfg(test)]
mod test {
    use std::path::Path;

    use rsqc::{
        get_time,
        reader::{data_loader, DataFrame, ReaderConfig},
        render::load_images,
    };

    fn get_df() -> DataFrame {
        let readercfg = ReaderConfig::default();
        let df = data_loader(&readercfg);
        df
    }

    #[test]
    fn reader_test() {
        // 测试data_loader
        let df = get_df();
        assert_eq!(df.data.len(), 22);
    }

    #[test]
    fn get_col_data_test() {
        let df = get_df();
        let line = df.get_by_col(0);
        assert_eq!(line.get(0).unwrap(), "353");
    }

    #[test]
    fn load_images_test() {
        let res = load_images(Path::new("./tests/images"));
        dbg!(res);
    }

    #[test]
    fn get_tiem_test() {
        get_time();
    }
}
