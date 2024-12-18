use std::io;
use std::fs;
use std::path::Path;

pub fn gen_date(range_year_start: &u16, range_year_end: &u16) -> Vec<String> {
    let start_year = if range_year_start > range_year_end {
        range_year_end
    } else {
        range_year_start
    };
    let end_year = if range_year_start > range_year_end {
        range_year_start
    } else {
        range_year_end
    };

    let mut dates = Vec::new();

    for year in *start_year..=*end_year {
        for month in 1..=12 {
            let date = format!("{:04}-{:02}-01", year, month);
            dates.push(date);
        }
    }

    dates
}


pub fn gen_name(input: &str) -> io::Result<Vec<String>> {
    if Path::new(input).exists() {
        let content = fs::read_to_string(input)?;
        let names: Vec<String> = content
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                match parts.len() {
                    1 => format!("{}", parts[0]),
                    2 => format!("{} {}", parts[0], parts[1]),
                    _ => format!("{} {} {}", parts[0], parts[1], parts[2]),
                }
            })
            .collect();
        Ok(names)
    } else {
        let first_names = vec!["John", "Jane", "Alice"];
        let middle_names = vec!["Michael", "Mary", "Bob"];
        let last_names = vec!["Doe", "Smith", "Bob"];

        let generated_names: Vec<String> = first_names.iter()
            .zip(middle_names.iter())
            .zip(last_names.iter())
            .map(|((first, middle), last)| {
                format!("{} {} {}", first, middle, last)
            })
            .collect();
        
        Ok(generated_names)
    }
}