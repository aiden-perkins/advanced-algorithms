use std::{fs, thread};
use std::path::Path;
use std::io::Write;
use std::sync::Arc;

pub mod closest_point_pair;
pub mod matching;
pub mod chromatic_number;
pub mod vertex_cover;
pub mod independent_set;

fn main() -> std::io::Result<()> {
    let file_paths = vec![
        "./tests/independent-set/data/1.txt",
        "./tests/independent-set/data/2.txt",
        "./tests/independent-set/data/3.txt",
        "./tests/independent-set/data/4.txt",
        "./tests/independent-set/data/5.txt",
        "./tests/independent-set/data/6.txt",
        "./tests/independent-set/data/7.txt",
        "./tests/independent-set/data/8.txt",
        "./tests/independent-set/data/9.txt",
        "./tests/independent-set/data/10.txt",
    ];

    let file_paths = Arc::new(file_paths);
    let mut handles = vec![];

    for (i, _) in file_paths.iter().enumerate() {
        let file_paths = Arc::clone(&file_paths);
        let handle = thread::spawn(move || {
            let result = independent_set::independent_set(&file_paths[i]);
            println!("File {}: {}", i + 1, result);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
    // , UndirectedEdge[15850, 0]
    // this goes at the end of 2.txt in vertex-cover data
    
    // , UndirectedEdge[5, 0]
    // this goes at the end of line 7 of 2.txt in independent-set data

fn mod_old_data() -> std::io::Result<()> {
    for i in 0..11 {
        let input_path = format!("./tests/closest-point-pair/old-data/{}.txt", i);
        let output_path = format!("./tests/closest-point-pair/mod-old-data/{}.txt", i);
        let file_contents = fs::read_to_string(&input_path)?;
        let contents_vec: Vec<&str> = file_contents.split(", ").collect();
        let mut mod_content = String::new();
        for j in 0..((contents_vec.len() / 7) + 1) {
            let start = j * 7;
            let end = (j + 1) * 7;
            let chunk: Vec<&str> = contents_vec[start..std::cmp::min(end, contents_vec.len())].to_vec();
            mod_content.push_str(&chunk.join(", "));
            mod_content.push_str(", \n");
        }
        mod_content = mod_content.trim_end_matches(", \n").to_string();
        if let Some(parent) = Path::new(&output_path).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut mod_file = fs::File::create(&output_path)?;
        mod_file.write_all(mod_content.as_bytes())?;

        println!("Wrote modified content to {}", output_path);
    }

    Ok(())
}
