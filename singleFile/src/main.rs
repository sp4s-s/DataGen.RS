use serde_json::{Value, Map};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() -> io::Result<()> {
    // Get user input for configuration
    let mut sample_file = String::new();
    let mut names_file = String::new();
    let mut places_file = String::new();
    let mut output_dir = String::new();
    let mut num_entries = String::new();
    let mut separate_files = String::new();

    println!("Enter the path to the sample format file:");
    io::stdin().read_line(&mut sample_file)?;

    println!("Enter the path to the names file:");
    io::stdin().read_line(&mut names_file)?;

    println!("Enter the path to the places file:");
    io::stdin().read_line(&mut places_file)?;

    println!("Enter the output directory path:");
    io::stdin().read_line(&mut output_dir)?;

    println!("Enter the number of entries to generate:");
    io::stdin().read_line(&mut num_entries)?;

    println!("Generate separate files for each entry? (yes/no):");
    io::stdin().read_line(&mut separate_files)?;

    // Trim inputs
    let sample_file = sample_file.trim();
    let names_file = names_file.trim();
    let places_file = places_file.trim();
    let output_dir = output_dir.trim();
    let num_entries: usize = num_entries.trim().parse().expect("Invalid number");
    let separate_files = separate_files.trim().eq_ignore_ascii_case("yes");

    // Read input files
    let sample_format = read_file(sample_file)?;
    let names = read_lines(names_file)?;
    let places = read_lines(places_file)?;

    // Parse sample format as JSON
    let sample_json: Value = serde_json::from_str(&sample_format).expect("Invalid JSON format in sample file");

    // Prepare output directory
    if separate_files {
        fs::create_dir_all(output_dir)?;
    }

    // Generate data
    let mut rng = thread_rng();
    let mut all_data = Vec::new();

    for i in 0..num_entries {
        let mut entry = sample_json.clone();
        if let Value::Object(map) = &mut entry {
            if let Some(Value::String(name)) = map.get_mut("name") {
                *name = names.choose(&mut rng).unwrap_or(&"Unknown".to_string()).clone();
            }
            if let Some(Value::String(place)) = map.get_mut("place") {
                *place = places.choose(&mut rng).unwrap_or(&"Unknown".to_string()).clone();
            }
            if let Some(Value::String(date)) = map.get_mut("date") {
                *date = chrono::Utc::now().format("%Y-%m-%d").to_string();
            }
        }

        if separate_files {
            let output_file = format!("{}/entry_{}.json", output_dir, i + 1);
            let mut file = File::create(output_file)?;
            file.write_all(entry.to_string().as_bytes())?;
        } else {
            all_data.push(entry);
        }
    }

    // Write all data to a single file if not separate
    if !separate_files {
        let output_file = format!("{}/all_entries.json", output_dir);
        let mut file = File::create(output_file)?;
        file.write_all(serde_json::to_string_pretty(&all_data)?.as_bytes())?;
    }

    println!("Data generation complete!");
    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}
