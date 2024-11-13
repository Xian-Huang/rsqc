#[cfg(test)]
mod test {
    use rsqc::reader::{data_loader, DataFrame, ReaderConfig};

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
}
