use std::{collections::HashMap, os::windows::prelude::MetadataExt};

use walkdir::WalkDir;

mod file;

fn main() {
    let a = WalkDir::new(format!(r#"D:\tencent"#));
    let mut file_extend_map = HashMap::new();
    for result in  a {
        if let Ok(file) = result {
            if file.path().is_file() {
                file_extend_map.insert(format!("{:#?}", file.file_name()), file);
            }
        }
        // let entry = entry.unwrap();
        // println!("{:?}",entry.metadata().unwrap());
        // println!("{}", entry.path().display());
    }
    println!("{}", file_extend_map.len());
    for (key, value) in file_extend_map {
        println!("{} / {:?}", key.trim_matches('"'), value.file_name());
        println!("{}", value.path().to_str().unwrap());
        println!("size: {:?}", value.metadata().unwrap().file_size());
        println!("info: {:?}", value.metadata().unwrap().file_attributes());
        println!("file type: {}", value.metadata().unwrap().is_symlink());
    }
    // println!("{:?}", file_extend_map);
}