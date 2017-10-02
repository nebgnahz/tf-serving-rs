extern crate futures;
extern crate grpcio;
extern crate tf_serving;
extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate bytes;
#[macro_use]
extern crate ndarray;

use bytes::BufMut;
use futures::Future;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tf_serving as serving;
use tf_serving::model::ModelSpec;
use tf_serving::predict::PredictRequest;
use tf_serving::prediction_service_grpc::PredictionServiceClient;
use tf_serving::tensor::TensorProto;
use tf_serving::tensor_shape::{TensorShapeProto, TensorShapeProto_Dim};

mod errors {
    use reqwest;
    error_chain! {
        foreign_links {
            Reqwest(reqwest::Error);
            UrlParse(reqwest::UrlError);
            Io(::std::io::Error);
            ArrayShape(::ndarray::ShapeError);
            GRPC(::grpcio::Error);
        }
        errors {
            Server {
                description("failed to download from the server")
                display("failed to download from the server")
            }
            DataCorruption {
                description("failed parse downloaded data")
                display("failed parse downloaded data")
            }
        }
    }
}
mod input;
use errors::*;

fn main() {
    run().unwrap()
}

fn run() -> Result<()> {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("128.32.171.191:9000");
    let client = PredictionServiceClient::new(ch);

    let test_data = input::DataSet::test("/tmp")?;
    let err_counter = AtomicUsize::new(0);
    let total = 10;
    for (image, label) in test_data.iter().take(total) {
        let mut model_spec = ModelSpec::new();
        model_spec.set_name("mnist".into());
        model_spec.set_signature_name("predict_images".into());

        let mut request = PredictRequest::new();
        request.set_model_spec(model_spec);

        let mut image_proto = TensorProto::new();
        image_proto.set_dtype(serving::types::DataType::DT_FLOAT);

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

        request.inputs.insert("images".into(), image_proto);
        let predict = client.predict_async(request).and_then(|response| {
            let output = response.get_outputs();
            let output = output.get("scores").unwrap();
            let scores = &output.float_val;
            let max_idx = scores
                .iter()
                .enumerate()
                .max_by(|&(_, x), &(_, y)| x.partial_cmp(y).unwrap())
                .unwrap();

            if label[0] != max_idx.0 as u8 {
                err_counter.fetch_add(1, Ordering::SeqCst);
            }
            Ok(())
        });

        predict.wait().unwrap();
    }

    println!(
        "Error rate: {}%",
        err_counter.load(Ordering::SeqCst) as f32 / total as f32 * 100.0
    );

    Ok(())
}

fn encode<'a>(input: &'a [f32]) -> Vec<u8> {
    let mut buf = vec![];
    for i in input {
        buf.put_f32::<bytes::LittleEndian>(*i);
    }
    buf
}
