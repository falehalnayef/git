use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{self, Error, Read};
use std::path::{Path, PathBuf};

pub fn cat_file(args: Vec<String>) -> Result<(), Error> {
    let gut_path = Path::new(".gut");
    if !gut_path.exists() {
        println!("Not Initialized");
        return Ok(());
    }

    let mut p: bool = false;
    let mut index = 2;
    if args.len() > 3 {
        if args[2] == "-P" {
            p = true;
        }
        index = 3;
    }
    let hashed_header = &args[index];

    let blob_path = find_blob(&hashed_header).unwrap();

    let blob = read_file(blob_path)?;

    let decompressed_blob = decompress_blob(&blob)?;

    if let Some(pos) = decompressed_blob.iter().position(|&b| b == 0) {
        print!("{}", String::from_utf8_lossy(&decompressed_blob[pos + 1..]));
    }

    Ok(())
}

pub fn find_blob(hashed_header: &str) -> Option<PathBuf> {
    let dir_name = &hashed_header[0..2];
    let file_name = &hashed_header[2..];
    let file_path = format!(".gut/objects/{}/{}", dir_name, file_name);
    let path = Path::new(&file_path);

    if path.exists() {
        Some(path.to_path_buf())
    } else {
        println!("Blob not found");
        None
    }
}

fn read_file(file_path: PathBuf) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}
fn decompress_blob(blob: &Vec<u8>) -> io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(&blob[..]);

    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    Ok(decompressed_data)
}
