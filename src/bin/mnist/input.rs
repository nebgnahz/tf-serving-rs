use bytes::{BigEndian, ByteOrder};
use errors::*;
use flate2::read::GzDecoder;
use reqwest::{self, Url};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

const SOURCE_URL: &str = "http://yann.lecun.com/exdb/mnist/";
pub const TRAIN_IMAGES: &str = "train-images-idx3-ubyte.gz";
pub const TRAIN_LABELS: &str = "train-labels-idx1-ubyte.gz";
pub const TEST_IMAGES: &str = "t10k-images-idx3-ubyte.gz";
pub const TEST_LABELS: &str = "t10k-labels-idx1-ubyte.gz";
const _VALIDATION_SIZE: usize = 5000;

/// Download the data from Yann's website, unless it's already here.
pub fn maybe_download<P: AsRef<Path>>(filename: &str, dir: P) -> Result<PathBuf> {
    let mut path = PathBuf::new();
    path.push(dir);
    path.push(filename);

    if path.exists() {
        return Ok(path);
    }

    let url = Url::parse(SOURCE_URL)?.join(filename)?;
    let mut response = reqwest::get(url)?;
    if response.status().is_success() {
        let mut file = File::create(&path)?;
        let size = io::copy(&mut response, &mut file)?;
        println!("Successfully downloaded {:?} with {} bytes", path, size);
        Ok(path)
    } else {
        Err(ErrorKind::Server.into())
    }
}

/// Extract the images into a 4D uint8 numpy array [index, y, x, depth].
pub fn extract_images<P: AsRef<Path>>(filename: P) -> Result<()> {
    println!("Extracting {:?}", filename.as_ref());

    let file = File::open(filename)?;
    let mut gzip = GzDecoder::new(file)?;
    let mut magic = [0; 4];
    gzip.read_exact(&mut magic)?;
    let magic_num = BigEndian::read_u32(&magic);
    if magic_num != 2051 {
        println!("error magic {}", magic_num);
    }

    Ok(())
}
