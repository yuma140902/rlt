use std::{fs::File, io::Read, path::Path};

use sha2::{Digest, Sha256};

pub mod cli;

pub fn get_hash(path: impl AsRef<Path>) -> anyhow::Result<String> {
    if !path.as_ref().is_file() {
        return Ok("".to_string());
    }

    let mut file = File::open(path.as_ref()).unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(buffer);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
