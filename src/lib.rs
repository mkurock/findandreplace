use glob::glob;
use std::{collections::HashMap, fs, path::PathBuf};

pub fn findanreplace(directory: &str, config_path: &PathBuf) {
    println!("directory to process: {directory}");
    let contents = fs::read_to_string(config_path).expect("File could not be read");
    let config: HashMap<String, HashMap<String, String>> =
        serde_yaml::from_str(&contents).expect("Failed to parse config");

    for (k, v) in config.iter() {
        println!("Processing glob: {k}");
        for file in glob(&format!("{directory}/{k}")).expect("failed to read file glob") {
            let path = match file {
                Ok(path) => path,
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };
            println!("Processing file: {}", path.display());
            let mut file_contents = match fs::read_to_string(&path) {
                Ok(str) => str,
                Err(_) => continue,
            };
            for (b, a) in v.iter() {
                file_contents = file_contents.replace(b, a);
            }
            if let Err(_) = fs::write(&path, file_contents) {
                println!("Failed to write file");
            };
        }
    }
}
