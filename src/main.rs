use std::{
    env,
    error::Error,
    fs::{self, DirEntry},
    io::{BufReader, BufWriter, Read, Write, self}, path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let env_args: Vec<String> = env::args().collect();

    if env_args.len() < 2 {
        println!("Usage:");
        println!("little_sorter: <path/to/folder>");
        return Ok(());
    }
    let path_to_folder = env_args[1].trim();

    let flag = env_args.get(2).map(|s| s.trim());

    sort_to_folders(path_to_folder, path_to_folder, flag)
}

fn sort_to_folders(path_to_folder: &str, orig_path: &str, flag: Option<&str>) -> Result<(), Box<dyn Error>> {
    if let Some(flag) = flag {
        if flag == "-d" {
            sort_to_folders_recursive(Path::new(path_to_folder), Path::new(orig_path), Some(flag))
        } else {
            return Err("Invalid flag".into())
        }
    } else {
        sort_to_folders_recursive(Path::new(path_to_folder), Path::new(orig_path), None)
    }
    
}

fn sort_to_folders_recursive(path: &Path, orig_path: &Path, flag: Option<&str>) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                sort_to_folders_recursive(&entry_path, orig_path, flag)?;
            } else {
                sort_file_to_folder(&entry, orig_path, flag)?;
            }
        }
    } else {
        return Err("Invalid directory path".into());
    }
    
    Ok(())
}

fn sort_file_to_folder(file_entry: &DirEntry, orig_path: &Path, flag: Option<&str>) -> io::Result<()> {
    let file_path = file_entry.path();
    let filename = file_entry.file_name().to_str().unwrap().to_owned();
    let extension = match filename.split('.').last() {
        Some(ext) => ext,
        None => "unnamed",
    };

    let new_dir = format!("{}/{}", orig_path.to_str().unwrap(), extension);

    if !Path::new(&new_dir).exists() {
        fs::create_dir(&new_dir)?;
    }

    let mut prev_file_reader = BufReader::new(fs::File::open(&file_path)?);
    
    let new_file = fs::File::create(format!("{}/{}", &new_dir, filename))?;

    let mut new_file_writer = BufWriter::new(new_file);

    let mut buffer: Vec<u8> = Vec::new();

    prev_file_reader.read_to_end(&mut buffer)?;

    new_file_writer.write_all(&buffer)?;

    if flag == Some("-d") {
        fs::remove_file(&file_path)?;
    }
    
    Ok(())
}

// fn is_dir_empty(path: &Path) -> bool {
//     match fs::read_dir(path) {
//         Ok(mut dir) => dir.next().is_none(),
//         Err(_) => false, // Or handle the error in some other way
//     }
// }
// fn sort_to_folders(path_to_folder: &str, flag: Option<&str>) -> Result<(), Box<dyn Error>> {
//     if let Some(flag) = flag {
//         if flag == "-d" {
//             if let Ok(files) = fs::read_dir(path_to_folder) {
//                 for file in files {
//                     let file = file?;
//                     let file_path = file.path();
//                     let filename = file.file_name().to_str().unwrap().to_owned();
//                     let extension = match filename.split('.').last() {
//                         Some(ext) => ext,
//                         None => "unnamed",
//                     };

//                     let new_dir = format!("{}/{}", path_to_folder, extension);
//                     match fs::create_dir(&new_dir) {
//                         Ok(()) => (),
//                         Err(_err) => {
//                             return Err("Folder with name of filename extension already exists".into())
//                         }
//                     };

//                     let prev_file = fs::File::open(&file_path)?;
//                     let mut prev_file_reader = BufReader::new(&prev_file);

//                     let new_file = fs::File::create(format!("{}/{}", &new_dir, filename))?;
//                     let mut new_file_writer = BufWriter::new(new_file);

//                     let mut buffer: Vec<u8> = Vec::new();

//                     prev_file_reader.read_to_end(&mut buffer)?;

//                     new_file_writer.write_all(&buffer)?;

//                     remove_file(&file_path).unwrap();
//                 }
//                 return Ok(());
//             }
//         }
//     }
//     if let Ok(files) = fs::read_dir(path_to_folder) {
//         for file in files {
//             let file = file?;
//             let file_path = file.path();
//             let filename = file.file_name().to_str().unwrap().to_owned();
//             let extension = match filename.split('.').last() {
//                 Some(ext) => ext,
//                 None => "unnamed",
//             };

//             let new_dir = format!("{}/{}", path_to_folder, extension);
//             match fs::create_dir(&new_dir) {
//                 Ok(()) => (),
//                 Err(_err) => {
//                     return Err("Folder with name of filename extension already exists".into())
//                 }
//             };

//             let prev_file = fs::File::open(&file_path)?;
//             let mut prev_file_reader = BufReader::new(&prev_file);

//             let new_file = fs::File::create(format!("{}/{}", &new_dir, filename))?;
//             let mut new_file_writer = BufWriter::new(new_file);

//             let mut buffer: Vec<u8> = Vec::new();

//             prev_file_reader.read_to_end(&mut buffer)?;

//             new_file_writer.write_all(&buffer)?;

//             // remove_file(&file_path).unwrap();
//         }
//         return Ok(());
//     }

//     Err("cannot find any files in folder".into())
// }

// fn sort_to_folders_with_delete(path_to_folder: &str) -> Result<(), Box<dyn Error>> {
//     if let Ok(files) = fs::read_dir(path_to_folder) {
//         for file in files {
//             let file = file?;
//             let file_path = file.path();
//             let filename = file.file_name().to_str().unwrap().to_owned();
//             let extension = match filename.split('.').last() {
//                 Some(ext) => ext,
//                 None => "unnamed",
//             };

//             let new_dir = format!("{}/{}", path_to_folder, extension);
//             match fs::create_dir(&new_dir) {
//                 Ok(()) => (),
//                 Err(_err) => {
//                     return Err("Folder with name of filename extension already exists".into())
//                 }
//             };

//             let prev_file = fs::File::open(&file_path)?;
//             let mut prev_file_reader = BufReader::new(&prev_file);

//             let new_file = fs::File::create(format!("{}/{}", &new_dir, filename))?;
//             let mut new_file_writer = BufWriter::new(new_file);

//             let mut buffer: Vec<u8> = Vec::new();

//             prev_file_reader.read_to_end(&mut buffer)?;

//             new_file_writer.write_all(&buffer)?;
//         }
//         return Ok(());
//     }

//     Err("cannot find any files in folder".into())
// }
