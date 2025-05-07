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
    let temp = compressed_blob.clone();
    if written {
        store_blob(&hashed_blob, compressed_blob)?;
    }
    println!("{}", hashed_blob);
    println!("{:?}", temp);
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_file() {
        let path = Path::new("test.txt");
        let mut file = File::create(&path).expect("Failed to create file");
        file.write_all(b"\0Hello, test!").expect("Write failed");

        let file = read_file(&path);
        assert!(file.is_ok());
        let file = file.unwrap();
        let s = if let Some(pos) = file.iter().position(|&b| b == 0) {
            String::from_utf8_lossy(&file[pos + 1..]).to_string()
        } else {
            String::new()
        };

        assert_eq!(s, "Hello, test!");
    }

    #[test]
    fn test_build_blob() {
        let s = String::from("Hello, test!");
        let content = s.into_bytes();

        let result = build_blob(&content);

        let mut expected = format!("blob {}\0", content.len()).into_bytes();
        expected.extend_from_slice(&content);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_hash_blob() {
        let blob: Vec<u8> = vec![
            98, 108, 111, 98, 32, 49, 51, 0, 0, 72, 101, 108, 108, 111, 44, 32, 116, 101, 115, 116,
            33,
        ];

        let hashed_blob = hash_blob(&blob);
        println!("test hash: {}", hashed_blob);

        assert_eq!(hashed_blob, "d50f93f7a00ded2bbd5706a897299cd490e12f26");
    }

    #[test]
    fn test_compress_blob() {
        let hashed_blob: Vec<u8> = vec![
            98, 108, 111, 98, 32, 49, 51, 0, 0, 72, 101, 108, 108, 111, 44, 32, 116, 101, 115, 116,
            33,
        ];

        let compressed_blob = compress_blob(&hashed_blob);

        assert!(compressed_blob.is_ok());
        assert_eq!(
            compressed_blob.unwrap(),
            [
                120, 156, 75, 202, 201, 79, 82, 48, 52, 102, 96, 240, 72, 205, 201, 201, 215, 81,
                40, 73, 45, 46, 81, 4, 0, 67, 62, 6, 69
            ]
        );
    }
}
