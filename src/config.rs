#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn can_read_toml_file() {
        // Arrange: 一時的なtomlファイルを作る
        let path = "test_config.toml";
        fs::write(path, "foo = 'bar'").unwrap();

        // Act: ファイルを読み込む
        let content = fs::read_to_string(path).unwrap();

        // Assert: 中身が期待通り
        assert!(content.contains("foo"));

        // Clean up
        fs::remove_file(path).unwrap();
    }
}
