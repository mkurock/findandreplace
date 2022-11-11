mod common;

#[cfg(test)]
mod test {
    use std::{path::PathBuf, str::FromStr, fs};

    use crate::common;

    #[test]
    fn test_replacements() {
        common::setup();
        let config_path = PathBuf::from_str(&format!("{}{}", common::TEST_PATH, "replace.yaml")).expect("Failed to get config path");
        mk_findandreplace::findanreplace(common::TEST_PATH, &config_path);

        // test.txt
        let content = fs::read_to_string(format!("{}{}", common::TEST_PATH, "test.txt")).expect("failed to read test.txt");
        assert_eq!(content, common::REPLACEDCONTENT, "test.txt replacement failed");

        // testno.txt
        let content = fs::read_to_string(format!("{}{}", common::TEST_PATH, "testno.txt")).expect("failed to read testno.txt");
        assert_eq!(content, common::REPLACETXT, "testno.txt replacement should not happen");

        // test2.txt
        let content = fs::read_to_string(format!("{}{}", common::TEST_PATH, "test2.txt")).expect("failed to read test2.txt");
        assert_eq!(content, common::NOREPLACETXT, "test2.txt replacement should not happen");

        // deep.txt
        let content = fs::read_to_string(format!("{}{}", common::TEST_PATH, "deep.txt")).expect("failed to read deep.txt");
        assert_eq!(content, common::REPLACEDCONTENT, "deep.txt replacement should not happen");

        // very/deep/deep.txt
        let content = fs::read_to_string(format!("{}{}", common::TEST_PATH, "very/deep/deep.txt")).expect("failed to read very/deep/deep.txt");
        assert_eq!(content, common::REPLACEDCONTENT, "very/deep/deep.txt replacement should not happen");

        common::cleanup();
    }
}
