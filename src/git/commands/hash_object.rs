use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::Write;
use std::io::{Error, Read};

pub fn hash_object(file_path: String) -> Result<(), Error> {
    let mut file = File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    let header = format!("blob {}\0", content.len());

    let mut blob = Vec::new();
    blob.extend_from_slice(header.as_bytes());
    blob.extend_from_slice(&content);

    let mut hasher = Sha1::new();

    hasher.update(&blob);

    let hashed_header = format!("{:x}", hasher.finalize());

    let dir_name = &hashed_header[0..2];
    let file_name = &hashed_header[2..];

    let dir_path = format!(".gut/objects/{}", dir_name);

    fs::create_dir(&dir_path)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    encoder.write_all(&blob)?;

    let compressed = encoder.finish()?;

    let file_path = format!("{}/{}", dir_path, file_name);

    fs::write(file_path, compressed)?;

    print!("{}", hashed_header);

    Ok(())
}
