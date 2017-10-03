/// Functions for downloading and reading MNIST data.
///
/// https://github.com/tensorflow/serving/blob/master/tensorflow_serving/example/mnist_client.py
use super::super::model::ModelSpec;
use super::super::predict::PredictRequest;
use super::super::tensor::TensorProto;
use super::super::tensor_shape::{TensorShapeProto, TensorShapeProto_Dim};
use super::super::types::DataType;
use bytes::{BigEndian, BufMut, ByteOrder, LittleEndian};
use errors::*;
use flate2::read::GzDecoder;

use itertools::Itertools;
use reqwest::{self, Url};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

const SOURCE_URL: &str = "http://yann.lecun.com/exdb/mnist/";
const _TRAIN_IMAGES: &str = "train-images-idx3-ubyte.gz";
const _TRAIN_LABELS: &str = "train-labels-idx1-ubyte.gz";
const TEST_IMAGES: &str = "t10k-images-idx3-ubyte.gz";
const TEST_LABELS: &str = "t10k-labels-idx1-ubyte.gz";
const _VALIDATION_SIZE: usize = 5000;

/// Download the data from Yann's website, unless it's already here.
pub fn maybe_download<P: AsRef<Path>>(filename: &str, dir: &P) -> Result<PathBuf> {
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
        info!("Successfully downloaded {:?} with {} bytes", path, size);
        Ok(path)
    } else {
        Err(ErrorKind::Server.into())
    }
}


fn read_u32<R: Read>(r: &mut R) -> Result<u32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(BigEndian::read_u32(&buf))
}

fn normalize(i: u8) -> f32 {
    i as f32 * 1.0 / 255.0
}

/// Extract the images into a vector of ndarray Vec<y, x, depth>.
pub fn extract_images<P: AsRef<Path>>(filename: &P) -> Result<Vec<Vec<f32>>> {
    info!("Extracting images: {:?}", filename.as_ref());

    let file = File::open(filename)?;
    let mut gzip = GzDecoder::new(file)?;

    let magic = read_u32(&mut gzip)?;
    if magic != 2051 {
        return Err(ErrorKind::DataCorruption.into());
    }

    let num_images = read_u32(&mut gzip)? as usize;
    let rows = read_u32(&mut gzip)? as usize;
    let cols = read_u32(&mut gzip)? as usize;

    let mut buf = vec![0; rows * cols * num_images];
    gzip.read_exact(&mut buf[..])?;
    let array = buf.into_iter()
        .map(normalize)
        .chunks(rows * cols)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(array)
}

/// Extract the labels into a 1D uint8 numpy array [index].
pub fn extract_labels<P: AsRef<Path>>(filename: P) -> Result<Vec<u8>> {
    info!("Extracting labels: {:?}", filename.as_ref());

    let file = File::open(filename)?;
    let mut gzip = GzDecoder::new(file)?;
    let magic = read_u32(&mut gzip)?;
    if magic != 2049 {
        return Err(ErrorKind::DataCorruption.into());
    }

    let num_items = read_u32(&mut gzip)? as usize;
    let mut buf = vec![0; num_items];
    gzip.read_exact(&mut buf[..])?;

    Ok(buf)
}


/// Data structure that holds images and corresponding labels
fn to_proto<'a>(image: &'a [f32]) -> TensorProto {
    let mut image_proto = TensorProto::new();
    image_proto.set_dtype(DataType::DT_FLOAT);

    // Set shape
    let mut shape = TensorShapeProto::new();
    let mut dim = TensorShapeProto_Dim::new();
    dim.set_size(1);
    shape.dim.insert(0, dim);

    let mut dim = TensorShapeProto_Dim::new();
    dim.set_size(784);
    shape.dim.insert(1, dim);
    image_proto.set_tensor_shape(shape);

    // Set content
    image_proto.set_tensor_content(encode(&image));

    image_proto
}


pub struct DataSet {
    pairs: Vec<(Vec<f32>, u8)>,
}

impl DataSet {
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
}

impl ::std::ops::Index<usize> for DataSet {
    type Output = (Vec<f32>, u8);

    fn index(&self, i: usize) -> &Self::Output {
        &self.pairs[i]
    }
}

impl DataSet {
    pub fn test<P: AsRef<Path>>(dir: P) -> Result<DataSet> {
        let local_file = maybe_download(TEST_IMAGES, &dir)?;
        let test_images = extract_images(&local_file)?;

        let local_file = maybe_download(TEST_LABELS, &dir)?;
        let test_labels = extract_labels(&local_file)?;

        let pairs = test_images
            .into_iter()
            .zip(test_labels.into_iter())
            .collect();
        Ok(DataSet { pairs: pairs })
    }
}

pub fn predict_request<'a>(image: &'a [f32]) -> PredictRequest {
    let mut model_spec = ModelSpec::new();
    model_spec.set_name("mnist".into());
    model_spec.set_signature_name("predict_images".into());

    let mut request = PredictRequest::new();
    request.set_model_spec(model_spec);
    request.inputs.insert("images".into(), to_proto(image));

    request
}

fn encode<'a>(input: &'a [f32]) -> Vec<u8> {
    let mut buf = vec![];
    for i in input {
        buf.put_f32::<LittleEndian>(*i);
    }
    buf
}
