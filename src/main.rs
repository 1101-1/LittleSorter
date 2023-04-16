use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write}, path::{Path, PathBuf}
};
fn main() -> Result<(), Box<dyn Error>> {
    let env_args: Vec<String> = env::args().collect();

    if env_args.len() < 2 {
        println!("Usage:");
        println!("cargo run -- <path/to/folder> <flag(if is need)>");
        return Ok(());
    }
    
    let path_to_folder = env_args[1].trim();
    let path = Path::new(&path_to_folder);

    if !path.is_dir() {
        return Err(format!("{} is not a valid directory path", path_to_folder).into());
    }

    let flag = env_args.get(2).map(|s| s.trim());

    sort_folder(path, path, flag)
}

fn sort_folder(path: &Path, orig_path: &Path, flag: Option<&str>) -> Result<(), Box<dyn Error>> {
    let mut is_empty = true;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() {
                let result = sort_file_to_folder(&entry_path, orig_path, flag).unwrap();
                is_empty = result;
            } else if entry_path.is_dir() {
                sort_folder(&entry_path, orig_path, flag).unwrap();
                is_empty = false;
            }
        }
    } else {
        return Err("THE link on folder IS NOT A DIR".into())
    }
    
    if is_empty && flag == Some("-d") {
        fs::remove_dir(&path)?;
    }
    Ok(())
}

fn sort_file_to_folder(file_entry: &PathBuf, orig_path: &Path, flag: Option<&str>) -> Result<bool, Box<dyn Error>> {
    let file_path = file_entry;
    let filename = file_entry.file_name().unwrap().to_str().unwrap().to_owned();
    
    let mut new_filename = filename.clone();

    let mut extension = match filename.split('.').last() {
        Some(ext) => ext,
        None => {
            new_filename = format!("{}.unnamed", filename);
            "unnamed"
        },
    };

    if &extension.to_string() == &filename {
        extension = "unnamed"
    }

    let new_dir = orig_path.join(&extension);
    
    if !Path::new(&new_dir).exists() {
        match fs::create_dir(&new_dir) {
            Ok(()) => (),
            Err(err) => return Err(err.into())
        };
    }
    let dest_file_path = Path::new(&new_dir).join(&new_filename);

    if dest_file_path.exists() {
        return Ok(false);
    }
    
    let mut prev_file_reader = BufReader::new(fs::File::open(&file_path)?);

    let new_file = File::create(format!("{}/{}", &new_dir.to_str().unwrap(), &new_filename))?;
   
    let mut new_file_writer = BufWriter::new(new_file);

    let mut buffer: Vec<u8> = Vec::new();
    prev_file_reader.read_to_end(&mut buffer)?;

    new_file_writer.write_all(&buffer)?;

    if flag == Some("-d") {
        fs::remove_file(&file_path)?;
    }
    
    Ok(true)
}