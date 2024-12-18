use std::fs::{self, OpenOptions, File};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;


pub fn create_file(dir: Option<&str>, text: &str, fileName: &str)-> std::io::Result<()> {
    let path = Path::new(dir);
    if !path.exists(){
        fs::create_dir_all(path)? ;
    }
    let filePath = path.join(fileName);
    OpenOptions::new().create(true).write(true).open(filePath);
    Ok(())
}

pub fn append_2_file(filePath: &str, text: &str) -> std::io::Result<()>{
    let mut file = OpenOptions::new().append(true).open(filePath);
    writeln!(file, "{}", text);
    Ok(())
}

pub fn replace_text(filePath: &str, old_text: &str, new_text: &str)-> std::io::Result<()>{
    let mut content = String::new();
    let mut file = OpenOptions::new().read(true).write(true).open(filePath)?;
    
    file.read_to_string(&mut content)?;

    let updated_content = content.replace(old_text, new_text);
    file.set_len(0)?;
    file.seek(SeekFrom::start(0))?;
    file.write_all(updated_content.as_bytes())?;

    Ok(())
}

pub fn add_2_line(filePath: &str, text: &str)-> std::io::Result<()>{
    let mut content = String::new();
    let file = OpenOptions::new().read(true).write(true).open(filePath);

    file.read_to_string(&mut content);

    let updated_content : String = content.lines().map(|line| format!("{} {} ", line, text))
                                                  .collect::<Vec<_>>()
                                                  .join("\n");

    file.set_len(0)?;
    file.seek(SeekFrom(0));
    file.write_all(updated_content.as_bytes())?;


    Ok(());

}

// Read Out File
pub fn get_contents(filePath: &str)-> std::io::Result<()>{
    let mut contents = String::new();
    let mut file = OpenOptions::new().read(true).open(filePath)?;
    file.read_to_string(&mut contents);
    Ok((contents))
}