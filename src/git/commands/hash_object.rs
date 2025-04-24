use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{self, Write};
use std::io::{Error, Read};
use std::path::Path;

pub fn hash_object(args: Vec<String>) -> Result<(), Error> {
    let p = Path::new(".gut");
    if !p.exists() {
        println!("Not Initialized");
        return Ok(());
    }

    let mut written: bool = false;
    let mut file_index = 2;
    if args.len() > 3 {
        if args[2] == "-W" {
            written = true;
        }
        file_index = 3;
    }

    let content = read_file(Path::new(&args[file_index]))?;

    let blob = build_blob(&content);

    let hashed_blob = hash_blob(&blob);

    let compressed_blob = compress_blob(&blob)?;

    if written {
        store_blob(&hashed_blob, compressed_blob)?;
    }
    println!("{}", hashed_blob);

    Ok(())
}

fn store_blob(hashed_blob: &str, compressed_blob: Vec<u8>) -> io::Result<()> {
    let dir_name = &hashed_blob[0..2];
    let file_name = &hashed_blob[2..];

    let dir_path = format!(".gut/objects/{}", dir_name);

    fs::create_dir(&dir_path)?;

    let file_path = format!("{}/{}", dir_path, file_name);

    fs::write(file_path, compressed_blob)?;

    Ok(())
}
fn read_file(file_path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

fn build_blob(content: &Vec<u8>) -> Vec<u8> {
    let header = format!("blob {}\0", content.len());

    let mut blob = Vec::with_capacity(header.len() + content.len());
    blob.extend_from_slice(header.as_bytes());
    blob.extend_from_slice(content);

    blob
}

fn hash_blob(blob: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();

    hasher.update(&blob);

    format!("{:x}", hasher.finalize())
}
fn compress_blob(blob: &Vec<u8>) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    encoder.write_all(blob)?;

    encoder.finish()
}
