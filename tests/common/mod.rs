use std::{fs};

pub const REPLACE_YAML: &str = r###"
"test.txt":
  doreplace: newcontent
"test2.txt":
  doreplace: newcontent
"**/deep.txt":
  doreplace: newcontent
"###;
pub const REPLACETXT: &str = "testcontent: doreplace";
pub const NOREPLACETXT: &str = "testcontent: donotreplace";
pub const REPLACEDCONTENT: &str = "testcontent: newcontent";

pub const TEST_PATH: &str = "tests/tmp/";

pub fn setup() {
    fs::create_dir_all(format!("{}{}", TEST_PATH, "very/deep/")).expect("Error");
    fs::write(format!("{}{}", TEST_PATH, "replace.yaml"), REPLACE_YAML).expect("Error");

    fs::write(format!("{}{}", TEST_PATH, "test.txt"), REPLACETXT).expect("Error");
    fs::write(format!("{}{}", TEST_PATH, "testno.txt"), REPLACETXT).expect("Error");
    fs::write(format!("{}{}", TEST_PATH, "test2.txt"), NOREPLACETXT).expect("Error");
    fs::write(format!("{}very/deep/{}", TEST_PATH, "deep.txt"), REPLACETXT).expect("Error");
    fs::write(format!("{}{}", TEST_PATH, "deep.txt"), REPLACETXT).expect("Error");
}
pub fn cleanup() {
    fs::remove_dir_all(TEST_PATH).expect("Failed to remove test tmp dir");
}
