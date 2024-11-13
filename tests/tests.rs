#[cfg(test)]
mod test {
    use rsqc::reader::{data_loader, ReaderConfig};


    #[test]
    fn reader_test() {
        // 测试data_loader
        let readercfg = ReaderConfig::default();
        let df = data_loader(&readercfg);
        assert_eq!(df.data.len(), 21);
    }
}
