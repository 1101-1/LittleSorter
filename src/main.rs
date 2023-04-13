use std::{
    env,
    error::Error,
    fs::{self, remove_file},
    io::{BufReader, BufWriter, Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let env_args: Vec<String> = env::args().collect();

    if env_args.len() != 2 {
        println!("Usage:");
        println!("little_sorter: <path/to/folder>");
        return Ok(());
    }
    let path_to_folder = env_args[1].trim();

    match sort_to_folders(path_to_folder) {
        Ok(()) => return Ok(()),
        Err(err) => return Err(err.into()),
    };
}

fn sort_to_folders(path_to_folder: &str) -> Result<(), Box<dyn Error>> {
    if let Ok(files) = fs::read_dir(path_to_folder) {
        for file in files {
            let file = file?;
            let file_path = file.path();
            let filename = file.file_name().to_str().unwrap().to_owned();
            let extension = match filename.split('.').last() {
                Some(ext) => ext,
                None => "unnamed",
            };

            let new_dir = format!("{}/{}", path_to_folder, extension);
            match fs::create_dir(&new_dir) {
                Ok(()) => (),
                Err(_err) => {
                    return Err("Folder with name of filename extension already exists".into())
                }
            };

            let prev_file = fs::File::open(&file_path)?;
            let mut prev_file_reader = BufReader::new(&prev_file);

            let new_file = fs::File::create(format!("{}/{}", &new_dir, filename))?;
            let mut new_file_writer = BufWriter::new(new_file);

            let mut buffer: Vec<u8> = Vec::new();

            prev_file_reader.read_to_end(&mut buffer)?;

            new_file_writer.write_all(&buffer)?;

            remove_file(&file_path).unwrap();
        }
        return Ok(());
    }

    Err("cannot find any files in folder".into())
}
