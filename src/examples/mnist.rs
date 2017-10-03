use super::super::model::ModelSpec;
use super::super::predict::PredictRequest;
use super::super::tensor::TensorProto;
use super::super::tensor_shape::{TensorShapeProto, TensorShapeProto_Dim};
use super::super::types::DataType;
/// Functions for downloading and reading MNIST data.
///
/// https://github.com/tensorflow/serving/blob/master/tensorflow_serving/example/mnist_client.py

use bytes::{BigEndian, BufMut, ByteOrder, LittleEndian};
use errors::*;
use flate2::read::GzDecoder;
use ndarray::prelude::*;
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

/// Extract the images into a vector of ndarray Vec<y, x, depth>.
pub fn extract_images<P: AsRef<Path>>(filename: &P) -> Result<Array4<u8>> {
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
    let array = arr1(&buf).into_shape((num_images, rows, cols, 1))?;
    Ok(array)
}

/// Extract the labels into a 1D uint8 numpy array [index].
pub fn extract_labels<P: AsRef<Path>>(filename: P) -> Result<Array1<u8>> {
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

    Ok(arr1(&buf))
}


/// Data structure that holds images and corresponding labels
fn to_proto<'a>(image: ArrayView2<'a, f32>) -> TensorProto {
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
    let tensor_content = image.into_slice().unwrap();
    image_proto.set_tensor_content(encode(tensor_content));

    image_proto
}


pub struct DataSet {
    images: Array2<f32>,
    labels: Array1<u8>,
}

impl DataSet {
    pub fn iter<'a>(&'a self) -> DataSetIter<'a> {
        let num = self.images.shape()[0];
        DataSetIter {
            data: &self,
            index: 0,
            num_examples: num,
            epochs_completed: 0,
        }
    }
}

pub struct DataSetIter<'a> {
    data: &'a DataSet,
    index: usize,
    num_examples: usize,
    epochs_completed: usize,
}

impl<'a> Iterator for DataSetIter<'a> {
    type Item = (ArrayView2<'a, f32>, ArrayView1<'a, u8>);
    fn next(&mut self) -> Option<Self::Item> {
        let range = (self.index as isize)..((self.index + 1) as isize);
        let images = self.data.images.slice(s![range, .., ]);

        let range = (self.index as isize)..((self.index + 1) as isize);
        let labels = self.data.labels.slice(s![range]);

        self.index = self.index + 1;
        if self.index == self.num_examples {
            self.epochs_completed += 1;
            self.index = 0;
            None
        } else {
            Some((images, labels))
        }
    }
}

impl DataSet {
    pub fn test<P: AsRef<Path>>(dir: P) -> Result<DataSet> {
        let local_file = maybe_download(TEST_IMAGES, &dir)?;
        let test_images = extract_images(&local_file)?;

        let local_file = maybe_download(TEST_LABELS, &dir)?;
        let test_labels = extract_labels(&local_file)?;

        // Convert shape from [num examples, rows, columns, depth]
        // to [num examples, rows*columns] (assuming depth == 1)
        let shape = {
            let shape = test_images.shape();
            assert_eq!(shape[3], 1);
            (shape[0], shape[1], shape[2])
        };

        let images = test_images.into_shape((shape.0, shape.1 * shape.2))?;
        let images = images.map(|&i| i as f32 * 1.0 / 255.0);
        Ok(DataSet {
            images: images,
            labels: test_labels,
        })
    }
}

pub fn predict_request<'a>(image: ArrayView2<'a, f32>) -> PredictRequest {
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
