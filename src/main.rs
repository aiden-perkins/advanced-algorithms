use std::fs;
use std::path::Path;
use std::io::Write;

pub mod closest_point_pair;

fn main() -> std::io::Result<()> {
    println!("{}", closest_point_pair::divide_and_conquer("./tests/closest-point-pair/mod-old-data/2.txt"));
    println!("{}", closest_point_pair::brute_force("./tests/closest-point-pair/mod-old-data/2.txt"));

    Ok(())
}

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
